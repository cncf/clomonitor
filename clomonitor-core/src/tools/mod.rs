//! External tools used by some datasources (AFDocs and OpenSSF Scorecard).
//!
//! CLOMonitor does not embed these tools: it runs their command line binaries
//! and interprets the JSON reports they emit. This module is the single place
//! where each tool is described, so that a tool behaves the same no matter
//! where it runs:
//!
//! - [`Tool`] identifies a tool and holds its runtime properties: binary,
//!   deadline, accepted exit codes, credentials required and supported
//!   version.
//! - [`ToolRequest`] is the validated request to run a tool. Validation is
//!   shared by every entry point, so untrusted input (including urls that
//!   would reach local or private networks) is rejected before any process
//!   is spawned or any request is sent.
//! - [`LocalTool`] and [`run_local`] locate a binary, make sure its version is
//!   supported and run a request in isolation (per-run working directory,
//!   scrubbed environment, output caps and deadline) via [`process`].
//! - [`RunOutput`] is the outcome of a run, shared with the runner API.
//!
//! Two execution modes are built on these definitions (see
//! `linter::ToolMode`): the linter datasources can run the tools locally with
//! [`run_local`], or delegate the run to `clomonitor-runner`, the HTTP service
//! that runs them on behalf of the tracker, using [`runner::RunnerClient`].
//! The runner itself serves requests with [`run_local`], so both modes
//! produce identical results.

use std::{
    fmt,
    net::{IpAddr, Ipv4Addr},
    path::{Path, PathBuf},
    sync::LazyLock,
    time::{Duration, Instant},
};

use anyhow::{Context, Result, bail, format_err};
use regex::Regex;
use serde::{Deserialize, Serialize};
use which::which;

use self::process::{CommandOutput, CommandSpec};

pub mod process;
pub mod runner;

/// Maximum number of pages (and per-check links) AFDocs samples.
pub const AFDOCS_MAX_LINKS: u32 = 20;

/// AFDocs version (major.minor) the report contract has been validated against.
pub const AFDOCS_SUPPORTED_VERSION: &str = "0.20";

/// Extra time granted to clients over the tool deadline (covers transport,
/// queueing at the runner and bounded retries).
pub const CLIENT_TIMEOUT_MARGIN: Duration = Duration::from_secs(30);

/// Maximum size of a tool report (stdout).
pub const MAX_OUTPUT_BYTES: usize = 2 * 1024 * 1024;

/// Maximum size of the tool stderr kept for error reporting.
pub const MAX_STDERR_BYTES: usize = 64 * 1024;

/// Deadline for the tool version probes.
pub const PROBE_TIMEOUT: Duration = Duration::from_secs(15);

/// Scorecard checks CLOMonitor relies on.
pub const SCORECARD_CHECKS: [&str; 7] = [
    "Binary-Artifacts",
    "Code-Review",
    "Dangerous-Workflow",
    "Dependency-Update-Tool",
    "Maintained",
    "Signed-Releases",
    "Token-Permissions",
];

/// Content of the controlled AFDocs configuration file. It must contain the
/// empty mapping: a zero-byte file makes AFDocs throw.
const AFDOCS_CONFIG_CONTENT: &str = "{}\n";

/// Name of the controlled AFDocs configuration file. AFDocs auto-discovers
/// `agent-docs.config.yml` walking up from the cwd, so an explicit (empty)
/// config is always provided.
const AFDOCS_CONFIG_FILE: &str = "agent-docs.config.yml";

/// Maximum number of concurrent requests AFDocs makes (identical in local and
/// runner mode).
const AFDOCS_MAX_CONCURRENCY: &str = "3";

/// Maximum size of the Node.js heap (MiB) an AFDocs run can grow to before it
/// aborts. Keeps a run against a pathological site from exhausting the memory
/// of the process hosting it (and the other runs).
const AFDOCS_MAX_HEAP_MIB: u32 = 256;

/// Delay between AFDocs requests in milliseconds (identical in local and
/// runner mode).
const AFDOCS_REQUEST_DELAY_MS: &str = "200";

/// Soft memory limit for a scorecard run (Go `GOMEMLIMIT` value). The runtime
/// collects aggressively as the heap approaches it rather than aborting.
const SCORECARD_MEMORY_LIMIT: &str = "256MiB";

/// GitHub repository url accepted by scorecard requests.
static GITHUB_REPO_URL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^https://github\.com/[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+/?$")
        .expect("exprs in GITHUB_REPO_URL to be valid")
});

/// Scorecard check name (guards against injecting extra arguments).
static SCORECARD_CHECK_NAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[A-Za-z][A-Za-z0-9-]*$").expect("exprs in SCORECARD_CHECK_NAME to be valid")
});

/// Semantic version (with optional `v` prefix) in a tool version probe output.
static SEMVER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bv?(\d+\.\d+\.\d+)\b").expect("exprs in SEMVER to be valid"));

/// External tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tool {
    /// AFDocs, the agent-friendly documentation checker.
    Afdocs,
    /// OpenSSF Scorecard.
    Scorecard,
}

impl Tool {
    /// All tools supported.
    pub const ALL: [Tool; 2] = [Tool::Afdocs, Tool::Scorecard];

    /// Whether the exit code provided still denotes a completed run whose
    /// output can be used.
    #[must_use]
    pub fn accepts_exit_code(self, code: i32) -> bool {
        match self {
            // afdocs exits with 1 whenever any check fails (report on stdout)
            Self::Afdocs => matches!(code, 0 | 1),
            Self::Scorecard => code == 0,
        }
    }

    /// Name of the tool binary.
    #[must_use]
    pub fn binary(self) -> &'static str {
        self.id()
    }

    /// Overall deadline for a client requesting a run (covers retries).
    #[must_use]
    pub fn client_timeout(self) -> Duration {
        self.deadline() + CLIENT_TIMEOUT_MARGIN
    }

    /// Maximum time a tool run can take (request-wide deadline at the runner,
    /// including queue time).
    #[must_use]
    pub fn deadline(self) -> Duration {
        match self {
            Self::Afdocs => Duration::from_mins(4),
            Self::Scorecard => Duration::from_mins(8),
        }
    }

    /// Get the tool with the identifier provided.
    #[must_use]
    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|t| t.id() == id)
    }

    /// Tool identifier (used in the runner API paths and configuration).
    #[must_use]
    pub fn id(self) -> &'static str {
        match self {
            Self::Afdocs => "afdocs",
            Self::Scorecard => "scorecard",
        }
    }

    /// Locate the tool binary in PATH.
    ///
    /// # Errors
    ///
    /// Returns an error when the binary cannot be found.
    pub fn locate(self) -> Result<PathBuf> {
        which(self.binary()).map_err(|_| {
            format_err!(
                "{} not found in PATH ({})",
                self.binary(),
                match self {
                    Self::Afdocs => "https://afdocs.dev",
                    Self::Scorecard => "https://github.com/ossf/scorecard#installation",
                }
            )
        })
    }

    /// Whether the tool needs a GitHub token to run.
    #[must_use]
    pub fn requires_github_token(self) -> bool {
        matches!(self, Self::Scorecard)
    }

    /// Environment variables capping the memory a run of the tool can use, so
    /// a run exhausting it fails on its own instead of taking down the process
    /// hosting it along with any other run in progress.
    fn env(self) -> Vec<(String, String)> {
        match self {
            Self::Afdocs => vec![(
                "NODE_OPTIONS".to_string(),
                format!("--max-old-space-size={AFDOCS_MAX_HEAP_MIB}"),
            )],
            Self::Scorecard => vec![("GOMEMLIMIT".to_string(), SCORECARD_MEMORY_LIMIT.to_string())],
        }
    }

    /// Extract and validate the tool version from the probe output (stdout
    /// and stderr combined, as tools differ in the stream they report it on).
    fn parse_version(self, output: &str) -> Result<String> {
        // Extract the version from the output
        let version = SEMVER
            .captures(output)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| {
                format_err!(
                    "unable to detect {} version from {:?}",
                    self.binary(),
                    output.trim()
                )
            })?;

        // Make sure the version is supported
        if self == Self::Afdocs && !version.starts_with(&format!("{AFDOCS_SUPPORTED_VERSION}.")) {
            bail!("unsupported afdocs version {version} (expected {AFDOCS_SUPPORTED_VERSION}.x)");
        }
        Ok(version)
    }

    /// Arguments used to probe the tool version.
    fn version_args(self) -> Vec<String> {
        match self {
            Self::Afdocs => vec!["--version".to_string()],
            Self::Scorecard => vec!["version".to_string()],
        }
    }
}

impl fmt::Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id())
    }
}

/// Request to run AFDocs against a website.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AfdocsRequest {
    /// Website url to analyse.
    pub url: String,

    /// Maximum number of pages to sample (capped to [`AFDOCS_MAX_LINKS`]).
    #[serde(default = "default_max_links")]
    pub max_links: u32,
}

/// Request to run OpenSSF Scorecard against a GitHub repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScorecardRequest {
    /// Scorecard checks to run.
    pub checks: Vec<String>,
    /// GitHub repository url.
    pub repo_url: String,
}

/// Request to run a tool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolRequest {
    /// Run AFDocs.
    Afdocs(AfdocsRequest),
    /// Run OpenSSF Scorecard.
    Scorecard(ScorecardRequest),
}

impl ToolRequest {
    /// Build a request for the tool provided from its JSON representation.
    ///
    /// # Errors
    ///
    /// Returns an error when the JSON does not match the tool request shape
    /// or when the request is not valid.
    pub fn from_json(tool: Tool, value: serde_json::Value) -> Result<Self> {
        // Deserialize the request for the tool provided
        let request = match tool {
            Tool::Afdocs => Self::Afdocs(serde_json::from_value(value)?),
            Tool::Scorecard => Self::Scorecard(serde_json::from_value(value)?),
        };

        // Validate it before handing it over
        request.validate()?;
        Ok(request)
    }

    /// JSON representation of the request.
    #[must_use]
    pub fn to_json(&self) -> serde_json::Value {
        // Requests only contain strings and integers, so serialization cannot fail
        match self {
            Self::Afdocs(r) => serde_json::to_value(r).unwrap_or_default(),
            Self::Scorecard(r) => serde_json::to_value(r).unwrap_or_default(),
        }
    }

    /// Tool the request is for.
    #[must_use]
    pub fn tool(&self) -> Tool {
        match self {
            Self::Afdocs(_) => Tool::Afdocs,
            Self::Scorecard(_) => Tool::Scorecard,
        }
    }

    /// Validate the request.
    ///
    /// # Errors
    ///
    /// Returns an error describing the first invalid field found.
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Afdocs(r) => {
                // Check the url is a well formed http(s) url without credentials
                let url = reqwest::Url::parse(&r.url)
                    .map_err(|err| format_err!("invalid url {:?}: {err}", r.url))?;
                if !matches!(url.scheme(), "http" | "https") {
                    bail!("invalid url {:?}: only http(s) urls are supported", r.url);
                }
                if url.host_str().is_none() {
                    bail!("invalid url {:?}: host missing", r.url);
                }
                if !url.username().is_empty() || url.password().is_some() {
                    bail!("invalid url {:?}: credentials not allowed", r.url);
                }

                // Check the url does not target local or private networks
                if is_restricted_host(&url) {
                    bail!(
                        "invalid url {:?}: local and private network hosts are not allowed",
                        r.url
                    );
                }

                // Check the sampling size is within bounds
                if r.max_links == 0 || r.max_links > AFDOCS_MAX_LINKS {
                    bail!(
                        "invalid max_links {}: must be 1-{AFDOCS_MAX_LINKS}",
                        r.max_links
                    );
                }
            }
            Self::Scorecard(r) => {
                // Check the repository url is a GitHub repository
                if !GITHUB_REPO_URL.is_match(&r.repo_url) {
                    bail!(
                        "invalid repo_url {:?}: expected https://github.com/<org>/<repo>",
                        r.repo_url
                    );
                }

                // Check the checks requested are well formed names
                if r.checks.is_empty() {
                    bail!("at least one scorecard check must be provided");
                }
                if let Some(check) = r.checks.iter().find(|c| !SCORECARD_CHECK_NAME.is_match(c)) {
                    bail!("invalid scorecard check name {check:?}");
                }
            }
        }
        Ok(())
    }

    /// Build the command arguments for the request. The run directory is used
    /// for any auxiliary file the tool needs.
    fn args(&self, run_dir: &Path) -> Result<Vec<String>> {
        Ok(match self {
            Self::Afdocs(r) => {
                // Write the controlled config file in the run directory
                let config_path = run_dir.join(AFDOCS_CONFIG_FILE);
                std::fs::write(&config_path, AFDOCS_CONFIG_CONTENT)
                    .context("error writing afdocs config file")?;

                // Build the arguments
                vec![
                    "check".to_string(),
                    r.url.clone(),
                    "--format".to_string(),
                    "json".to_string(),
                    "--score".to_string(),
                    "--sampling".to_string(),
                    "deterministic".to_string(),
                    "--max-links".to_string(),
                    r.max_links.to_string(),
                    "--max-concurrency".to_string(),
                    AFDOCS_MAX_CONCURRENCY.to_string(),
                    "--request-delay".to_string(),
                    AFDOCS_REQUEST_DELAY_MS.to_string(),
                    "--config".to_string(),
                    config_path.to_string_lossy().to_string(),
                ]
            }
            Self::Scorecard(r) => vec![
                format!("--repo={}", r.repo_url),
                "--format=json".to_string(),
                "--show-details".to_string(),
                format!("--checks={}", r.checks.join(",")),
            ],
        })
    }
}

/// Output of a tool run (same shape as the runner API response).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunOutput {
    /// Time the run took.
    pub duration_ms: u64,
    /// Tool report (JSON emitted by the tool on stdout).
    pub output: serde_json::Value,
    /// Tool that was run.
    pub tool: Tool,
    /// Version of the tool.
    pub tool_version: String,
}

/// Tool binary available locally whose version has been probed and is
/// supported.
#[derive(Debug, Clone)]
pub struct LocalTool {
    /// Path to the tool binary.
    pub bin: PathBuf,
    /// Tool the binary is for.
    pub tool: Tool,
    /// Version reported by the binary.
    pub version: String,
}

impl LocalTool {
    /// Locate the tool binary in PATH and probe its version.
    ///
    /// # Errors
    ///
    /// Returns an error when the binary cannot be found, cannot be run, does
    /// not report a version or reports an unsupported one.
    pub async fn locate(tool: Tool) -> Result<Self> {
        let bin = tool.locate()?;
        Self::probe(tool, bin).await
    }

    /// Probe the version of the tool binary provided, making sure it is
    /// supported. Runs with the same isolation as the tool itself.
    ///
    /// # Errors
    ///
    /// Returns an error when the binary cannot be run, does not report a
    /// version or reports an unsupported one.
    pub async fn probe(tool: Tool, bin: PathBuf) -> Result<Self> {
        // Isolated working directory for the probe (removed on drop)
        let run_dir = tempfile::Builder::new()
            .prefix(&format!("clomonitor-{}-", tool.id()))
            .tempdir()
            .context("error creating working directory")?;

        // Probe the version
        let version = probe_version(tool, &bin, run_dir.path(), PROBE_TIMEOUT).await?;
        Ok(Self { bin, tool, version })
    }
}

/// Run the tool request provided locally using the probed tool given.
///
/// The tool runs in a temporary working directory (also used as its temporary
/// files directory) with a scrubbed environment (the tool memory caps are set
/// and the GitHub token, when provided, is only injected for tools that
/// require it), output caps and the deadline provided.
///
/// # Errors
///
/// Returns an error when the request is not for the tool given, when the tool
/// cannot be run, exceeds the deadline or the output caps, exits abnormally
/// or does not emit valid JSON.
pub async fn run_local(
    request: &ToolRequest,
    tool: &LocalTool,
    github_token: Option<&str>,
    deadline: Duration,
) -> Result<RunOutput> {
    // Validate the request and check it matches the tool
    if request.tool() != tool.tool {
        bail!(
            "{} request cannot be run with {}",
            request.tool(),
            tool.tool
        );
    }
    request.validate()?;
    let start = Instant::now();

    // Per-run working directory: keeps the tool away from any config file in
    // the repository or its ancestors, holds auxiliary files and, as TMPDIR,
    // any temporary file the tool creates so they are all removed on drop
    let run_dir = tempfile::Builder::new()
        .prefix(&format!("clomonitor-{}-", tool.tool.id()))
        .tempdir()
        .context("error creating working directory")?;

    // Prepare the environment: per-run TMPDIR, the tool memory caps and the
    // token only when required
    let mut env = vec![("TMPDIR".to_string(), run_dir.path().display().to_string())];
    env.extend(tool.tool.env());
    if tool.tool.requires_github_token() {
        let token =
            github_token.ok_or_else(|| format_err!("{} requires a GitHub token", tool.tool))?;
        env.push(("GITHUB_TOKEN".to_string(), token.to_string()));
    }

    // Run the tool
    let output = process::run(CommandSpec {
        args: request.args(run_dir.path())?,
        bin: &tool.bin,
        cwd: run_dir.path(),
        deadline: deadline.saturating_sub(start.elapsed()),
        env,
        stderr_cap: MAX_STDERR_BYTES,
        stdout_cap: MAX_OUTPUT_BYTES,
    })
    .await
    .with_context(|| format!("error running {}", tool.tool))?;

    // Check the exit status and parse the report
    let output = interpret_output(tool.tool, &output)?;

    // Prepare the run output
    Ok(RunOutput {
        duration_ms: u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX),
        output,
        tool: tool.tool,
        tool_version: tool.version.clone(),
    })
}

/// Default maximum number of pages AFDocs samples.
fn default_max_links() -> u32 {
    AFDOCS_MAX_LINKS
}

/// Check the tool exit status and parse its stdout as JSON.
fn interpret_output(tool: Tool, output: &CommandOutput) -> Result<serde_json::Value> {
    // Check the exit status denotes a completed run
    let stderr = output.stderr_lossy();
    match output.status.code() {
        Some(code) if tool.accepts_exit_code(code) => {}
        Some(code) => bail!("{tool} exited with code {code}: {stderr}"),
        None => bail!("{tool} terminated abnormally: {stderr}"),
    }

    // Parse the report
    serde_json::from_slice(&output.stdout)
        .with_context(|| format!("error parsing {tool} output as JSON (stderr: {stderr})"))
}

/// Check if the url host must not be targeted by a tool: localhost names and
/// loopback, link-local, private, shared and unspecified IP literals. This is
/// a defence in depth measure independent of any network policy in place.
fn is_restricted_host(url: &reqwest::Url) -> bool {
    match url.domain() {
        Some(domain) => {
            let domain = domain.trim_end_matches('.').to_ascii_lowercase();
            domain == "localhost" || domain.ends_with(".localhost")
        }
        None => url
            .host_str()
            .and_then(|host| host.trim_matches(['[', ']']).parse::<IpAddr>().ok())
            .is_some_and(is_restricted_ip),
    }
}

/// Check if the IP address provided belongs to a local or private range.
fn is_restricted_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => is_restricted_ipv4(ip),
        IpAddr::V6(ip) => match ip.to_ipv4_mapped() {
            Some(ip) => is_restricted_ipv4(ip),
            None => {
                ip.is_loopback()
                    || ip.is_unique_local()
                    || ip.is_unicast_link_local()
                    || ip.is_unspecified()
            }
        },
    }
}

/// Check if the IPv4 address provided belongs to a local or private range.
fn is_restricted_ipv4(ip: Ipv4Addr) -> bool {
    // Shared address space (100.64.0.0/10) has no stable std predicate
    let octets = ip.octets();
    let is_shared = octets[0] == 100 && (octets[1] & 0b1100_0000) == 64;
    ip.is_loopback()
        || ip.is_private()
        || ip.is_link_local()
        || ip.is_broadcast()
        || ip.is_unspecified()
        || is_shared
}

/// Probe the version of the tool binary provided, making sure it is
/// supported.
async fn probe_version(tool: Tool, bin: &Path, cwd: &Path, deadline: Duration) -> Result<String> {
    // Run the version command
    let output = process::run(CommandSpec {
        args: tool.version_args(),
        bin,
        cwd,
        deadline,
        env: vec![("TMPDIR".to_string(), cwd.display().to_string())],
        stderr_cap: MAX_STDERR_BYTES,
        stdout_cap: MAX_STDERR_BYTES,
    })
    .await
    .with_context(|| format!("error probing {tool} version"))?;
    if !output.status.success() {
        bail!("error probing {tool} version: {}", output.stderr_lossy());
    }

    // Tools differ in the stream they report the version on (scorecard uses
    // stderr), so both are inspected
    let reported = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        output.stderr_lossy()
    );
    tool.parse_version(&reported)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn request_args() {
        // Check the afdocs arguments and the controlled config written
        let dir = tempfile::tempdir().unwrap();
        let args = ToolRequest::Afdocs(AfdocsRequest {
            url: "https://docs.example.org/".to_string(),
            max_links: 20,
        })
        .args(dir.path())
        .unwrap();
        assert_eq!(
            &args[..13],
            &[
                "check",
                "https://docs.example.org/",
                "--format",
                "json",
                "--score",
                "--sampling",
                "deterministic",
                "--max-links",
                "20",
                "--max-concurrency",
                "3",
                "--request-delay",
                "200",
            ]
        );
        assert_eq!(args[13], "--config");
        assert_eq!(
            std::fs::read_to_string(&args[14]).unwrap(),
            AFDOCS_CONFIG_CONTENT
        );

        // Check the scorecard arguments
        let args = ToolRequest::Scorecard(ScorecardRequest {
            checks: vec!["Code-Review".to_string(), "Maintained".to_string()],
            repo_url: "https://github.com/org/repo".to_string(),
        })
        .args(dir.path())
        .unwrap();
        assert_eq!(
            args,
            vec![
                "--repo=https://github.com/org/repo",
                "--format=json",
                "--show-details",
                "--checks=Code-Review,Maintained",
            ]
        );
    }

    #[test]
    fn request_validation_afdocs() {
        // Setup validation helper
        let valid = |url: &str, max_links: u32| {
            ToolRequest::Afdocs(AfdocsRequest {
                url: url.to_string(),
                max_links,
            })
            .validate()
        };

        // Check url scheme, shape and sampling bounds
        assert!(valid("https://docs.example.org/", 20).is_ok());
        assert!(valid("http://docs.example.org", 1).is_ok());
        assert!(valid("https://93.184.216.34/", 20).is_ok());
        assert!(valid("https://[2606:2800:220:1:248:1893:25c8:1946]/", 20).is_ok());
        assert!(valid("ftp://docs.example.org/", 20).is_err());
        assert!(valid("https://user:pw@docs.example.org/", 20).is_err());
        assert!(valid("not a url", 20).is_err());
        assert!(valid("https://docs.example.org/", 0).is_err());
        assert!(valid("https://docs.example.org/", 21).is_err());

        // Check local and private network hosts are rejected
        for url in [
            "http://localhost:8080/",
            "http://docs.localhost/",
            "http://LOCALHOST./",
            "http://127.0.0.1:8080/run/afdocs",
            "http://127.1/",
            "http://0.0.0.0/",
            "http://10.1.2.3/",
            "http://172.16.0.1/",
            "http://192.168.1.1/",
            "http://169.254.169.254/latest/meta-data/",
            "http://100.64.0.1/",
            "http://255.255.255.255/",
            "http://[::1]/",
            "http://[::]/",
            "http://[::ffff:127.0.0.1]/",
            "http://[::ffff:10.0.0.1]/",
            "http://[fd00::1]/",
            "http://[fe80::1]/",
        ] {
            let err = valid(url, 20).unwrap_err().to_string();
            assert!(err.contains("not allowed"), "{url}: {err}");
        }

        // Check the JSON round trip applies the default max_links
        let request =
            ToolRequest::from_json(Tool::Afdocs, json!({"url": "https://docs.example.org/"}))
                .unwrap();
        assert_eq!(
            request,
            ToolRequest::Afdocs(AfdocsRequest {
                url: "https://docs.example.org/".to_string(),
                max_links: AFDOCS_MAX_LINKS,
            })
        );
        assert_eq!(request.to_json()["max_links"], json!(AFDOCS_MAX_LINKS));
        assert!(ToolRequest::from_json(Tool::Afdocs, json!({"repo_url": "x"})).is_err());
    }

    #[test]
    fn request_validation_scorecard() {
        // Setup validation helper
        let valid = |repo_url: &str, checks: &[&str]| {
            ToolRequest::Scorecard(ScorecardRequest {
                checks: checks.iter().map(ToString::to_string).collect(),
                repo_url: repo_url.to_string(),
            })
            .validate()
        };

        // Check repository url and check names
        assert!(valid("https://github.com/org/repo", &SCORECARD_CHECKS).is_ok());
        assert!(valid("https://github.com/org/repo/", &["Code-Review"]).is_ok());
        assert!(valid("https://gitlab.com/org/repo", &["Code-Review"]).is_err());
        assert!(valid("https://github.com/org", &["Code-Review"]).is_err());
        assert!(valid("https://github.com/org/repo", &[]).is_err());
        assert!(valid("https://github.com/org/repo", &["Code Review"]).is_err());
        assert!(valid("https://github.com/org/repo", &["--repo=x"]).is_err());

        // Check the JSON shape is enforced
        assert!(
            ToolRequest::from_json(
                Tool::Scorecard,
                json!({"repo_url": "https://github.com/o/r"})
            )
            .is_err()
        );
    }

    #[test]
    fn tool_ids_and_properties() {
        // Check identifiers round trip through lookup, display and serde
        for tool in Tool::ALL {
            assert_eq!(Tool::from_id(tool.id()), Some(tool));
            assert_eq!(tool.to_string(), tool.id());
            assert_eq!(serde_json::to_value(tool).unwrap(), json!(tool.id()));
        }
        assert_eq!(Tool::from_id("unknown"), None);

        // Check per-tool run properties
        assert!(Tool::Scorecard.requires_github_token());
        assert!(!Tool::Afdocs.requires_github_token());
        assert!(Tool::Afdocs.accepts_exit_code(1));
        assert!(!Tool::Scorecard.accepts_exit_code(1));
        assert!(Tool::Afdocs.client_timeout() > Tool::Afdocs.deadline());

        // Check the memory caps set for each tool
        assert_eq!(
            Tool::Afdocs.env(),
            vec![(
                "NODE_OPTIONS".to_string(),
                "--max-old-space-size=256".to_string()
            )]
        );
        assert_eq!(
            Tool::Scorecard.env(),
            vec![("GOMEMLIMIT".to_string(), "256MiB".to_string())]
        );
    }

    #[test]
    fn tool_parse_version() {
        // Check afdocs versions, including the supported range
        assert_eq!(Tool::Afdocs.parse_version("0.20.0\n").unwrap(), "0.20.0");
        assert_eq!(
            Tool::Afdocs.parse_version("afdocs v0.20.3").unwrap(),
            "0.20.3"
        );
        assert!(
            Tool::Afdocs
                .parse_version("0.21.0")
                .unwrap_err()
                .to_string()
                .contains("unsupported")
        );
        assert!(Tool::Afdocs.parse_version("garbage").is_err());

        // Check scorecard versions
        assert_eq!(
            Tool::Scorecard
                .parse_version("scorecard version:\nGitVersion:    v4.13.0\nGitCommit: abc\n")
                .unwrap(),
            "4.13.0"
        );
        assert!(Tool::Scorecard.parse_version("nothing here").is_err());
    }

    #[cfg(unix)]
    mod local {
        use std::fs;

        use crate::tools::process::tests::script;

        use super::*;

        /// Integration test against the pinned afdocs binary (skipped when
        /// absent). Proves the controlled `{}` config is accepted and that an
        /// ancestor `agent-docs.config.yml` is ignored.
        #[tokio::test]
        async fn pinned_afdocs_accepts_controlled_config() {
            let Ok(bin) = Tool::Afdocs.locate() else {
                eprintln!("afdocs binary not found, skipping integration test");
                return;
            };

            // Setup an ancestor config that would disable a check and the
            // controlled config
            let dir = tempfile::tempdir().unwrap();
            fs::write(
                dir.path().join(AFDOCS_CONFIG_FILE),
                "checks:\n  llms-txt-exists:\n    enabled: false\n",
            )
            .unwrap();
            fs::write(dir.path().join("controlled.yml"), AFDOCS_CONFIG_CONTENT).unwrap();

            // Run afdocs with an invalid target url so it fails fast (a
            // config error would be reported instead of the url one)
            let output = process::run(CommandSpec {
                args: vec![
                    "check".to_string(),
                    "not-a-url".to_string(),
                    "--format".to_string(),
                    "json".to_string(),
                    "--config".to_string(),
                    dir.path()
                        .join("controlled.yml")
                        .to_string_lossy()
                        .to_string(),
                ],
                bin: &bin,
                cwd: dir.path(),
                deadline: Duration::from_mins(1),
                env: vec![],
                stderr_cap: MAX_STDERR_BYTES,
                stdout_cap: MAX_OUTPUT_BYTES,
            })
            .await
            .unwrap();

            // Check no config error was reported
            assert!(!output.stderr_lossy().to_lowercase().contains("config"));
        }

        #[tokio::test]
        async fn probe_afdocs_success_with_isolation() {
            // Setup stub reporting a supported version on stdout
            let dir = tempfile::tempdir().unwrap();
            let bin = stub(dir.path(), "echo 0.20.0; exit 0", "exit 0");

            // Probe the tool
            let tool = LocalTool::probe(Tool::Afdocs, bin.clone()).await.unwrap();

            // Check the probed tool and the probe isolation
            assert_eq!(tool.bin, bin);
            assert_eq!(tool.tool, Tool::Afdocs);
            assert_eq!(tool.version, "0.20.0");
            let log = dir.path().join("log");
            let cwd = fs::read_to_string(log.join("probe.cwd")).unwrap();
            assert!(cwd.contains("clomonitor-afdocs-"), "{cwd}");
            assert_eq!(
                fs::read_to_string(log.join("probe.tmpdir")).unwrap().trim(),
                cwd_name(&cwd)
            );
            assert!(
                !fs::read_to_string(log.join("probe.env"))
                    .unwrap()
                    .contains("GITHUB_TOKEN")
            );
            assert!(!log.join("run.args").exists());
        }

        #[tokio::test]
        async fn probe_failure_rejected() {
            // Setup stub failing the version probe
            let dir = tempfile::tempdir().unwrap();
            let bin = stub(dir.path(), "echo nope >&2; exit 3", "exit 0");

            // Probe the tool
            let err = format!(
                "{:#}",
                LocalTool::probe(Tool::Afdocs, bin).await.unwrap_err()
            );

            // Check the probe error includes the tool stderr
            assert!(
                err.contains("error probing afdocs version") && err.contains("nope"),
                "{err}"
            );
        }

        #[tokio::test]
        async fn probe_scorecard_version_reported_on_stderr() {
            // Setup stub reporting the version on stderr
            let dir = tempfile::tempdir().unwrap();
            let bin = stub(
                dir.path(),
                "echo 'GitVersion: v4.13.0' >&2; exit 0",
                "exit 0",
            );

            // Probe the tool and check the version is detected
            let tool = LocalTool::probe(Tool::Scorecard, bin).await.unwrap();
            assert_eq!(tool.version, "4.13.0");
        }

        #[tokio::test]
        async fn probe_unsupported_afdocs_version_rejected() {
            // Setup stub reporting an unsupported version
            let dir = tempfile::tempdir().unwrap();
            let bin = stub(dir.path(), "echo 0.21.0; exit 0", "exit 0");

            // Probe the tool
            let err = LocalTool::probe(Tool::Afdocs, bin)
                .await
                .unwrap_err()
                .to_string();

            // Check the version is rejected
            assert!(err.contains("unsupported afdocs version 0.21.0"), "{err}");
        }

        #[tokio::test]
        async fn run_afdocs_success_with_isolation() {
            // Setup stub emitting a report and failing checks (exit code 1)
            let dir = tempfile::tempdir().unwrap();
            let tool = local(dir.path(), Tool::Afdocs, "echo '{\"ok\": true}'; exit 1").await;

            // Run the tool
            let output = run_local(
                &afdocs_request(),
                &tool,
                Some("secret"),
                Duration::from_secs(10),
            )
            .await
            .unwrap();

            // Check the output
            assert_eq!(output.tool, Tool::Afdocs);
            assert_eq!(output.tool_version, "0.20.0");
            assert_eq!(output.output, json!({"ok": true}));

            // Check the run isolation: no token, memory cap, controlled config,
            // per-run working directory also used as TMPDIR and expected
            // arguments
            let log = dir.path().join("log");
            let env = fs::read_to_string(log.join("run.env")).unwrap();
            assert!(!env.contains("GITHUB_TOKEN"), "run.env leaked the token");
            assert!(!env.contains("secret"), "run.env leaked the token");
            assert!(
                env.contains("NODE_OPTIONS=--max-old-space-size=256"),
                "{env}"
            );
            assert!(!env.contains("GOMEMLIMIT"), "{env}");
            assert_eq!(fs::read_to_string(log.join("run.config")).unwrap(), "{}\n");
            let cwd = fs::read_to_string(log.join("run.cwd")).unwrap();
            assert!(cwd.contains("clomonitor-afdocs-"), "{cwd}");
            assert_eq!(
                fs::read_to_string(log.join("run.tmpdir")).unwrap().trim(),
                cwd_name(&cwd)
            );
            assert!(!fs::exists(cwd.trim()).unwrap(), "run dir not removed");
            let args = fs::read_to_string(log.join("run.args")).unwrap();
            assert!(args.starts_with("check https://docs.example.org/ --format json --score"));
        }

        #[tokio::test]
        async fn run_deadline_enforced() {
            // Setup stub outliving the deadline
            let dir = tempfile::tempdir().unwrap();
            let tool = local(dir.path(), Tool::Afdocs, "sleep 30").await;

            // Run the tool with a short deadline
            let start = Instant::now();
            let err = format!(
                "{:#}",
                run_local(&afdocs_request(), &tool, None, Duration::from_millis(500))
                    .await
                    .unwrap_err()
            );

            // Check the deadline error is reported promptly
            assert!(start.elapsed() < Duration::from_secs(10));
            assert!(err.contains("did not complete within"), "{err}");
        }

        #[tokio::test]
        async fn run_does_not_probe_version_again() {
            // Setup probed tool and clear the probe trace
            let dir = tempfile::tempdir().unwrap();
            let tool = local(dir.path(), Tool::Afdocs, "echo '{}'; exit 0").await;
            let log = dir.path().join("log");
            fs::remove_file(log.join("probe.env")).unwrap();

            // Run the tool
            run_local(&afdocs_request(), &tool, None, Duration::from_secs(10))
                .await
                .unwrap();

            // Check no new probe happened
            assert!(!log.join("probe.env").exists(), "version probed again");
        }

        #[tokio::test]
        async fn run_invalid_json_rejected() {
            // Setup stub emitting a non JSON report
            let dir = tempfile::tempdir().unwrap();
            let tool = local(dir.path(), Tool::Afdocs, "echo 'not json'; exit 0").await;

            // Run the tool
            let err = format!(
                "{:#}",
                run_local(&afdocs_request(), &tool, None, Duration::from_secs(10))
                    .await
                    .unwrap_err()
            );

            // Check the parsing error is reported
            assert!(err.contains("error parsing afdocs output as JSON"), "{err}");
        }

        #[tokio::test]
        async fn run_request_for_another_tool_rejected() {
            // Setup afdocs tool
            let dir = tempfile::tempdir().unwrap();
            let tool = local(dir.path(), Tool::Afdocs, "echo '{}'; exit 0").await;

            // Run a scorecard request with it
            let err = run_local(
                &scorecard_request(),
                &tool,
                Some("t"),
                Duration::from_secs(10),
            )
            .await
            .unwrap_err()
            .to_string();

            // Check the request is rejected before running the tool
            assert!(
                err.contains("scorecard request cannot be run with afdocs"),
                "{err}"
            );
            assert!(!dir.path().join("log/run.args").exists());
        }

        #[tokio::test]
        async fn run_scorecard_success_with_token() {
            // Setup stub echoing the token it receives
            let dir = tempfile::tempdir().unwrap();
            let tool = local(
                dir.path(),
                Tool::Scorecard,
                "echo \"{\\\"token\\\": \\\"$GITHUB_TOKEN\\\"}\"; exit 0",
            )
            .await;

            // Run the tool
            let output = run_local(
                &scorecard_request(),
                &tool,
                Some("secret"),
                Duration::from_secs(10),
            )
            .await
            .unwrap();

            // Check the token reached the run only, the memory cap and the
            // arguments
            assert_eq!(output.tool_version, "4.13.0");
            assert_eq!(output.output, json!({"token": "secret"}));
            let probe_env = fs::read_to_string(dir.path().join("log/probe.env")).unwrap();
            assert!(!probe_env.contains("secret"));
            let run_env = fs::read_to_string(dir.path().join("log/run.env")).unwrap();
            assert!(run_env.contains("GOMEMLIMIT=256MiB"), "{run_env}");
            assert!(!run_env.contains("NODE_OPTIONS"), "{run_env}");
            let args = fs::read_to_string(dir.path().join("log/run.args")).unwrap();
            assert_eq!(
                args.trim(),
                "--repo=https://github.com/org/repo --format=json --show-details --checks=Code-Review"
            );
        }

        #[tokio::test]
        async fn run_scorecard_without_token_fails() {
            // Setup scorecard tool
            let dir = tempfile::tempdir().unwrap();
            let tool = local(dir.path(), Tool::Scorecard, "echo '{}'; exit 0").await;

            // Run it without a token
            let err = run_local(&scorecard_request(), &tool, None, Duration::from_secs(10))
                .await
                .unwrap_err()
                .to_string();

            // Check the run is rejected before running the tool
            assert!(err.contains("requires a GitHub token"), "{err}");
            assert!(!dir.path().join("log/run.args").exists());
        }

        #[tokio::test]
        async fn run_signal_rejected() {
            // Setup stub killed by a signal
            let dir = tempfile::tempdir().unwrap();
            let tool = local(dir.path(), Tool::Afdocs, "kill -9 $$").await;

            // Run the tool
            let err = run_local(&afdocs_request(), &tool, None, Duration::from_secs(10))
                .await
                .unwrap_err()
                .to_string();

            // Check the abnormal termination is reported
            assert!(err.contains("terminated abnormally"), "{err}");
        }

        #[tokio::test]
        async fn run_unexpected_exit_code_rejected() {
            // Setup afdocs stub exiting with a code other than 0 or 1
            let dir = tempfile::tempdir().unwrap();
            let tool = local(dir.path(), Tool::Afdocs, "echo boom >&2; exit 2").await;

            // Run the tool and check the exit code and stderr are reported
            let err = run_local(&afdocs_request(), &tool, None, Duration::from_secs(10))
                .await
                .unwrap_err()
                .to_string();
            assert!(
                err.contains("exited with code 2") && err.contains("boom"),
                "{err}"
            );

            // Setup scorecard stub exiting with 1 (not accepted for scorecard)
            let tool = local(dir.path(), Tool::Scorecard, "echo '{}'; exit 1").await;

            // Run the tool and check the exit code is rejected
            let err = run_local(
                &scorecard_request(),
                &tool,
                Some("t"),
                Duration::from_secs(10),
            )
            .await
            .unwrap_err()
            .to_string();
            assert!(err.contains("exited with code 1"), "{err}");
        }

        // Helpers.

        /// Build a valid afdocs request.
        fn afdocs_request() -> ToolRequest {
            ToolRequest::Afdocs(AfdocsRequest {
                url: "https://docs.example.org/".to_string(),
                max_links: 20,
            })
        }

        /// Return the last path component of the directory reported by `pwd`.
        fn cwd_name(cwd: &str) -> String {
            Path::new(cwd.trim())
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
        }

        /// Probe a stub for the tool provided reporting a supported version
        /// and running the command given.
        async fn local(dir: &Path, tool: Tool, run_cmd: &str) -> LocalTool {
            let version_cmd = match tool {
                Tool::Afdocs => "echo 0.20.0; exit 0",
                Tool::Scorecard => "echo 'GitVersion: v4.13.0' >&2; exit 0",
            };
            let bin = stub(dir, version_cmd, run_cmd);
            LocalTool::probe(tool, bin).await.unwrap()
        }

        /// Build a valid scorecard request.
        fn scorecard_request() -> ToolRequest {
            ToolRequest::Scorecard(ScorecardRequest {
                checks: vec!["Code-Review".to_string()],
                repo_url: "https://github.com/org/repo".to_string(),
            })
        }

        /// Stub that records how it was invoked in the log dir provided. The
        /// TMPDIR value is recorded as its last path component so it can be
        /// compared against the working directory reported.
        fn stub(dir: &Path, version_cmd: &str, run_cmd: &str) -> PathBuf {
            let log = dir.join("log");
            fs::create_dir_all(&log).unwrap();
            script(
                dir,
                "tool",
                &format!(
                    r#"LOG="{log}"
if [ "$1" = "--version" ] || [ "$1" = "version" ]; then
  env > "$LOG/probe.env"; pwd > "$LOG/probe.cwd"; basename "$TMPDIR" > "$LOG/probe.tmpdir"
  {version_cmd}
fi
env > "$LOG/run.env"; pwd > "$LOG/run.cwd"; basename "$TMPDIR" > "$LOG/run.tmpdir"
echo "$@" > "$LOG/run.args"
while [ $# -gt 0 ]; do
  if [ "$1" = "--config" ]; then cp "$2" "$LOG/run.config"; fi
  shift
done
{run_cmd}"#,
                    log = log.display()
                ),
            )
        }
    }
}
