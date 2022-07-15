use crate::{config::*, linter::CheckOutput, linter::Report};
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

    // Documentation
    let d = &report.documentation;
    (score.documentation, score.documentation_weight) = calculate_section_score_and_weight(&[
        (ADOPTERS, should_score(&d.adopters)),
        (CHANGELOG, should_score(&d.changelog)),
        (CODE_OF_CONDUCT, should_score(&d.code_of_conduct)),
        (CONTRIBUTING, should_score(&d.contributing)),
        (GOVERNANCE, should_score(&d.governance)),
        (MAINTAINERS, should_score(&d.maintainers)),
        (README, should_score(&d.readme)),
        (ROADMAP, should_score(&d.roadmap)),
        (WEBSITE, should_score(&d.website)),
    ]);

    // License
    (score.license, score.license_weight) = calculate_section_score_and_weight(&[
        (
            LICENSE_APPROVED,
            should_score(&report.license.license_approved),
        ),
        (
            LICENSE_SCANNING,
            should_score(&report.license.license_scanning),
        ),
        (LICENSE_SPDX, should_score(&report.license.license_spdx_id)),
    ]);

    // Best practices
    let bp = &report.best_practices;
    (score.best_practices, score.best_practices_weight) = calculate_section_score_and_weight(&[
        (ARTIFACTHUB_BADGE, should_score(&bp.artifacthub_badge)),
        (CLA, should_score(&bp.cla)),
        (COMMUNITY_MEETING, should_score(&bp.community_meeting)),
        (DCO, should_score(&bp.dco)),
        (GA4, should_score(&bp.ga4)),
        (GITHUB_DISCUSSIONS, should_score(&bp.github_discussions)),
        (OPENSSF_BADGE, should_score(&bp.openssf_badge)),
        (RECENT_RELEASE, should_score(&bp.recent_release)),
        (SLACK_PRESENCE, should_score(&bp.slack_presence)),
    ]);

    // Security
    let s = &report.security;
    (score.security, score.security_weight) = calculate_section_score_and_weight(&[
        (BINARY_ARTIFACTS, should_score(&s.binary_artifacts)),
        (BRANCH_PROTECTION, should_score(&s.branch_protection)),
        (CODE_REVIEW, should_score(&s.code_review)),
        (DANGEROUS_WORKFLOW, should_score(&s.dangerous_workflow)),
        (
            DEPENDENCY_UPDATE_TOOL,
            should_score(&s.dependency_update_tool),
        ),
        (MAINTAINED, should_score(&s.maintained)),
        (SBOM, should_score(&s.sbom)),
        (SECURITY_POLICY, should_score(&s.security_policy)),
        (SIGNED_RELEASES, should_score(&s.signed_releases)),
        (TOKEN_PERMISSIONS, should_score(&s.token_permissions)),
        (VULNERABILITIES, should_score(&s.vulnerabilities)),
    ]);

    // Legal
    (score.legal, score.legal_weight) = calculate_section_score_and_weight(&[(
        TRADEMARK_DISCLAIMER,
        should_score(&report.legal.trademark_disclaimer),
    )]);

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
    // applying the given coefficient to it
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

/// Calculate score for a report's section from the checks provided.
fn calculate_section_score_and_weight(
    checks: &[(&'static str, Option<bool>)],
) -> (Option<f64>, Option<usize>) {
    // Calculate section weight
    let mut section_weight = 0;
    for (check_id, should_score) in checks {
        if should_score.is_some() {
            section_weight += CHECK_WEIGHT[check_id];
        }
    }

    // None of the checks were provided
    if section_weight == 0 {
        return (None, None);
    }

    // Calculate section score
    let mut score = 0.0;
    for (check_id, should_score) in checks {
        if let Some(should_score) = should_score {
            if *should_score {
                score += CHECK_WEIGHT[check_id] as f64 / section_weight as f64 * 100.0;
            }
        }
    }

    (Some(score), Some(section_weight))
}

/// Helper that checks if the provided check should be scored or not. At the
/// moment a check gets a score if it has passed or is exempt.
fn should_score<T>(output: &Option<CheckOutput<T>>) -> Option<bool> {
    output.as_ref().map(|o| o.passed || o.exempt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linter::*;

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
                    adopters: Some(true.into()),
                    code_of_conduct: Some(true.into()),
                    contributing: Some(true.into()),
                    changelog: Some(true.into()),
                    governance: Some(true.into()),
                    maintainers: Some(true.into()),
                    readme: Some(true.into()),
                    roadmap: Some(true.into()),
                    website: Some(true.into()),
                },
                license: License {
                    license_approved: Some(CheckOutput {
                        passed: true,
                        value: Some(true),
                        ..Default::default()
                    }),
                    license_scanning: Some(CheckOutput::from_url(Some(
                        "https://license-scanning.url".to_string()
                    ))),
                    license_spdx_id: Some(Some("Apache-2.0".to_string()).into()),
                },
                best_practices: BestPractices {
                    artifacthub_badge: Some(CheckOutput {
                        exempt: true,
                        ..Default::default()
                    }),
                    cla: Some(true.into()),
                    community_meeting: Some(true.into()),
                    dco: Some(true.into()),
                    ga4: Some(true.into()),
                    github_discussions: Some(true.into()),
                    openssf_badge: Some(true.into()),
                    recent_release: Some(true.into()),
                    slack_presence: Some(true.into()),
                },
                security: Security {
                    binary_artifacts: Some(true.into()),
                    branch_protection: Some(true.into()),
                    code_review: Some(true.into()),
                    dangerous_workflow: Some(true.into()),
                    dependency_update_tool: Some(true.into()),
                    maintained: Some(true.into()),
                    sbom: Some(true.into()),
                    security_policy: Some(true.into()),
                    signed_releases: Some(true.into()),
                    token_permissions: Some(true.into()),
                    vulnerabilities: Some(true.into()),
                },
                legal: Legal {
                    trademark_disclaimer: Some(true.into()),
                },
            }),
            Score {
                global: 100.0,
                global_weight: 96,
                documentation: Some(100.0),
                documentation_weight: Some(30),
                license: Some(100.0),
                license_weight: Some(20),
                best_practices: Some(100.0),
                best_practices_weight: Some(20),
                security: Some(99.99999999999999),
                security_weight: Some(21),
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
                    adopters: Some(false.into()),
                    code_of_conduct: Some(false.into()),
                    contributing: Some(false.into()),
                    changelog: Some(false.into()),
                    governance: Some(false.into()),
                    maintainers: Some(false.into()),
                    readme: Some(false.into()),
                    roadmap: Some(false.into()),
                    website: Some(false.into()),
                },
                license: License {
                    license_approved: Some(false.into()),
                    license_scanning: Some(false.into()),
                    license_spdx_id: Some(false.into()),
                },
                best_practices: BestPractices {
                    artifacthub_badge: Some(CheckOutput {
                        exempt: false,
                        ..Default::default()
                    }),
                    cla: Some(false.into()),
                    community_meeting: Some(false.into()),
                    dco: Some(false.into()),
                    ga4: Some(false.into()),
                    github_discussions: Some(false.into()),
                    openssf_badge: Some(false.into()),
                    recent_release: Some(false.into()),
                    slack_presence: Some(false.into()),
                },
                security: Security {
                    binary_artifacts: Some(false.into()),
                    branch_protection: Some(false.into()),
                    code_review: Some(false.into()),
                    dangerous_workflow: Some(false.into()),
                    dependency_update_tool: Some(false.into()),
                    maintained: Some(false.into()),
                    sbom: Some(false.into()),
                    security_policy: Some(false.into()),
                    signed_releases: Some(false.into()),
                    token_permissions: Some(false.into()),
                    vulnerabilities: Some(false.into()),
                },
                legal: Legal {
                    trademark_disclaimer: Some(false.into()),
                },
            }),
            Score {
                global: 0.0,
                global_weight: 96,
                documentation: Some(0.0),
                documentation_weight: Some(30),
                license: Some(0.0),
                license_weight: Some(20),
                best_practices: Some(0.0),
                best_practices_weight: Some(20),
                security: Some(0.0),
                security_weight: Some(21),
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
                    contributing: Some(true.into()),
                    changelog: Some(true.into()),
                    governance: None,
                    maintainers: Some(true.into()),
                    readme: Some(true.into()),
                    roadmap: None,
                    website: None,
                },
                license: License {
                    license_approved: Some(CheckOutput {
                        passed: true,
                        value: Some(true),
                        ..Default::default()
                    }),
                    license_scanning: Some(CheckOutput::from_url(Some(
                        "https://license-scanning.url".to_string()
                    ))),
                    license_spdx_id: Some(Some("Apache-2.0".to_string()).into()),
                },
                best_practices: BestPractices {
                    artifacthub_badge: Some(CheckOutput {
                        exempt: true,
                        ..Default::default()
                    }),
                    cla: Some(true.into()),
                    community_meeting: None,
                    dco: Some(true.into()),
                    ga4: Some(true.into()),
                    github_discussions: Some(true.into()),
                    openssf_badge: Some(true.into()),
                    recent_release: Some(true.into()),
                    slack_presence: None,
                },
                security: Security {
                    binary_artifacts: Some(true.into()),
                    branch_protection: Some(true.into()),
                    code_review: Some(true.into()),
                    dangerous_workflow: Some(true.into()),
                    dependency_update_tool: Some(true.into()),
                    maintained: Some(true.into()),
                    sbom: Some(true.into()),
                    security_policy: Some(true.into()),
                    signed_releases: Some(true.into()),
                    token_permissions: Some(true.into()),
                    vulnerabilities: Some(true.into()),
                },
                legal: Legal {
                    trademark_disclaimer: None,
                },
            }),
            Score {
                global: 100.0,
                global_weight: 76,
                documentation: Some(100.0),
                documentation_weight: Some(18),
                license: Some(100.0),
                license_weight: Some(20),
                best_practices: Some(100.0),
                best_practices_weight: Some(17),
                security: Some(99.99999999999999),
                security_weight: Some(21),
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
