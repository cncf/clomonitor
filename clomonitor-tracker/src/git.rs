use std::{path::Path, sync::Arc};

use anyhow::{format_err, Result};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use tokio::process::Command;
use which::which;

/// Type alias to represent a Git trait object.
pub(crate) type DynGit = Arc<dyn Git + Send + Sync>;

/// Trait that defines some operations a Git implementation must support.
#[async_trait]
#[cfg_attr(test, automock)]
pub(crate) trait Git {
    /// Clone (shallow) the repository in the destination path provided.
    async fn clone_repository(&self, url: &str, dst: &Path) -> Result<()>;

    /// Get the remote digest of a repository.
    async fn remote_digest(&self, url: &str) -> Result<String>;
}

/// Git implementation backed by the git cli tool.
pub(crate) struct GitCLI;

impl GitCLI {
    /// Create a new GitCLI instance.
    pub(crate) fn new() -> Result<Self> {
        if which("git").is_err() {
            return Err(format_err!("git not found in PATH"));
        }
        Ok(Self {})
    }
}

#[async_trait]
impl Git for GitCLI {
    async fn clone_repository(&self, url: &str, dst: &Path) -> Result<()> {
        let output = Command::new("git")
            .arg("clone")
            .arg("--depth=10")
            .arg(url)
            .arg(dst)
            .output()
            .await?;
        if !output.status.success() {
            return Err(format_err!("{}", String::from_utf8_lossy(&output.stderr)));
        }
        Ok(())
    }

    async fn remote_digest(&self, url: &str) -> Result<String> {
        let output = Command::new("git")
            .arg("ls-remote")
            .arg(url)
            .arg("HEAD")
            .output()
            .await?;
        if !output.status.success() {
            return Err(format_err!("{}", String::from_utf8_lossy(&output.stderr)));
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout
            .split_whitespace()
            .next()
            .expect("value present, status checked above")
            .to_string())
    }
}
