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
        }
    }
}

/// Calculate score for the given report produced by the core linter for a
/// repository of kind primary.
pub(crate) fn calculate_score(report: &Report) -> Score {
    let mut score = Score::new();

    // Documentation
    if report.documentation.adopters {
        score.documentation += 5;
    }
    if report.documentation.code_of_conduct {
        score.documentation += 5;
    }
    if report.documentation.contributing {
        score.documentation += 10;
    }
    if report.documentation.changelog {
        score.documentation += 5;
    }
    if report.documentation.governance {
        score.documentation += 10;
    }
    if report.documentation.maintainers {
        score.documentation += 5;
    }
    if report.documentation.readme {
        score.documentation += 50;
    }
    if report.documentation.roadmap {
        score.documentation += 5;
    }
    if report.documentation.website {
        score.documentation += 5;
    }

    // License
    if let Some(approved) = report.license.approved {
        if approved {
            score.license += 60;
        }
    }
    if report.license.scanning.is_some() {
        score.license += 20;
    }
    if report.license.spdx_id.is_some() {
        score.license += 20;
    }

    // Best practices
    if report.best_practices.artifacthub_badge {
        score.best_practices += 5;
    }
    if report.best_practices.community_meeting {
        score.best_practices += 25;
    }
    if report.best_practices.openssf_badge {
        score.best_practices += 50;
    }
    if report.best_practices.recent_release {
        score.best_practices += 10;
    }
    if report.best_practices.trademark_footer {
        score.best_practices += 10;
    }

    // Security
    if report.security.security_policy {
        score.security = 100;
    }

    // Global
    let global = (score.documentation as f64
        + score.license as f64
        + score.best_practices as f64
        + score.security as f64)
        / 4.0;
    score.global = global.round() as usize;

    score
}

#[cfg(test)]
mod tests {
    use super::*;

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
            }
        );
    }

    #[test]
    fn calculate_score_report_with_all_checks_passed_got_max_score() {
        assert_eq!(
            calculate_score(&Report {
                documentation: Documentation {
                    adopters: true,
                    code_of_conduct: true,
                    contributing: true,
                    changelog: true,
                    governance: true,
                    maintainers: true,
                    readme: true,
                    roadmap: true,
                    website: true,
                },
                license: License {
                    approved: Some(true),
                    scanning: Some("https://license-scanning.url".to_string()),
                    spdx_id: Some("Apache-2.0".to_string()),
                },
                best_practices: BestPractices {
                    artifacthub_badge: true,
                    community_meeting: true,
                    openssf_badge: true,
                    recent_release: true,
                    trademark_footer: true,
                },
                security: Security {
                    security_policy: true,
                },
            }),
            Score {
                global: 100,
                documentation: 100,
                license: 100,
                best_practices: 100,
                security: 100,
            }
        );
    }

    #[test]
    fn calculate_score_report_with_no_checks_passed_got_min_score() {
        assert_eq!(
            calculate_score(&Report {
                documentation: Documentation {
                    adopters: false,
                    code_of_conduct: false,
                    contributing: false,
                    changelog: false,
                    governance: false,
                    maintainers: false,
                    readme: false,
                    roadmap: false,
                    website: false,
                },
                license: License {
                    approved: None,
                    scanning: None,
                    spdx_id: None,
                },
                best_practices: BestPractices {
                    artifacthub_badge: false,
                    community_meeting: false,
                    openssf_badge: false,
                    recent_release: false,
                    trademark_footer: false,
                },
                security: Security {
                    security_policy: false,
                },
            }),
            Score::new()
        );
    }
}
