//! Supervised execution of external commands.
//!
//! Commands run with a scrubbed environment, an explicit working directory,
//! output caps enforced while streaming and a deadline. Each command leads its
//! own process group so that the processes it spawns can be killed along with
//! it. The whole group is killed and the command reaped when the deadline
//! expires, when the output caps are exceeded or when the future running it is
//! dropped.

use std::{
    fmt, io,
    path::Path,
    process::{ExitStatus, Stdio},
    time::Duration,
};

use anyhow::{Context, Error, Result, bail};
#[cfg(unix)]
use rustix::process::{Pid, Signal, kill_process_group};
use tokio::{
    io::{AsyncRead, AsyncReadExt},
    process::{Child, Command},
    time::timeout,
};

/// Environment variables inherited by the commands executed (when set).
pub const INHERITED_ENV_VARS: [&str; 3] = ["HOME", "PATH", "TMPDIR"];

/// Specification of a command to run.
#[derive(Debug, Clone)]
pub struct CommandSpec<'a> {
    /// Command arguments.
    pub args: Vec<String>,
    /// Path to the binary.
    pub bin: &'a Path,
    /// Working directory.
    pub cwd: &'a Path,
    /// Maximum time the command can take.
    pub deadline: Duration,
    /// Extra environment variables (in addition to the inherited ones).
    pub env: Vec<(String, String)>,
    /// Maximum number of stderr bytes kept (the rest is discarded).
    pub stderr_cap: usize,
    /// Maximum number of stdout bytes accepted (exceeding it is an error).
    pub stdout_cap: usize,
}

/// Output of a command run.
#[derive(Debug, Clone)]
pub struct CommandOutput {
    /// Exit status of the command.
    pub status: ExitStatus,
    /// Stderr content (truncated to the cap requested).
    pub stderr: Vec<u8>,
    /// Stdout content.
    pub stdout: Vec<u8>,
}

impl CommandOutput {
    /// Return the stderr content as a trimmed lossy string.
    #[must_use]
    pub fn stderr_lossy(&self) -> String {
        String::from_utf8_lossy(&self.stderr).trim().to_string()
    }
}

/// Error returned when a command does not complete before its deadline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeadlineExceeded {
    /// Deadline the command was given.
    pub deadline: Duration,
}

impl fmt::Display for DeadlineExceeded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "command did not complete within {}s",
            self.deadline.as_secs()
        )
    }
}

impl std::error::Error for DeadlineExceeded {}

/// Behaviour when a stream exceeds its cap.
#[derive(Clone, Copy)]
enum Overflow {
    /// Fail the run as soon as the cap is exceeded.
    Fail,
    /// Keep the bytes up to the cap and discard the rest.
    Truncate,
}

/// Child process supervised as the leader of its own process group.
///
/// Killing the whole group, rather than the child alone, makes sure the
/// processes the tool spawns do not outlive the run. The group is killed when
/// the guard is dropped unless the child has already been reaped (its pid, and
/// thus the group id, may have been reused since).
struct ProcessGroup {
    /// Child process leading the group.
    child: Child,
}

impl ProcessGroup {
    /// Spawn the command provided as the leader of a new process group.
    fn spawn(cmd: &mut Command) -> io::Result<Self> {
        #[cfg(unix)]
        cmd.process_group(0);
        Ok(Self {
            child: cmd.spawn()?,
        })
    }

    /// Kill every process in the group and reap the child.
    async fn kill(&mut self) {
        self.kill_group();
        let _ = self.child.wait().await;
    }

    /// Wait for the child to exit, reaping it.
    async fn wait(&mut self) -> io::Result<ExitStatus> {
        self.child.wait().await
    }

    /// Send `SIGKILL` to the group (best effort). The child id is only
    /// available until it is reaped, which prevents signalling a reused pid.
    fn kill_group(&mut self) {
        #[cfg(unix)]
        if let Some(pgid) = self
            .child
            .id()
            .and_then(|id| i32::try_from(id).ok())
            .and_then(Pid::from_raw)
        {
            let _ = kill_process_group(pgid, Signal::KILL);
        }
        #[cfg(not(unix))]
        let _ = self.child.start_kill();
    }
}

impl Drop for ProcessGroup {
    fn drop(&mut self) {
        self.kill_group();
    }
}

/// Run the command described by the spec provided.
///
/// # Errors
///
/// Returns an error when the command cannot be spawned, when it does not
/// complete before the deadline, when its stdout exceeds the cap provided or
/// when its output cannot be read.
pub async fn run(spec: CommandSpec<'_>) -> Result<CommandOutput> {
    // Build the command with a scrubbed environment
    let mut cmd = Command::new(spec.bin);
    cmd.args(&spec.args)
        .current_dir(spec.cwd)
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    for var in INHERITED_ENV_VARS {
        if let Some(value) = std::env::var_os(var) {
            cmd.env(var, value);
        }
    }
    for (name, value) in &spec.env {
        cmd.env(name, value);
    }

    // Spawn the process in its own group taking ownership of its output streams
    let mut group = ProcessGroup::spawn(&mut cmd)
        .with_context(|| format!("error running {}", spec.bin.display()))?;
    let stdout = group.child.stdout.take();
    let stderr = group.child.stderr.take();

    // Collect both streams, failing fast when stdout exceeds its cap so the
    // group is killed right away instead of when stderr reaches EOF
    let collect = async {
        let (stdout, stderr) = tokio::try_join!(
            read_capped(stdout, spec.stdout_cap, Overflow::Fail),
            read_capped(stderr, spec.stderr_cap, Overflow::Truncate),
        )?;
        let status = group.wait().await.context("error waiting for command")?;
        Ok::<_, Error>(CommandOutput {
            status,
            stderr,
            stdout,
        })
    };

    // Enforce the deadline, killing the process group whenever the run fails
    match timeout(spec.deadline, collect).await {
        Ok(Ok(output)) => Ok(output),
        Ok(Err(err)) => {
            group.kill().await;
            Err(err)
        }
        Err(_) => {
            group.kill().await;
            Err(DeadlineExceeded {
                deadline: spec.deadline,
            }
            .into())
        }
    }
}

/// Check if the error provided was caused by a deadline being exceeded.
#[must_use]
pub fn is_deadline_exceeded(err: &Error) -> bool {
    err.chain()
        .any(|e| e.downcast_ref::<DeadlineExceeded>().is_some())
}

/// Read a stream up to the maximum number of bytes provided.
async fn read_capped<R: AsyncRead + Unpin>(
    reader: Option<R>,
    max_bytes: usize,
    overflow: Overflow,
) -> Result<Vec<u8>> {
    let Some(mut reader) = reader else {
        return Ok(Vec::new());
    };

    // Read the stream in chunks until EOF (the buffer is heap allocated to
    // keep the futures holding it small)
    let mut data = Vec::new();
    let mut buf = vec![0u8; 8192];
    loop {
        let n = reader
            .read(&mut buf)
            .await
            .context("error reading command output")?;
        if n == 0 {
            return Ok(data);
        }

        // Apply the overflow behaviour requested once the cap is exceeded
        if data.len() + n > max_bytes {
            match overflow {
                Overflow::Fail => bail!("command output larger than {max_bytes} bytes"),
                Overflow::Truncate => {
                    let room = max_bytes.saturating_sub(data.len());
                    data.extend_from_slice(&buf[..room]);
                    // Keep draining so the child is not blocked on a full pipe
                    while reader.read(&mut buf).await? > 0 {}
                    return Ok(data);
                }
            }
        }
        data.extend_from_slice(&buf[..n]);
    }
}

#[cfg(test)]
#[cfg(unix)]
pub(crate) mod tests {
    use std::{fs, os::unix::fs::PermissionsExt, path::PathBuf, time::Instant};

    use super::*;

    #[tokio::test]
    async fn run_deadline_kills_process() {
        // Setup a script that records its pid and outlives the deadline
        let dir = tempfile::tempdir().unwrap();
        let bin = script(dir.path(), "tool", "echo $$ > pid; sleep 30");

        // Run the command
        let start = Instant::now();
        let err = run(spec(&bin, dir.path(), Duration::from_millis(300)))
            .await
            .unwrap_err();

        // Check the deadline error is reported promptly
        assert!(start.elapsed() < Duration::from_secs(10));
        assert!(is_deadline_exceeded(&err), "{err}");
        assert!(err.to_string().contains("did not complete within"), "{err}");
        assert!(!is_deadline_exceeded(&anyhow::anyhow!("other")));

        // Check the process is gone
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert!(!is_alive(&pid(dir.path(), "pid")), "process still alive");
    }

    #[tokio::test]
    async fn run_deadline_kills_process_group() {
        // Setup a script that spawns a child outliving the deadline and
        // records both pids
        let dir = tempfile::tempdir().unwrap();
        let bin = script(
            dir.path(),
            "tool",
            "sleep 30 & echo $! > child_pid; echo $$ > pid; wait",
        );

        // Run the command
        let err = run(spec(&bin, dir.path(), Duration::from_millis(300)))
            .await
            .unwrap_err();
        assert!(is_deadline_exceeded(&err), "{err}");

        // Check the whole process group is gone
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert!(!is_alive(&pid(dir.path(), "pid")), "process still alive");
        assert!(
            !is_alive(&pid(dir.path(), "child_pid")),
            "spawned process still alive"
        );
    }

    #[tokio::test]
    async fn run_dropped_kills_process_group() {
        // Setup a script that spawns a long running child and records both pids
        let dir = tempfile::tempdir().unwrap();
        let bin = script(
            dir.path(),
            "tool",
            "sleep 30 & echo $! > child_pid; echo $$ > pid; wait",
        );

        // Drop the run future as soon as the processes are running
        let child_pid_file = dir.path().join("child_pid");
        tokio::select! {
            result = run(spec(&bin, dir.path(), Duration::from_secs(30))) => {
                panic!("run completed unexpectedly: {result:?}");
            }
            () = async {
                while !child_pid_file.exists() {
                    tokio::time::sleep(Duration::from_millis(20)).await;
                }
            } => {}
        }

        // Check the whole process group is gone
        tokio::time::sleep(Duration::from_millis(200)).await;
        assert!(!is_alive(&pid(dir.path(), "pid")), "process still alive");
        assert!(
            !is_alive(&pid(dir.path(), "child_pid")),
            "spawned process still alive"
        );
    }

    #[tokio::test]
    async fn run_missing_binary_fails() {
        // Run a binary that does not exist
        let dir = tempfile::tempdir().unwrap();
        let bin = dir.path().join("missing");
        let err = run(spec(&bin, dir.path(), Duration::from_secs(1)))
            .await
            .unwrap_err()
            .to_string();

        // Check the spawn error is reported
        assert!(err.contains("error running"), "{err}");
    }

    #[tokio::test]
    async fn run_stderr_cap_truncates() {
        // Setup a script writing past the stderr cap
        let dir = tempfile::tempdir().unwrap();
        let bin = script(
            dir.path(),
            "tool",
            "head -c 4096 /dev/zero | tr '\\0' 'e' >&2; echo ok",
        );

        // Run the command
        let output = run(spec(&bin, dir.path(), Duration::from_secs(10)))
            .await
            .unwrap();

        // Check stderr is truncated and the run still succeeds
        assert!(output.status.success());
        assert_eq!(output.stderr.len(), 64);
        assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), "ok");
    }

    #[tokio::test]
    async fn run_stdout_cap_exceeded_fails() {
        // Setup a script writing past the stdout cap
        let dir = tempfile::tempdir().unwrap();
        let bin = script(dir.path(), "tool", "head -c 4096 /dev/zero | tr '\\0' 'a'");

        // Run the command
        let err = run(spec(&bin, dir.path(), Duration::from_secs(10)))
            .await
            .unwrap_err()
            .to_string();

        // Check the overflow is reported
        assert!(err.contains("larger than 1024 bytes"), "{err}");
    }

    #[tokio::test]
    async fn run_stdout_cap_exceeded_fails_fast_while_stderr_open() {
        // Setup a script that overflows stdout and then keeps running (and
        // its stderr open) well past the deadline
        let dir = tempfile::tempdir().unwrap();
        let bin = script(
            dir.path(),
            "tool",
            "head -c 4096 /dev/zero | tr '\\0' 'a'; exec sleep 30",
        );

        // Run the command
        let start = Instant::now();
        let err = run(spec(&bin, dir.path(), Duration::from_secs(10)))
            .await
            .unwrap_err();

        // Check the overflow is reported right away rather than a timeout
        assert!(
            start.elapsed() < Duration::from_secs(5),
            "{:?}",
            start.elapsed()
        );
        assert!(!is_deadline_exceeded(&err), "{err}");
        assert!(err.to_string().contains("larger than 1024 bytes"), "{err}");
    }

    #[tokio::test]
    async fn run_success_with_scrubbed_env_and_cwd() {
        // Setup a script reporting its environment, cwd and arguments
        let dir = tempfile::tempdir().unwrap();
        let bin = script(
            dir.path(),
            "tool",
            "env; pwd; echo \"$@\"; echo oops >&2; exit 3",
        );

        // Run the command
        let output = run(spec(&bin, dir.path(), Duration::from_secs(10)))
            .await
            .unwrap();

        // Check the exit status and stderr are captured
        assert_eq!(output.status.code(), Some(3));
        assert_eq!(output.stderr_lossy(), "oops");

        // Check only the inherited and extra variables reach the command
        let stdout = String::from_utf8(output.stdout).unwrap();
        let vars: Vec<&str> = stdout
            .lines()
            .filter_map(|l| l.split_once('=').map(|(k, _)| k))
            .filter(|k| !["PWD", "SHLVL", "OLDPWD", "_"].contains(k))
            .collect();
        for var in &vars {
            assert!(
                INHERITED_ENV_VARS.contains(var) || *var == "EXTRA_VAR",
                "leaked {var}"
            );
        }
        assert!(stdout.contains("EXTRA_VAR=extra"));

        // Check the working directory and arguments
        let cwd = fs::canonicalize(dir.path()).unwrap();
        assert!(stdout.contains(&cwd.to_string_lossy().to_string()));
        assert!(stdout.trim_end().ends_with("arg1 arg 2"));
    }

    // Helpers.

    /// Check if the process with the pid provided is still running (zombies
    /// count as gone).
    fn is_alive(pid: &str) -> bool {
        fs::read_to_string(format!("/proc/{pid}/status"))
            .is_ok_and(|s| !s.contains("zombie") && !s.contains("dead"))
    }

    /// Read a pid recorded by a script in the file provided.
    fn pid(dir: &Path, file: &str) -> String {
        fs::read_to_string(dir.join(file))
            .unwrap()
            .trim()
            .to_string()
    }

    /// Create an executable shell script with the body provided.
    pub(crate) fn script(dir: &Path, name: &str, body: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    /// Build a command spec with small output caps and sample arguments and
    /// environment.
    fn spec<'a>(bin: &'a Path, cwd: &'a Path, deadline: Duration) -> CommandSpec<'a> {
        CommandSpec {
            args: vec!["arg1".to_string(), "arg 2".to_string()],
            bin,
            cwd,
            deadline,
            env: vec![("EXTRA_VAR".to_string(), "extra".to_string())],
            stderr_cap: 64,
            stdout_cap: 1024,
        }
    }
}
