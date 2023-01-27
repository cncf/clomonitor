use super::{
    checks::{
        signed_releases,
        util::{
            github,
            scorecard::{Scorecard, ScorecardCheck},
        },
        CHECKS,
    },
    metadata::{Exemption, Metadata, METADATA_FILE},
    util::scorecard::scorecard,
    CheckSet, LinterInput,
};
use anyhow::{format_err, Context, Error, Result};
use serde::{Deserialize, Serialize};
use which::which;

/// Type alias to represent a check identifier.
pub type CheckId = &'static str;

/// Check configuration.
pub(crate) struct CheckConfig {
    pub weight: usize,
    pub check_sets: Vec<CheckSet>,
    pub scorecard_name: Option<String>,
}

/// Input used by checks to perform their operations.
#[derive(Debug)]
pub(crate) struct CheckInput<'a> {
    pub li: &'a LinterInput,
    pub cm_md: Option<Metadata>,
    pub gh_md: github::md::MdRepository,
    pub scorecard: Result<Scorecard>,
}

impl<'a> CheckInput<'a> {
    pub(crate) async fn new(li: &LinterInput) -> Result<CheckInput> {
        // Check if required external tools are available
        if which("scorecard").is_err() {
            return Err(format_err!(
                "scorecard not found in PATH (https://github.com/ossf/scorecard#installation)"
            ));
        }

        // Get CLOMonitor metadata
        let cm_md = Metadata::from(li.root.join(METADATA_FILE))?;

        // The next both actions (get GitHub metadata and get scorecard) make use
        // of the GitHub token, which when used concurrently, may trigger some
        // GitHub secondary rate limits. So they should not be run concurrently.

        // Get GitHub metadata
        let gh_md = github::metadata(&li.url, &li.github_token).await?;

        // Get OpenSSF scorecard
        let scorecard = scorecard(&li.url, &li.github_token)
            .await
            .context("error running scorecard command");

        // Prepare and return check input
        let ci = CheckInput {
            li,
            cm_md,
            gh_md,
            scorecard,
        };
        Ok(ci)
    }
}

/// Check output information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckOutput<T = ()> {
    pub passed: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    pub exempt: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption_reason: Option<String>,

    pub failed: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_reason: Option<String>,
}

impl<T> CheckOutput<T> {
    /// Create a new CheckOutput instance with the passed field set to true.
    pub fn passed() -> Self {
        Self {
            passed: true,
            ..Default::default()
        }
    }

    /// Create a new CheckOutput instance with the passed field set to false.
    pub fn not_passed() -> Self {
        Self {
            passed: false,
            ..Default::default()
        }
    }

    /// Create a new CheckOutput instance with the exempt field set to true.
    pub fn exempt() -> Self {
        Self {
            exempt: true,
            ..Default::default()
        }
    }

    /// Create a new CheckOutput instance with the failed field set to true.
    pub fn failed() -> Self {
        Self {
            failed: true,
            ..Default::default()
        }
    }

    /// Url field setter.
    pub fn url(mut self, url: Option<String>) -> CheckOutput<T> {
        self.url = url;
        self
    }

    /// Value field setter.
    pub fn value(mut self, value: Option<T>) -> CheckOutput<T> {
        self.value = value;
        self
    }

    /// Details field setter.
    pub fn details(mut self, details: Option<String>) -> CheckOutput<T> {
        self.details = details;
        self
    }

    /// Exemption reason field setter.
    pub fn examption_reason(mut self, reason: Option<String>) -> CheckOutput<T> {
        self.exemption_reason = reason;
        self
    }

    /// Fail reason field setter.
    pub fn fail_reason(mut self, reason: Option<String>) -> CheckOutput<T> {
        self.fail_reason = reason;
        self
    }
}

impl<T> Default for CheckOutput<T> {
    fn default() -> Self {
        Self {
            passed: false,
            url: None,
            value: None,
            details: None,
            exempt: false,
            exemption_reason: None,
            failed: false,
            fail_reason: None,
        }
    }
}

impl<T> From<Exemption> for CheckOutput<T> {
    fn from(exemption: Exemption) -> Self {
        Self::exempt().examption_reason(Some(exemption.reason))
    }
}

impl<T> From<Result<Option<&ScorecardCheck>, &Error>> for CheckOutput<T> {
    fn from(sc_check: Result<Option<&ScorecardCheck>, &Error>) -> Self {
        match sc_check {
            Ok(sc_check) => match sc_check {
                Some(sc_check) => {
                    let signed_releases =
                        CHECKS[signed_releases::ID].scorecard_name.as_ref().unwrap();
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

/// Wrapper macro that takes care of running some common pre-check operations
/// and the synchronous check function.
macro_rules! run {
    ($check:ident, $input:expr) => {
        (|| {
            // Check if this check should be skipped
            if should_skip_check($check::ID, &$input.li.check_sets) {
                return None;
            }

            // Check if an exemption has been declared for this check
            if let Some(exemption) = find_exemption($check::ID, $input.cm_md.as_ref()) {
                return Some(CheckOutput::from(exemption));
            }

            // Call sync check function and wrap returned check output in an option
            let output = match $check::check($input) {
                Ok(output) => output,
                Err(err) => CheckOutput::failed().fail_reason(Some(format!("{:#}", err))),
            };
            Some(output)
        })()
    };
}
pub(crate) use run;

/// Wrapper macro that takes care of running some common pre-check operations
/// and the asynchronous check function.
macro_rules! run_async {
    ($check:ident, $input:expr) => {
        (|| async {
            // Check if this check should be skipped
            if should_skip_check($check::ID, &$input.li.check_sets) {
                return None;
            }

            // Check if an exemption has been declared for this check
            if let Some(exemption) = find_exemption($check::ID, $input.cm_md.as_ref()) {
                return Some(CheckOutput::from(exemption));
            }

            // Call async check function and wrap returned check output in an option
            let output = match $check::check($input).await {
                Ok(output) => output,
                Err(err) => CheckOutput::failed().fail_reason(Some(format!("{:#}", err))),
            };
            Some(output)
        })()
    };
}
pub(crate) use run_async;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linter::checks::util::scorecard::ScorecardCheckDocs;
    use anyhow::{format_err, Result};

    #[test]
    fn check_output_from_exemption() {
        let exemption = Exemption {
            check: "test".to_string(),
            reason: "test".to_string(),
        };

        assert_eq!(
            CheckOutput::<()>::from(exemption),
            CheckOutput {
                exempt: true,
                exemption_reason: Some("test".to_string()),
                ..Default::default()
            }
        );
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
