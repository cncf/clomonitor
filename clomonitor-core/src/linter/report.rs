use serde::{Deserialize, Serialize};

use super::{CheckOutput, check::CheckId, checks::*};

/// Linter report.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Report {
    pub documentation: Documentation,
    pub license: License,
    pub best_practices: BestPractices,
    pub security: Security,
    pub legal: Legal,
}

impl Report {
    /// Apply inter-checks exemptions.
    pub(crate) fn apply_exemptions(&mut self) {
        let passed = |o: Option<&CheckOutput>| -> bool {
            match o {
                Some(o) => o.passed || o.exempt,
                None => false,
            }
        };

        // CLA / DCO
        if passed(self.best_practices.cla.as_ref()) && !passed(self.best_practices.dco.as_ref()) {
            self.best_practices.dco =
                Some(CheckOutput::exempt().exemption_reason(Some("CLA check passed".to_string())));
        }
        if passed(self.best_practices.dco.as_ref()) && !passed(self.best_practices.cla.as_ref()) {
            self.best_practices.cla =
                Some(CheckOutput::exempt().exemption_reason(Some("DCO check passed".to_string())));
        }

        // Slack presence / GitHub discussions
        if passed(self.best_practices.slack_presence.as_ref())
            && !passed(self.best_practices.github_discussions.as_ref())
        {
            self.best_practices.github_discussions = Some(
                CheckOutput::exempt()
                    .exemption_reason(Some("Slack presence check passed".to_string())),
            );
        }
        if passed(self.best_practices.github_discussions.as_ref())
            && !passed(self.best_practices.slack_presence.as_ref())
        {
            self.best_practices.slack_presence = Some(
                CheckOutput::exempt()
                    .exemption_reason(Some("Github discussions check passed".to_string())),
            );
        }
    }
}

/// Documentation section of the report.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Documentation {
    pub adopters: Option<CheckOutput>,
    pub changelog: Option<CheckOutput>,
    pub code_of_conduct: Option<CheckOutput>,
    pub contributing: Option<CheckOutput>,
    pub governance: Option<CheckOutput>,
    pub maintainers: Option<CheckOutput>,
    pub readme: Option<CheckOutput>,
    pub roadmap: Option<CheckOutput>,
    pub summary_table: Option<CheckOutput>,
    pub website: Option<CheckOutput>,
}

#[rustfmt::skip]
section_impl!(
    Documentation,
    adopters,
    changelog,
    code_of_conduct,
    contributing,
    governance,
    maintainers,
    readme,
    roadmap,
    summary_table,
    website
);

/// License section of the report.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct License {
    pub license_approved: Option<CheckOutput>,
    pub license_scanning: Option<CheckOutput>,
    pub license_spdx_id: Option<CheckOutput<String>>,
}

#[rustfmt::skip]
section_impl!(
    License,
    license_approved,
    license_scanning,
    license_spdx_id
);

/// BestPractices section of the report.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BestPractices {
    pub analytics: Option<CheckOutput<Vec<String>>>,
    pub artifacthub_badge: Option<CheckOutput>,
    pub cla: Option<CheckOutput>,
    pub community_meeting: Option<CheckOutput>,
    pub dco: Option<CheckOutput>,
    pub github_discussions: Option<CheckOutput>,
    pub openssf_badge: Option<CheckOutput>,
    pub openssf_scorecard_badge: Option<CheckOutput>,
    pub recent_release: Option<CheckOutput>,
    pub slack_presence: Option<CheckOutput>,
}

#[rustfmt::skip]
section_impl!(
    BestPractices,
    analytics,
    artifacthub_badge,
    cla,
    community_meeting,
    dco,
    github_discussions,
    openssf_badge,
    openssf_scorecard_badge,
    recent_release,
    slack_presence
);

/// Security section of the report.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Security {
    pub binary_artifacts: Option<CheckOutput>,
    pub code_review: Option<CheckOutput>,
    pub dangerous_workflow: Option<CheckOutput>,
    pub dependencies_policy: Option<CheckOutput>,
    pub dependency_update_tool: Option<CheckOutput>,
    pub maintained: Option<CheckOutput>,
    pub sbom: Option<CheckOutput>,
    pub security_insights: Option<CheckOutput>,
    pub security_policy: Option<CheckOutput>,
    pub signed_releases: Option<CheckOutput>,
    pub token_permissions: Option<CheckOutput>,
}

#[rustfmt::skip]
section_impl!(
    Security,
    binary_artifacts,
    code_review,
    dangerous_workflow,
    dependencies_policy,
    dependency_update_tool,
    maintained,
    sbom,
    security_insights,
    security_policy,
    signed_releases,
    token_permissions
);

/// Legal section of the report.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Legal {
    pub trademark_disclaimer: Option<CheckOutput>,
}

#[rustfmt::skip]
section_impl!(
    Legal,
    trademark_disclaimer
);

/// Prepare the implementation for a section in the report.
macro_rules! section_impl {
    ( $section:ident, $( $check:ident ),* ) => {
        impl $section {
            pub(crate) fn available(&self) -> Vec<CheckId> {
                let mut checks = Vec::new();
                $(
                if self.$check.as_ref().is_some() {
                    checks.push($check::ID);
                }
                )*
                checks
            }

            pub(crate) fn passed_or_exempt(&self) -> Vec<CheckId> {
                let mut checks = Vec::new();
                $(
                if self.$check.as_ref().map_or(false, |o| o.passed || o.exempt) {
                    checks.push($check::ID);
                }
                )*
                checks
            }
        }
    };
}
use section_impl;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_exemptions_cla_passed() {
        let mut report = Report {
            best_practices: BestPractices {
                cla: Some(CheckOutput::passed()),
                ..Default::default()
            },
            ..Default::default()
        };

        report.apply_exemptions();
        assert_eq!(
            report,
            Report {
                best_practices: BestPractices {
                    cla: Some(CheckOutput::passed()),
                    dco: Some(
                        CheckOutput::exempt()
                            .exemption_reason(Some("CLA check passed".to_string()))
                    ),
                    ..Default::default()
                },
                ..Default::default()
            }
        );
    }

    #[test]
    fn apply_exemptions_dco_passed() {
        let mut report = Report {
            best_practices: BestPractices {
                dco: Some(CheckOutput::passed()),
                ..Default::default()
            },
            ..Default::default()
        };

        report.apply_exemptions();
        assert_eq!(
            report,
            Report {
                best_practices: BestPractices {
                    dco: Some(CheckOutput::passed()),
                    cla: Some(
                        CheckOutput::exempt()
                            .exemption_reason(Some("DCO check passed".to_string()))
                    ),
                    ..Default::default()
                },
                ..Default::default()
            }
        );
    }

    #[test]
    fn apply_exemptions_slack_presence_passed() {
        let mut report = Report {
            best_practices: BestPractices {
                slack_presence: Some(CheckOutput::passed()),
                ..Default::default()
            },
            ..Default::default()
        };

        report.apply_exemptions();
        assert_eq!(
            report,
            Report {
                best_practices: BestPractices {
                    slack_presence: Some(CheckOutput::passed()),
                    github_discussions: Some(
                        CheckOutput::exempt()
                            .exemption_reason(Some("Slack presence check passed".to_string()))
                    ),
                    ..Default::default()
                },
                ..Default::default()
            }
        );
    }

    #[test]
    fn apply_exemptions_github_discussions_passed() {
        let mut report = Report {
            best_practices: BestPractices {
                github_discussions: Some(CheckOutput::passed()),
                ..Default::default()
            },
            ..Default::default()
        };

        report.apply_exemptions();
        assert_eq!(
            report,
            Report {
                best_practices: BestPractices {
                    github_discussions: Some(CheckOutput::passed()),
                    slack_presence: Some(
                        CheckOutput::exempt()
                            .exemption_reason(Some("Github discussions check passed".to_string()))
                    ),
                    ..Default::default()
                },
                ..Default::default()
            }
        );
    }
}
