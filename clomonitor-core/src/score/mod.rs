use crate::linter::*;
use serde::{Deserialize, Serialize};

/// Score information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Score {
    pub global: f64,
    pub global_weight: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_weight: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_weight: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_practices: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_practices_weight: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_weight: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_weight: Option<usize>,
}

impl Score {
    /// Return the score's global value.
    pub fn global(&self) -> f64 {
        self.global
    }

    /// Return the score's rating (a, b, c or d).
    pub fn rating(&self) -> char {
        rating(self.global())
    }
}

/// Calculate score for the given linter report.
pub fn calculate(report: &Report) -> Score {
    let mut score = Score::default();

    // Sections
    (score.documentation, score.documentation_weight) = calculate_section(
        &report.documentation.available(),
        &report.documentation.passed_or_exempt(),
    );
    (score.license, score.license_weight) = calculate_section(
        &report.license.available(),
        &report.license.passed_or_exempt(),
    );
    (score.best_practices, score.best_practices_weight) = calculate_section(
        &report.best_practices.available(),
        &report.best_practices.passed_or_exempt(),
    );
    (score.security, score.security_weight) = calculate_section(
        &report.security.available(),
        &report.security.passed_or_exempt(),
    );
    (score.legal, score.legal_weight) =
        calculate_section(&report.legal.available(), &report.legal.passed_or_exempt());

    // Global
    let sections_scores = &[
        score.documentation,
        score.license,
        score.best_practices,
        score.security,
        score.legal,
    ];
    let sections_weights = &[
        score.documentation_weight,
        score.license_weight,
        score.best_practices_weight,
        score.security_weight,
        score.legal_weight,
    ];
    score.global_weight = sections_weights
        .iter()
        .fold(0, |gw, sw| gw + sw.unwrap_or_default());
    score.global = sections_scores
        .iter()
        .zip(sections_weights.iter())
        .fold(0.0, |gs, (ss, sw)| {
            let k = sw.unwrap_or_default() as f64 / score.global_weight as f64;
            gs + ss.unwrap_or_default() * k
        });

    score
}

/// Calculate score and weight for a report's section from the checks provided.
fn calculate_section(
    checks_available: &[CheckId],
    checks_passed_or_exempt: &[CheckId],
) -> (Option<f64>, Option<usize>) {
    // Calculate section weight
    let weight = checks_available
        .iter()
        .fold(0, |weight, check_id| weight + CHECKS[check_id].weight);
    if weight == 0 {
        return (None, None);
    }

    // Calculate section score
    let score = checks_passed_or_exempt.iter().fold(0.0, |score, check_id| {
        score + CHECKS[check_id].weight as f64 / weight as f64 * 100.0
    });

    (Some(score), Some(weight))
}

/// Merge the scores provided into a single score.
pub fn merge(scores: &[Score]) -> Score {
    // Sum all scores weights for each of the sections. We'll use them to
    // calculate the coefficient we'll apply to each of the scores.
    let mut global_weights_sum = 0;
    let mut documentation_weights_sum = 0;
    let mut license_weights_sum = 0;
    let mut best_practices_weights_sum = 0;
    let mut security_weights_sum = 0;
    let mut legal_weights_sum = 0;
    for score in scores {
        global_weights_sum += score.global_weight;
        documentation_weights_sum += score.documentation_weight.unwrap_or_default();
        license_weights_sum += score.license_weight.unwrap_or_default();
        best_practices_weights_sum += score.best_practices_weight.unwrap_or_default();
        security_weights_sum += score.security_weight.unwrap_or_default();
        legal_weights_sum += score.legal_weight.unwrap_or_default();
    }

    // Helper function that merges a score into the merged value provided after
    // applying the given coefficient to it.
    let merge = |merged: Option<f64>, score: Option<f64>, k: f64| -> Option<f64> {
        if let Some(v) = score {
            return match merged {
                Some(mv) => Some(mv + v * k),
                None => Some(v * k),
            };
        }
        merged
    };

    // Calculate merged score for each of the sections.
    let mut m = Score::default();
    for s in scores {
        m.global += s.global * (s.global_weight as f64 / global_weights_sum as f64);
        m.documentation = merge(
            m.documentation,
            s.documentation,
            s.documentation_weight.unwrap_or_default() as f64 / documentation_weights_sum as f64,
        );
        m.license = merge(
            m.license,
            s.license,
            s.license_weight.unwrap_or_default() as f64 / license_weights_sum as f64,
        );
        m.best_practices = merge(
            m.best_practices,
            s.best_practices,
            s.best_practices_weight.unwrap_or_default() as f64 / best_practices_weights_sum as f64,
        );
        m.security = merge(
            m.security,
            s.security,
            s.security_weight.unwrap_or_default() as f64 / security_weights_sum as f64,
        );
        m.legal = merge(
            m.legal,
            s.legal,
            s.legal_weight.unwrap_or_default() as f64 / legal_weights_sum as f64,
        );
    }

    m
}

/// Return the score's rating (a, b, c or d).
pub fn rating(score: f64) -> char {
    match score as usize {
        75..=100 => 'a',
        50..=74 => 'b',
        25..=49 => 'c',
        0..=24 => 'd',
        _ => '?',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_global() {
        assert_eq!(
            Score {
                global: 10.0,
                ..Score::default()
            }
            .global(),
            10.0
        );
    }

    #[test]
    fn score_rating() {
        assert_eq!(
            Score {
                global: 80.0,
                ..Score::default()
            }
            .rating(),
            'a'
        );
    }

    #[test]
    fn rating_returns_correct_level() {
        assert_eq!(rating(80.0), 'a');
        assert_eq!(rating(75.0), 'a');
        assert_eq!(rating(74.0), 'b');
        assert_eq!(rating(50.0), 'b');
        assert_eq!(rating(49.0), 'c');
        assert_eq!(rating(25.0), 'c');
        assert_eq!(rating(20.0), 'd');
    }

    #[test]
    fn calculate_report_with_all_checks_passed_got_max_score() {
        assert_eq!(
            calculate(&Report {
                documentation: Documentation {
                    adopters: Some(CheckOutput::passed()),
                    code_of_conduct: Some(CheckOutput::passed()),
                    contributing: Some(CheckOutput::passed()),
                    changelog: Some(CheckOutput::passed()),
                    governance: Some(CheckOutput::passed()),
                    maintainers: Some(CheckOutput::passed()),
                    readme: Some(CheckOutput::passed()),
                    roadmap: Some(CheckOutput::passed()),
                    website: Some(CheckOutput::passed()),
                },
                license: License {
                    license_approved: Some(CheckOutput::passed()),
                    license_scanning: Some(
                        CheckOutput::passed().url(Some("https://license-scanning.url".to_string()))
                    ),
                    license_spdx_id: Some(
                        CheckOutput::passed().value(Some("Apache-2.0".to_string()))
                    ),
                },
                best_practices: BestPractices {
                    analytics: Some(CheckOutput::passed()),
                    artifacthub_badge: Some(CheckOutput::exempt()),
                    cla: Some(CheckOutput::passed()),
                    community_meeting: Some(CheckOutput::passed()),
                    dco: Some(CheckOutput::passed()),
                    github_discussions: Some(CheckOutput::passed()),
                    openssf_badge: Some(CheckOutput::passed()),
                    recent_release: Some(CheckOutput::passed()),
                    slack_presence: Some(CheckOutput::passed()),
                },
                security: Security {
                    binary_artifacts: Some(CheckOutput::passed()),
                    code_review: Some(CheckOutput::passed()),
                    dangerous_workflow: Some(CheckOutput::passed()),
                    dependency_update_tool: Some(CheckOutput::passed()),
                    maintained: Some(CheckOutput::passed()),
                    sbom: Some(CheckOutput::passed()),
                    security_policy: Some(CheckOutput::passed()),
                    signed_releases: Some(CheckOutput::passed()),
                    token_permissions: Some(CheckOutput::passed()),
                },
                legal: Legal {
                    trademark_disclaimer: Some(CheckOutput::passed()),
                },
            }),
            Score {
                global: 99.99999999999999,
                global_weight: 95,
                documentation: Some(100.0),
                documentation_weight: Some(30),
                license: Some(100.0),
                license_weight: Some(20),
                best_practices: Some(100.0),
                best_practices_weight: Some(20),
                security: Some(100.0),
                security_weight: Some(20),
                legal: Some(100.0),
                legal_weight: Some(5),
            }
        );
    }

    #[test]
    fn calculate_report_with_no_checks_passed_got_min_score() {
        assert_eq!(
            calculate(&Report {
                documentation: Documentation {
                    adopters: Some(CheckOutput::not_passed()),
                    code_of_conduct: Some(CheckOutput::not_passed()),
                    contributing: Some(CheckOutput::not_passed()),
                    changelog: Some(CheckOutput::not_passed()),
                    governance: Some(CheckOutput::not_passed()),
                    maintainers: Some(CheckOutput::not_passed()),
                    readme: Some(CheckOutput::not_passed()),
                    roadmap: Some(CheckOutput::not_passed()),
                    website: Some(CheckOutput::not_passed()),
                },
                license: License {
                    license_approved: Some(CheckOutput::not_passed()),
                    license_scanning: Some(CheckOutput::not_passed()),
                    license_spdx_id: Some(CheckOutput::not_passed()),
                },
                best_practices: BestPractices {
                    analytics: Some(CheckOutput::not_passed()),
                    artifacthub_badge: Some(CheckOutput::not_passed()),
                    cla: Some(CheckOutput::not_passed()),
                    community_meeting: Some(CheckOutput::not_passed()),
                    dco: Some(CheckOutput::not_passed()),
                    github_discussions: Some(CheckOutput::not_passed()),
                    openssf_badge: Some(CheckOutput::not_passed()),
                    recent_release: Some(CheckOutput::not_passed()),
                    slack_presence: Some(CheckOutput::not_passed()),
                },
                security: Security {
                    binary_artifacts: Some(CheckOutput::not_passed()),
                    code_review: Some(CheckOutput::not_passed()),
                    dangerous_workflow: Some(CheckOutput::not_passed()),
                    dependency_update_tool: Some(CheckOutput::not_passed()),
                    maintained: Some(CheckOutput::not_passed()),
                    sbom: Some(CheckOutput::not_passed()),
                    security_policy: Some(CheckOutput::not_passed()),
                    signed_releases: Some(CheckOutput::not_passed()),
                    token_permissions: Some(CheckOutput::not_passed()),
                },
                legal: Legal {
                    trademark_disclaimer: Some(CheckOutput::not_passed()),
                },
            }),
            Score {
                global: 0.0,
                global_weight: 95,
                documentation: Some(0.0),
                documentation_weight: Some(30),
                license: Some(0.0),
                license_weight: Some(20),
                best_practices: Some(0.0),
                best_practices_weight: Some(20),
                security: Some(0.0),
                security_weight: Some(20),
                legal: Some(0.0),
                legal_weight: Some(5),
            }
        );
    }

    #[test]
    fn calculate_report_with_all_checks_passed_but_some_missing_got_max_score() {
        assert_eq!(
            calculate(&Report {
                documentation: Documentation {
                    adopters: None,
                    code_of_conduct: None,
                    contributing: Some(CheckOutput::passed()),
                    changelog: Some(CheckOutput::passed()),
                    governance: None,
                    maintainers: Some(CheckOutput::passed()),
                    readme: Some(CheckOutput::passed()),
                    roadmap: None,
                    website: None,
                },
                license: License {
                    license_approved: Some(CheckOutput::passed()),
                    license_scanning: Some(
                        CheckOutput::passed().url(Some("https://license-scanning.url".to_string()))
                    ),
                    license_spdx_id: Some(
                        CheckOutput::passed().value(Some("Apache-2.0".to_string()))
                    ),
                },
                best_practices: BestPractices {
                    analytics: Some(CheckOutput::passed()),
                    artifacthub_badge: Some(CheckOutput::exempt()),
                    cla: Some(CheckOutput::passed()),
                    community_meeting: None,
                    dco: Some(CheckOutput::passed()),
                    github_discussions: Some(CheckOutput::passed()),
                    openssf_badge: Some(CheckOutput::passed()),
                    recent_release: Some(CheckOutput::passed()),
                    slack_presence: None,
                },
                security: Security {
                    binary_artifacts: Some(CheckOutput::passed()),
                    code_review: Some(CheckOutput::passed()),
                    dangerous_workflow: Some(CheckOutput::passed()),
                    dependency_update_tool: Some(CheckOutput::passed()),
                    maintained: Some(CheckOutput::passed()),
                    sbom: Some(CheckOutput::passed()),
                    security_policy: Some(CheckOutput::passed()),
                    signed_releases: Some(CheckOutput::passed()),
                    token_permissions: Some(CheckOutput::passed()),
                },
                legal: Legal {
                    trademark_disclaimer: None,
                },
            }),
            Score {
                global: 100.00000000000001,
                global_weight: 75,
                documentation: Some(100.0),
                documentation_weight: Some(18),
                license: Some(100.0),
                license_weight: Some(20),
                best_practices: Some(100.0),
                best_practices_weight: Some(17),
                security: Some(100.0),
                security_weight: Some(20),
                legal: None,
                legal_weight: None,
            }
        );
    }

    #[test]
    fn merge_scores() {
        assert_eq!(
            merge(&[
                Score {
                    global: 100.0,
                    global_weight: 90,
                    documentation: Some(100.0),
                    documentation_weight: Some(30),
                    license: Some(100.0),
                    license_weight: Some(20),
                    best_practices: Some(100.0),
                    best_practices_weight: Some(20),
                    security: Some(100.0),
                    security_weight: Some(15),
                    legal: Some(100.0),
                    legal_weight: Some(5),
                },
                Score {
                    global: 0.0,
                    global_weight: 45,
                    documentation: Some(0.0),
                    documentation_weight: Some(15),
                    license: Some(0.0),
                    license_weight: Some(10),
                    best_practices: Some(0.0),
                    best_practices_weight: Some(10),
                    security: Some(0.0),
                    security_weight: Some(10),
                    legal: None,
                    legal_weight: None,
                }
            ]),
            Score {
                global: 66.66666666666666,
                global_weight: 0,
                documentation: Some(66.66666666666666),
                documentation_weight: None,
                license: Some(66.66666666666666),
                license_weight: None,
                best_practices: Some(66.66666666666666),
                best_practices_weight: None,
                security: Some(60.0),
                security_weight: None,
                legal: Some(100.0),
                legal_weight: None,
            }
        )
    }
}
