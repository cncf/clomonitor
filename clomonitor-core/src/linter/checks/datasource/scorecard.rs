use anyhow::{Context, Error, Result, bail};
use serde::Deserialize;

use crate::{
    linter::{
        ToolMode,
        check::CheckOutput,
        checks::{CHECKS, signed_releases},
    },
    tools::{
        self, LocalTool, RunOutput, SCORECARD_CHECKS, ScorecardRequest, Tool, ToolRequest,
        runner::RunnerClient,
    },
};

/// Scorecard report (list of checks).
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Scorecard {
    checks: Vec<ScorecardCheck>,
}

impl Scorecard {
    /// Build a scorecard from the output of a tool run.
    pub(crate) fn from_run_output(run: RunOutput) -> Result<Self> {
        if run.tool != Tool::Scorecard {
            bail!("unexpected tool output: {}", run.tool);
        }
        serde_json::from_value(run.output).context("error parsing scorecard output")
    }
}

/// Scorecard check details.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct ScorecardCheck {
    pub name: String,
    pub reason: String,
    pub details: Option<Vec<String>>,
    pub score: f64,
    pub documentation: ScorecardCheckDocs,
}

impl<T> From<Result<Option<&ScorecardCheck>, &Error>> for CheckOutput<T> {
    fn from(sc_check: Result<Option<&ScorecardCheck>, &Error>) -> Self {
        match sc_check {
            Ok(sc_check) => match sc_check {
                Some(sc_check) => {
                    let signed_releases = check_name(signed_releases::ID);
                    let mut output = CheckOutput::default();
                    let pass_threshold = match &sc_check.name {
                        n if n == signed_releases => 1.0,
                        _ => 5.0,
                    };
                    if sc_check.score >= pass_threshold {
                        output.passed = true;
                    }
                    output.details = Some(format!(
                        r"# {} OpenSSF Scorecard check

**Score**: {} (check passes with score >= {})

**Reason**: {}

**Details**: {}

**Please see the [check documentation]({}) in the ossf/scorecard repository for more details**",
                        sc_check.name,
                        sc_check.score,
                        pass_threshold,
                        sc_check.reason,
                        match &sc_check.details {
                            Some(details) => format!("\n\n>{}", details.join("\n")),
                            None => "-".to_string(),
                        },
                        sc_check.documentation.url,
                    ));
                    output
                }
                None => CheckOutput::not_passed(),
            },
            Err(err) => CheckOutput::failed().fail_reason(Some(format!("{err:#}"))),
        }
    }
}

/// Scorecard check documentation.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct ScorecardCheckDocs {
    pub url: String,
}

/// Get repository's OpenSSF Scorecard using the mode requested.
pub(crate) async fn scorecard(
    repo_url: &str,
    github_token: &str,
    mode: &ToolMode,
) -> Result<Scorecard> {
    let request = ToolRequest::Scorecard(ScorecardRequest {
        checks: SCORECARD_CHECKS.iter().map(ToString::to_string).collect(),
        repo_url: repo_url.to_string(),
    });
    let run = match mode {
        ToolMode::Disabled => bail!("Scorecard is not configured"),
        ToolMode::Local => {
            let tool = LocalTool::locate(Tool::Scorecard).await?;
            tools::run_local(
                &request,
                &tool,
                Some(github_token),
                Tool::Scorecard.deadline(),
            )
            .await?
        }
        ToolMode::Runner { url } => {
            RunnerClient::new(url)?
                .run(&request, Some(github_token))
                .await?
        }
    };
    Scorecard::from_run_output(run)
}

/// Return the scorecard check name the check provided is backed by.
pub(crate) fn check_name(check_id: &str) -> &str {
    CHECKS[check_id]
        .datasource
        .as_ref()
        .and_then(|ds| ds.scorecard_name())
        .expect("check to be backed by scorecard")
}

// Get a check from the scorecard provided if available.
pub(crate) fn get_check<'a>(
    scorecard: &'a Result<Scorecard>,
    check_id: &'a str,
) -> Result<Option<&'a ScorecardCheck>, &'a Error> {
    match scorecard {
        Ok(scorecard) => {
            let name = check_name(check_id);
            Ok(scorecard.checks.iter().find(|c| c.name == name))
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use anyhow::format_err;

    use crate::linter::checks::code_review;

    use super::*;

    #[test]
    fn get_check_found() {
        let scorecard = Ok(Scorecard {
            checks: vec![ScorecardCheck {
                name: "Code-Review".to_string(),
                reason: "test".to_string(),
                details: None,
                score: 8.0,
                documentation: ScorecardCheckDocs {
                    url: "https://test.url".to_string(),
                },
            }],
        });

        assert_eq!(
            get_check(&scorecard, code_review::ID).unwrap().unwrap(),
            &scorecard.as_ref().unwrap().checks[0]
        );
    }

    #[test]
    fn scorecard_from_run_output() {
        let run = RunOutput {
            duration_ms: 1,
            output: serde_json::json!({
                "checks": [{
                    "name": "Code-Review",
                    "reason": "test",
                    "score": 8.0,
                    "documentation": {"url": "https://test.url"}
                }]
            }),
            tool: Tool::Scorecard,
            tool_version: "4.13.0".to_string(),
        };
        let scorecard = Scorecard::from_run_output(run).unwrap();
        assert_eq!(scorecard.checks.len(), 1);

        let run = RunOutput {
            duration_ms: 1,
            output: serde_json::json!({"checks": []}),
            tool: Tool::Afdocs,
            tool_version: "0.20.0".to_string(),
        };
        assert!(Scorecard::from_run_output(run).is_err());

        let run = RunOutput {
            duration_ms: 1,
            output: serde_json::json!({"unexpected": true}),
            tool: Tool::Scorecard,
            tool_version: "4.13.0".to_string(),
        };
        assert!(Scorecard::from_run_output(run).is_err());
    }

    #[tokio::test]
    async fn scorecard_disabled() {
        let err = scorecard("https://github.com/org/repo", "token", &ToolMode::Disabled)
            .await
            .unwrap_err()
            .to_string();
        assert_eq!(err, "Scorecard is not configured");
    }

    #[test]
    fn get_check_not_found() {
        let scorecard = Ok(Scorecard { checks: vec![] });

        assert!(get_check(&scorecard, code_review::ID).unwrap().is_none());
    }

    #[test]
    fn check_output_from_scorecard_check_passed() {
        let sc_check = ScorecardCheck {
            name: "Code-Review".to_string(),
            reason: "reason".to_string(),
            details: Some(vec!["details".to_string()]),
            score: 8.0,
            documentation: ScorecardCheckDocs {
                url: "https://test.url".to_string(),
            },
        };

        assert_eq!(
            CheckOutput::<()>::from(Ok(Some(&sc_check))),
            CheckOutput {
                passed: true,
                details: Some("# Code-Review OpenSSF Scorecard check\n\n**Score**: 8 (check passes with score >= 5)\n\n**Reason**: reason\n\n**Details**: \n\n>details\n\n**Please see the [check documentation](https://test.url) in the ossf/scorecard repository for more details**".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_output_from_scorecard_check_not_passed() {
        let sc_check = ScorecardCheck {
            name: "Code-Review".to_string(),
            reason: "reason".to_string(),
            details: Some(vec!["details".to_string()]),
            score: 4.0,
            documentation: ScorecardCheckDocs {
                url: "https://test.url".to_string(),
            },
        };

        assert_eq!(
            CheckOutput::<()>::from(Ok(Some(&sc_check))),
            CheckOutput {
                passed: false,
                details: Some("# Code-Review OpenSSF Scorecard check\n\n**Score**: 4 (check passes with score >= 5)\n\n**Reason**: reason\n\n**Details**: \n\n>details\n\n**Please see the [check documentation](https://test.url) in the ossf/scorecard repository for more details**".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_output_from_scorecard_signed_releases_threshold() {
        let sc_check = ScorecardCheck {
            name: "Signed-Releases".to_string(),
            reason: "reason".to_string(),
            details: None,
            score: 1.0,
            documentation: ScorecardCheckDocs {
                url: "https://test.url".to_string(),
            },
        };

        assert!(CheckOutput::<()>::from(Ok(Some(&sc_check))).passed);
    }

    #[test]
    fn check_output_from_scorecard_check_not_available() {
        assert_eq!(
            CheckOutput::<()>::from(Ok(None)),
            CheckOutput {
                passed: false,
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_output_from_scorecard_check_failed() {
        let err = format_err!("fake error");
        let sc_check: Result<Option<&ScorecardCheck>, &Error> = Err(&err);

        assert_eq!(
            CheckOutput::<()>::from(sc_check),
            CheckOutput {
                failed: true,
                fail_reason: Some("fake error".to_string()),
                ..Default::default()
            }
        );
    }
}
