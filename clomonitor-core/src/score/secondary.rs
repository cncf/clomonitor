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
    if report.documentation.contributing {
        score.documentation += 20;
    }
    if report.documentation.maintainers {
        score.documentation += 10;
    }
    if report.documentation.readme {
        score.documentation += 70;
    }

    // License
    if let Some(approved) = report.license.approved {
        if approved {
            score.license += 75;
        }
    }
    if report.license.spdx_id.is_some() {
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
    fn report_with_all_checks_passed_got_max_score() {
        assert_eq!(
            calculate_score(&Report {
                documentation: Documentation {
                    contributing: true,
                    maintainers: true,
                    readme: true,
                },
                license: License {
                    approved: Some(true),
                    spdx_id: Some("Apache-2.0".to_string()),
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
    fn report_with_no_checks_passed_got_min_score() {
        assert_eq!(
            calculate_score(&Report {
                documentation: Documentation {
                    contributing: false,
                    maintainers: false,
                    readme: false,
                },
                license: License {
                    approved: None,
                    spdx_id: None,
                },
            }),
            Score::new()
        );
    }
}
