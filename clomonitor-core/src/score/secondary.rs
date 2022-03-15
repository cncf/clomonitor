use crate::linter::secondary::*;
use serde::{Deserialize, Serialize};

/// Score information for a repository of kind secondary.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Score {
    pub global: usize,
    pub documentation: usize,
    pub license: usize,
}

impl Score {
    /// Create a new score with all values set to zero.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Score {
            global: 0,
            documentation: 0,
            license: 0,
        }
    }
}

/// Calculate score for the given report produced by the core linter for a
/// repository of kind secondary.
pub(crate) fn calculate_score(report: &Report) -> Score {
    let mut score = Score::new();

    // Documentation
    if report.documentation.contributing.passed {
        score.documentation += 20;
    }
    if report.documentation.maintainers.passed {
        score.documentation += 10;
    }
    if report.documentation.readme.passed {
        score.documentation += 70;
    }

    // License
    if report.license.approved.passed {
        score.license += 75;
    }
    if report.license.spdx_id.passed {
        score.license += 25;
    }

    // Global
    let global = (score.documentation as f64 + score.license as f64) / 2.0;
    score.global = global.round() as usize;

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linter::CheckResult;

    #[test]
    fn new_returns_all_zeroes_score() {
        assert_eq!(
            Score::new(),
            Score {
                global: 0,
                documentation: 0,
                license: 0,
            }
        );
    }

    #[test]
    fn calculate_score_report_with_all_checks_passed_got_max_score() {
        assert_eq!(
            calculate_score(&Report {
                documentation: Documentation {
                    contributing: true.into(),
                    maintainers: true.into(),
                    readme: true.into(),
                },
                license: License {
                    approved: CheckResult {
                        passed: true,
                        value: Some(true),
                        ..Default::default()
                    },
                    spdx_id: Some("Apache-2.0".to_string()).into(),
                },
            }),
            Score {
                global: 100,
                documentation: 100,
                license: 100,
            }
        );
    }

    #[test]
    fn calculate_score_report_with_no_checks_passed_got_min_score() {
        assert_eq!(
            calculate_score(&Report {
                documentation: Documentation {
                    contributing: false.into(),
                    maintainers: false.into(),
                    readme: false.into(),
                },
                license: License {
                    approved: CheckResult {
                        passed: false,
                        ..Default::default()
                    },
                    spdx_id: None.into(),
                },
            }),
            Score::new()
        );
    }
}
