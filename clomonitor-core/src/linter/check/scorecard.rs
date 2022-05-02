use crate::config::SCORECARD_CHECK;
use anyhow::{format_err, Error, Result};
use cached::proc_macro::cached;
use futures::{
    future::{BoxFuture, Shared},
    FutureExt, TryFutureExt,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::process::Command;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Scorecard {
    checks: Vec<ScorecardCheck>,
}

impl Scorecard {
    /// Get a check from the scoreboard if available.
    pub(crate) fn get_check(&self, check_id: &str) -> Option<&ScorecardCheck> {
        self.checks
            .iter()
            .find(|c| c.name == SCORECARD_CHECK[check_id])
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ScorecardCheck {
    pub name: String,
    pub reason: String,
    pub details: Option<Vec<String>>,
    pub score: f64,
    pub documentation: ScorecardCheckDocs,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ScorecardCheckDocs {
    pub url: String,
}

/// Type alias for a shared boxed future that will resolve to a result
/// containing a scorecard.
type ScorecardFuture = Shared<BoxFuture<'static, Result<Scorecard, Arc<Error>>>>;

/// Get repository's OpenSSF Scorecard.
#[cached(
    sync_writes = true,
    key = "String",
    convert = r#"{ format!("{}", repo_url) }"#
)]
pub(crate) fn scorecard(repo_url: String, github_token: String) -> ScorecardFuture {
    async fn get_scorecard(repo_url: String, github_token: String) -> Result<Scorecard> {
        let output = Command::new("scorecard")
            .env("GITHUB_AUTH_TOKEN", github_token)
            .arg(format!("--repo={repo_url}"))
            .arg("--format=json")
            .arg("--show-details")
            .arg("--checks=Binary-Artifacts,Branch-Protection,Code-Review,Dangerous-Workflow,Dependency-Update-Tool,Maintained,Signed-Releases,Token-Permissions,Vulnerabilities")
            .output()
            .await?;
        if !output.status.success() {
            return Err(format_err!("{}", String::from_utf8_lossy(&output.stderr)));
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let scorecard: Scorecard = serde_json::from_str(stdout.as_ref())?;
        Ok(scorecard)
    }

    get_scorecard(repo_url, github_token)
        .map_err(Arc::new)
        .boxed()
        .shared()
}
