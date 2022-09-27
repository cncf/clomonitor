use super::CheckOutput;
use serde::{Deserialize, Serialize};

/// Linter report.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Report {
    pub documentation: Documentation,
    pub license: License,
    pub best_practices: BestPractices,
    pub security: Security,
    pub legal: Legal,
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
    pub website: Option<CheckOutput>,
}

/// License section of the report.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct License {
    pub license_approved: Option<CheckOutput<bool>>,
    pub license_scanning: Option<CheckOutput>,
    pub license_spdx_id: Option<CheckOutput<String>>,
}

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
    pub recent_release: Option<CheckOutput>,
    pub slack_presence: Option<CheckOutput>,
}

/// Security section of the report.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Security {
    pub binary_artifacts: Option<CheckOutput>,
    pub code_review: Option<CheckOutput>,
    pub dangerous_workflow: Option<CheckOutput>,
    pub dependency_update_tool: Option<CheckOutput>,
    pub maintained: Option<CheckOutput>,
    pub sbom: Option<CheckOutput>,
    pub security_policy: Option<CheckOutput>,
    pub signed_releases: Option<CheckOutput>,
    pub token_permissions: Option<CheckOutput>,
}

/// Legal section of the report.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Legal {
    pub trademark_disclaimer: Option<CheckOutput>,
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
            self.best_practices.dco = Some(CheckOutput {
                exempt: true,
                exemption_reason: Some("CLA check passed".to_string()),
                ..Default::default()
            });
        }
        if passed(self.best_practices.dco.as_ref()) && !passed(self.best_practices.cla.as_ref()) {
            self.best_practices.cla = Some(CheckOutput {
                exempt: true,
                exemption_reason: Some("DCO check passed".to_string()),
                ..Default::default()
            });
        }

        // Slack presence / GitHub discussions
        if passed(self.best_practices.slack_presence.as_ref())
            && !passed(self.best_practices.github_discussions.as_ref())
        {
            self.best_practices.github_discussions = Some(CheckOutput {
                exempt: true,
                exemption_reason: Some("Slack presence check passed".to_string()),
                ..Default::default()
            });
        }
        if passed(self.best_practices.github_discussions.as_ref())
            && !passed(self.best_practices.slack_presence.as_ref())
        {
            self.best_practices.slack_presence = Some(CheckOutput {
                exempt: true,
                exemption_reason: Some("GitHub discussions check passed".to_string()),
                ..Default::default()
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_exemptions_cla_passed() {
        let mut report = Report {
            best_practices: BestPractices {
                cla: Some(true.into()),
                ..Default::default()
            },
            ..Default::default()
        };

        report.apply_exemptions();
        assert_eq!(
            report,
            Report {
                best_practices: BestPractices {
                    cla: Some(true.into()),
                    dco: Some(CheckOutput {
                        exempt: true,
                        exemption_reason: Some("CLA check passed".to_string()),
                        ..Default::default()
                    }),
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
                dco: Some(true.into()),
                ..Default::default()
            },
            ..Default::default()
        };

        report.apply_exemptions();
        assert_eq!(
            report,
            Report {
                best_practices: BestPractices {
                    dco: Some(true.into()),
                    cla: Some(CheckOutput {
                        exempt: true,
                        exemption_reason: Some("DCO check passed".to_string()),
                        ..Default::default()
                    }),
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
                slack_presence: Some(true.into()),
                ..Default::default()
            },
            ..Default::default()
        };

        report.apply_exemptions();
        assert_eq!(
            report,
            Report {
                best_practices: BestPractices {
                    slack_presence: Some(true.into()),
                    github_discussions: Some(CheckOutput {
                        exempt: true,
                        exemption_reason: Some("Slack presence check passed".to_string()),
                        ..Default::default()
                    }),
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
                github_discussions: Some(true.into()),
                ..Default::default()
            },
            ..Default::default()
        };

        report.apply_exemptions();
        assert_eq!(
            report,
            Report {
                best_practices: BestPractices {
                    github_discussions: Some(true.into()),
                    slack_presence: Some(CheckOutput {
                        exempt: true,
                        exemption_reason: Some("GitHub discussions check passed".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }
        );
    }
}
