use serde::{Deserialize, Serialize};

use crate::linter::*;

/// Number of decimals scores are rounded to.
const SCORE_DECIMALS: i32 = 2;

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_readiness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_readiness_weight: Option<usize>,
}

impl Score {
    /// Return the score's global value.
    #[must_use]
    pub fn global(&self) -> f64 {
        self.global
    }

    /// Return the score's rating (a, b, c or d).
    #[must_use]
    pub fn rating(&self) -> char {
        rating(self.global())
    }
}

/// Calculate score for the given linter report.
#[must_use]
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
    (score.agent_readiness, score.agent_readiness_weight) = calculate_section(
        &report.agent_readiness.available(),
        &report.agent_readiness.passed_or_exempt(),
    );

    // Global. The agent readiness section is advisory: it gets its own score
    // but does not move the global score or rating.
    let global_sections = [
        (score.documentation, score.documentation_weight),
        (score.license, score.license_weight),
        (score.best_practices, score.best_practices_weight),
        (score.security, score.security_weight),
        (score.legal, score.legal_weight),
    ];
    score.global_weight = global_sections
        .iter()
        .fold(0, |gw, (_, sw)| gw + sw.unwrap_or_default());
    score.global = if score.global_weight == 0 {
        0.0
    } else {
        let weighted_sum = global_sections.iter().fold(0.0, |sum, (ss, sw)| {
            sum + ss.unwrap_or_default() * sw.unwrap_or_default() as f64
        });
        round(weighted_sum / score.global_weight as f64)
    };

    score
}

/// Calculate score and weight for a report's section from the checks provided.
fn calculate_section(
    checks_available: &[CheckId],
    checks_passed_or_exempt: &[CheckId],
) -> (Option<f64>, Option<usize>) {
    // Calculate section weight
    let weight: usize = checks_available
        .iter()
        .map(|check_id| CHECKS[check_id].weight)
        .sum();
    if weight == 0 {
        return (None, None);
    }

    // Calculate section score
    let weight_passed_or_exempt: usize = checks_passed_or_exempt
        .iter()
        .map(|check_id| CHECKS[check_id].weight)
        .sum();
    let score = round(weight_passed_or_exempt as f64 / weight as f64 * 100.0);

    (Some(score), Some(weight))
}

/// Merge the scores provided into a single score.
#[must_use]
pub fn merge(scores: &[Score]) -> Score {
    // Sum all scores weights for each of the sections. We'll use them to
    // calculate the coefficient we'll apply to each of the scores.
    let mut global_weights_sum = 0;
    let mut documentation_weights_sum = 0;
    let mut license_weights_sum = 0;
    let mut best_practices_weights_sum = 0;
    let mut security_weights_sum = 0;
    let mut legal_weights_sum = 0;
    let mut agent_readiness_weights_sum = 0;
    for score in scores {
        global_weights_sum += score.global_weight;
        documentation_weights_sum += score.documentation_weight.unwrap_or_default();
        license_weights_sum += score.license_weight.unwrap_or_default();
        best_practices_weights_sum += score.best_practices_weight.unwrap_or_default();
        security_weights_sum += score.security_weight.unwrap_or_default();
        legal_weights_sum += score.legal_weight.unwrap_or_default();
        agent_readiness_weights_sum += score.agent_readiness_weight.unwrap_or_default();
    }

    // Helper function that adds a score to the weighted sum provided after
    // applying its weight. Scores not available are skipped.
    let add =
        |weighted_sum: Option<f64>, score: Option<f64>, weight: Option<usize>| -> Option<f64> {
            match score {
                Some(v) => {
                    Some(weighted_sum.unwrap_or_default() + v * weight.unwrap_or_default() as f64)
                }
                None => weighted_sum,
            }
        };

    // Helper function that finishes a merged score from its weighted sum and
    // the sum of weights. Sections without any weight are skipped to avoid
    // dividing by zero.
    let finish = |weighted_sum: Option<f64>, weights_sum: usize| -> Option<f64> {
        if weights_sum == 0 {
            return None;
        }
        weighted_sum.map(|s| round(s / weights_sum as f64))
    };

    // Calculate merged score for each of the sections.
    let mut m = Score::default();
    for s in scores {
        m.global += s.global * s.global_weight as f64;
        m.documentation = add(m.documentation, s.documentation, s.documentation_weight);
        m.license = add(m.license, s.license, s.license_weight);
        m.best_practices = add(m.best_practices, s.best_practices, s.best_practices_weight);
        m.security = add(m.security, s.security, s.security_weight);
        m.legal = add(m.legal, s.legal, s.legal_weight);
        m.agent_readiness = add(
            m.agent_readiness,
            s.agent_readiness,
            s.agent_readiness_weight,
        );
    }
    m.global = if global_weights_sum == 0 {
        0.0
    } else {
        round(m.global / global_weights_sum as f64)
    };
    m.documentation = finish(m.documentation, documentation_weights_sum);
    m.license = finish(m.license, license_weights_sum);
    m.best_practices = finish(m.best_practices, best_practices_weights_sum);
    m.security = finish(m.security, security_weights_sum);
    m.legal = finish(m.legal, legal_weights_sum);
    m.agent_readiness = finish(m.agent_readiness, agent_readiness_weights_sum);

    m
}

/// Round the value provided to the number of decimals used for scores.
fn round(value: f64) -> f64 {
    let factor = 10_f64.powi(SCORE_DECIMALS);
    (value * factor).round() / factor
}

/// Return the score's rating (a, b, c or d).
#[must_use]
pub fn rating(score: f64) -> char {
    match score.round() as usize {
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
        assert!(
            (Score {
                global: 10.0,
                ..Score::default()
            }
            .global()
                - 10.0)
                .abs()
                < f64::EPSILON
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
                    summary_table: Some(CheckOutput::passed()),
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
                    openssf_scorecard_badge: Some(CheckOutput::passed()),
                    recent_release: Some(CheckOutput::passed()),
                    slack_presence: Some(CheckOutput::passed()),
                },
                security: Security {
                    binary_artifacts: Some(CheckOutput::passed()),
                    code_review: Some(CheckOutput::passed()),
                    dangerous_workflow: Some(CheckOutput::passed()),
                    dependencies_policy: Some(CheckOutput::passed()),
                    dependency_update_tool: Some(CheckOutput::passed()),
                    maintained: Some(CheckOutput::passed()),
                    sbom: Some(CheckOutput::passed()),
                    security_insights: Some(CheckOutput::passed()),
                    security_policy: Some(CheckOutput::passed()),
                    signed_releases: Some(CheckOutput::passed()),
                    token_permissions: Some(CheckOutput::passed()),
                },
                legal: Legal {
                    trademark_disclaimer: Some(CheckOutput::passed()),
                },
                agent_readiness: AgentReadiness {
                    authentication: Some(CheckOutput::passed()),
                    content_discoverability: Some(CheckOutput::passed()),
                    content_structure: Some(CheckOutput::passed()),
                    markdown_availability: Some(CheckOutput::passed()),
                    observability: Some(CheckOutput::passed()),
                    page_size: Some(CheckOutput::passed()),
                    url_stability: Some(CheckOutput::passed()),
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
                best_practices_weight: Some(19),
                security: Some(100.0),
                security_weight: Some(22),
                legal: Some(100.0),
                legal_weight: Some(5),
                agent_readiness: Some(100.0),
                agent_readiness_weight: Some(49),
            }
        );
    }

    #[test]
    fn calculate_report_with_all_checks_non_passed_got_min_score() {
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
                    summary_table: Some(CheckOutput::not_passed()),
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
                    openssf_scorecard_badge: Some(CheckOutput::not_passed()),
                    recent_release: Some(CheckOutput::not_passed()),
                    slack_presence: Some(CheckOutput::not_passed()),
                },
                security: Security {
                    binary_artifacts: Some(CheckOutput::not_passed()),
                    code_review: Some(CheckOutput::not_passed()),
                    dangerous_workflow: Some(CheckOutput::not_passed()),
                    dependencies_policy: Some(CheckOutput::not_passed()),
                    dependency_update_tool: Some(CheckOutput::not_passed()),
                    maintained: Some(CheckOutput::not_passed()),
                    sbom: Some(CheckOutput::not_passed()),
                    security_insights: Some(CheckOutput::not_passed()),
                    security_policy: Some(CheckOutput::not_passed()),
                    signed_releases: Some(CheckOutput::not_passed()),
                    token_permissions: Some(CheckOutput::not_passed()),
                },
                legal: Legal {
                    trademark_disclaimer: Some(CheckOutput::not_passed()),
                },
                agent_readiness: AgentReadiness {
                    authentication: Some(CheckOutput::not_passed()),
                    content_discoverability: Some(CheckOutput::not_passed()),
                    content_structure: Some(CheckOutput::not_passed()),
                    markdown_availability: Some(CheckOutput::not_passed()),
                    observability: Some(CheckOutput::not_passed()),
                    page_size: Some(CheckOutput::not_passed()),
                    url_stability: Some(CheckOutput::not_passed()),
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
                best_practices_weight: Some(19),
                security: Some(0.0),
                security_weight: Some(22),
                legal: Some(0.0),
                legal_weight: Some(5),
                agent_readiness: Some(0.0),
                agent_readiness_weight: Some(49),
            }
        );
    }

    #[test]
    fn calculate_report_with_some_missing_checks_but_all_passed_got_max_score() {
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
                    summary_table: None,
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
                    openssf_scorecard_badge: Some(CheckOutput::passed()),
                    recent_release: Some(CheckOutput::passed()),
                    slack_presence: None,
                },
                security: Security {
                    binary_artifacts: Some(CheckOutput::passed()),
                    code_review: Some(CheckOutput::passed()),
                    dangerous_workflow: Some(CheckOutput::passed()),
                    dependencies_policy: Some(CheckOutput::passed()),
                    dependency_update_tool: Some(CheckOutput::passed()),
                    maintained: Some(CheckOutput::passed()),
                    sbom: Some(CheckOutput::passed()),
                    security_policy: Some(CheckOutput::passed()),
                    security_insights: Some(CheckOutput::passed()),
                    signed_releases: Some(CheckOutput::passed()),
                    token_permissions: Some(CheckOutput::passed()),
                },
                legal: Legal {
                    trademark_disclaimer: None,
                },
                agent_readiness: AgentReadiness::default(),
            }),
            Score {
                global: 100.0,
                global_weight: 76,
                documentation: Some(100.0),
                documentation_weight: Some(18),
                license: Some(100.0),
                license_weight: Some(20),
                best_practices: Some(100.0),
                best_practices_weight: Some(16),
                security: Some(100.0),
                security_weight: Some(22),
                legal: None,
                legal_weight: None,
                agent_readiness: None,
                agent_readiness_weight: None,
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
                    agent_readiness: Some(100.0),
                    agent_readiness_weight: Some(49),
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
                    agent_readiness: None,
                    agent_readiness_weight: None,
                }
            ],),
            Score {
                global: 66.67,
                global_weight: 0,
                documentation: Some(66.67),
                documentation_weight: None,
                license: Some(66.67),
                license_weight: None,
                best_practices: Some(66.67),
                best_practices_weight: None,
                security: Some(60.0),
                security_weight: None,
                legal: Some(100.0),
                legal_weight: None,
                agent_readiness: Some(100.0),
                agent_readiness_weight: None,
            }
        );
    }

    /// Community report used to verify the global score is not affected by the
    /// agent readiness section.
    fn community_report(agent_readiness: AgentReadiness) -> Report {
        Report {
            documentation: Documentation {
                adopters: Some(CheckOutput::passed()),
                code_of_conduct: Some(CheckOutput::not_passed()),
                contributing: Some(CheckOutput::passed()),
                changelog: None,
                governance: Some(CheckOutput::passed()),
                maintainers: Some(CheckOutput::not_passed()),
                readme: Some(CheckOutput::passed()),
                roadmap: Some(CheckOutput::not_passed()),
                summary_table: Some(CheckOutput::passed()),
                website: Some(CheckOutput::passed()),
            },
            license: License::default(),
            best_practices: BestPractices {
                analytics: Some(CheckOutput::passed()),
                artifacthub_badge: Some(CheckOutput::not_passed()),
                cla: Some(CheckOutput::passed()),
                community_meeting: Some(CheckOutput::not_passed()),
                dco: Some(CheckOutput::exempt()),
                github_discussions: Some(CheckOutput::passed()),
                openssf_badge: Some(CheckOutput::not_passed()),
                openssf_scorecard_badge: Some(CheckOutput::passed()),
                recent_release: None,
                slack_presence: Some(CheckOutput::passed()),
            },
            security: Security::default(),
            legal: Legal {
                trademark_disclaimer: Some(CheckOutput::not_passed()),
            },
            agent_readiness,
        }
    }

    #[test]
    fn calculate_global_is_bit_identical_with_and_without_agent_readiness() {
        let without = calculate(&community_report(AgentReadiness::default()));
        let with = calculate(&community_report(AgentReadiness {
            authentication: Some(CheckOutput::passed()),
            content_discoverability: Some(CheckOutput::not_passed()),
            content_structure: Some(CheckOutput::passed()),
            markdown_availability: Some(CheckOutput::not_passed()),
            observability: Some(CheckOutput::failed()),
            page_size: Some(CheckOutput::passed()),
            url_stability: Some(CheckOutput::exempt()),
        }));

        assert_eq!(without.agent_readiness, None);
        assert_eq!(without.agent_readiness_weight, None);
        assert_eq!(with.agent_readiness_weight, Some(49));
        // authentication (10) + content_structure (4) + page_size (10) + url_stability (4) = 28/49
        assert_eq!(with.agent_readiness, Some(57.14));
        assert_eq!(with.global.to_bits(), without.global.to_bits());
        assert_eq!(with.global_weight, without.global_weight);
        assert_eq!(with.rating(), without.rating());
        assert_eq!(
            Score {
                agent_readiness: None,
                agent_readiness_weight: None,
                ..with
            },
            without
        );
    }

    #[test]
    fn calculate_advisory_only_report_is_finite() {
        let score = calculate(&Report {
            agent_readiness: AgentReadiness {
                authentication: Some(CheckOutput::passed()),
                ..AgentReadiness::default()
            },
            ..Report::default()
        });

        assert!(score.global.is_finite());
        assert!(score.global.abs() < f64::EPSILON);
        assert_eq!(score.global_weight, 0);
        assert_eq!(score.agent_readiness, Some(100.0));
        assert_eq!(score.agent_readiness_weight, Some(10));
        assert_eq!(score.rating(), 'd');
    }

    #[test]
    fn calculate_empty_report_is_finite() {
        let score = calculate(&Report::default());
        assert!(score.global.is_finite());
        assert_eq!(score, Score::default());
    }

    #[test]
    fn merge_mixed_old_and_new_scores() {
        let merged = merge(&[
            // Legacy score without the agent readiness section
            Score {
                global: 80.0,
                global_weight: 50,
                documentation: Some(80.0),
                documentation_weight: Some(50),
                ..Score::default()
            },
            // New score with the agent readiness section
            Score {
                global: 40.0,
                global_weight: 50,
                documentation: Some(40.0),
                documentation_weight: Some(50),
                agent_readiness: Some(30.0),
                agent_readiness_weight: Some(49),
                ..Score::default()
            },
        ]);

        assert!((merged.global - 60.0).abs() < f64::EPSILON);
        assert_eq!(merged.documentation, Some(60.0));
        assert_eq!(merged.agent_readiness, Some(30.0));
        assert_eq!(merged.license, None);
        assert!(merged.global.is_finite());
    }

    #[test]
    fn merge_scores_without_weights_is_finite() {
        let merged = merge(&[Score::default(), Score::default()]);
        assert!(merged.global.is_finite());
        assert_eq!(merged, Score::default());

        let merged = merge(&[Score {
            agent_readiness: Some(50.0),
            agent_readiness_weight: Some(10),
            ..Score::default()
        }]);
        assert!(merged.global.is_finite());
        assert!(merged.global.abs() < f64::EPSILON);
        assert_eq!(merged.agent_readiness, Some(50.0));
    }
}
