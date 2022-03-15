use crate::linter::primary::*;
use serde::{Deserialize, Serialize};

/// Score information for a repository of kind primary.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Score {
    pub global: usize,
    pub documentation: usize,
    pub license: usize,
    pub best_practices: usize,
    pub security: usize,
    pub legal: usize,
}

impl Score {
    /// Create a new score with all values set to zero.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Score {
            global: 0,
            documentation: 0,
            license: 0,
            best_practices: 0,
            security: 0,
            legal: 0,
        }
    }
}

/// Calculate score for the given report produced by the core linter for a
/// repository of kind primary.
pub(crate) fn calculate_score(report: &Report) -> Score {
    let mut score = Score::new();

    // Documentation
    if report.documentation.adopters.passed {
        score.documentation += 5;
    }
    if report.documentation.changelog.passed {
        score.documentation += 5;
    }
    if report.documentation.code_of_conduct.passed {
        score.documentation += 5;
    }
    if report.documentation.contributing.passed {
        score.documentation += 10;
    }
    if report.documentation.governance.passed {
        score.documentation += 10;
    }
    if report.documentation.maintainers.passed {
        score.documentation += 5;
    }
    if report.documentation.readme.passed {
        score.documentation += 50;
    }
    if report.documentation.roadmap.passed {
        score.documentation += 5;
    }
    if report.documentation.website.passed {
        score.documentation += 5;
    }

    // License
    if report.license.approved.passed {
        score.license += 60;
    }
    if report.license.scanning.passed {
        score.license += 20;
    }
    if report.license.spdx_id.passed {
        score.license += 20;
    }

    // Best practices
    if report.best_practices.artifacthub_badge.passed {
        score.best_practices += 5;
    }
    if report.best_practices.community_meeting.passed {
        score.best_practices += 25;
    }
    if report.best_practices.dco.passed {
        score.best_practices += 10;
    }
    if report.best_practices.openssf_badge.passed {
        score.best_practices += 50;
    }
    if report.best_practices.recent_release.passed {
        score.best_practices += 10;
    }

    // Security
    if report.security.security_policy.passed {
        score.security += 100;
    }

    // Legal
    if report.legal.trademark_disclaimer.passed {
        score.legal += 100;
    }

    // Global
    let global = (score.documentation as f64 * 0.3)
        + (score.license as f64 * 0.2)
        + (score.best_practices as f64 * 0.2)
        + (score.security as f64 * 0.2)
        + (score.legal as f64 * 0.1);
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
                best_practices: 0,
                security: 0,
                legal: 0,
            }
        );
    }

    #[test]
    fn calculate_score_report_with_all_checks_passed_got_max_score() {
        assert_eq!(
            calculate_score(&Report {
                documentation: Documentation {
                    adopters: true.into(),
                    code_of_conduct: true.into(),
                    contributing: true.into(),
                    changelog: true.into(),
                    governance: true.into(),
                    maintainers: true.into(),
                    readme: true.into(),
                    roadmap: true.into(),
                    website: true.into(),
                },
                license: License {
                    approved: CheckResult {
                        passed: true,
                        value: Some(true),
                        ..Default::default()
                    },
                    scanning: CheckResult::from_url(Some(
                        "https://license-scanning.url".to_string()
                    )),
                    spdx_id: Some("Apache-2.0".to_string()).into(),
                },
                best_practices: BestPractices {
                    artifacthub_badge: true.into(),
                    community_meeting: true.into(),
                    dco: true.into(),
                    openssf_badge: true.into(),
                    recent_release: true.into(),
                },
                security: Security {
                    security_policy: true.into(),
                },
                legal: Legal {
                    trademark_disclaimer: true.into(),
                },
            }),
            Score {
                global: 100,
                documentation: 100,
                license: 100,
                best_practices: 100,
                security: 100,
                legal: 100,
            }
        );
    }

    #[test]
    fn calculate_score_report_with_no_checks_passed_got_min_score() {
        assert_eq!(
            calculate_score(&Report {
                documentation: Documentation {
                    adopters: false.into(),
                    code_of_conduct: false.into(),
                    contributing: false.into(),
                    changelog: false.into(),
                    governance: false.into(),
                    maintainers: false.into(),
                    readme: false.into(),
                    roadmap: false.into(),
                    website: false.into(),
                },
                license: License {
                    approved: CheckResult {
                        passed: false,
                        ..Default::default()
                    },
                    scanning: CheckResult {
                        ..Default::default()
                    },
                    spdx_id: None.into(),
                },
                best_practices: BestPractices {
                    artifacthub_badge: false.into(),
                    community_meeting: false.into(),
                    dco: false.into(),
                    openssf_badge: false.into(),
                    recent_release: false.into(),
                },
                security: Security {
                    security_policy: false.into(),
                },
                legal: Legal {
                    trademark_disclaimer: false.into(),
                },
            }),
            Score::new()
        );
    }
}
