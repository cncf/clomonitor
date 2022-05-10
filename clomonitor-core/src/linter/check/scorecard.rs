use crate::config::SCORECARD_CHECK;
use anyhow::{format_err, Result};
use serde::Deserialize;
use tokio::process::Command;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Scorecard {
    checks: Vec<ScorecardCheck>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct ScorecardCheck {
    pub name: String,
    pub reason: String,
    pub details: Option<Vec<String>>,
    pub score: f64,
    pub documentation: ScorecardCheckDocs,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct ScorecardCheckDocs {
    pub url: String,
}

impl Scorecard {
    /// Get a check from the scoreboard if available.
    pub(crate) fn get_check(&self, check_id: &str) -> Option<&ScorecardCheck> {
        self.checks
            .iter()
            .find(|c| c.name == SCORECARD_CHECK[check_id])
    }
}

/// Get repository's OpenSSF Scorecard.
pub(crate) async fn scorecard(repo_url: &str, github_token: &str) -> Result<Scorecard> {
    let output = Command::new("scorecard")
        .env("GITHUB_TOKEN", github_token)
        .env_remove("GITHUB_REF")
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CODE_REVIEW;

    #[test]
    fn get_check_found() {
        let scorecard = Scorecard {
            checks: vec![ScorecardCheck {
                name: "Code-Review".to_string(),
                reason: "test".to_string(),
                details: None,
                score: 8.0,
                documentation: ScorecardCheckDocs {
                    url: "https://test.url".to_string(),
                },
            }],
        };

        assert_eq!(
            scorecard.get_check(CODE_REVIEW).unwrap(),
            &scorecard.checks[0]
        );
    }

    #[test]
    fn get_check_not_found() {
        let scorecard = Scorecard { checks: vec![] };

        assert!(matches!(scorecard.get_check(CODE_REVIEW), None));
    }
}
