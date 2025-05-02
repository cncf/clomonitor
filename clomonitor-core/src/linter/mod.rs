use std::{fmt, path::PathBuf, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use clap::ValueEnum;
#[cfg(feature = "mocks")]
use mockall::automock;
use postgres_types::ToSql;
use serde::{Deserialize, Serialize};
use time::Date;

use self::{
    check::*,
    checks::util::helpers::{find_exemption, should_skip_check},
};

mod check;
mod checks;
mod metadata;
mod report;

pub use self::{
    check::{CheckId, CheckOutput},
    report::*,
};
pub use checks::datasource::github::setup_http_client as setup_github_http_client;
pub(crate) use checks::*;

/// Type alias to represent a Linter trait object.
pub type DynLinter = Arc<dyn Linter + Send + Sync>;

/// Trait that defines some operations a Linter implementation must support.
#[async_trait]
#[cfg_attr(feature = "mocks", automock)]
pub trait Linter {
    /// Lint the repository provided returning a report with the results.
    async fn lint(&self, input: &LinterInput) -> Result<Report>;
}

/// Input used by the linter to perform its operations.
#[derive(Debug, Clone, Default)]
pub struct LinterInput {
    pub project: Option<Project>,
    pub root: PathBuf,
    pub url: String,
    pub check_sets: Vec<CheckSet>,
    pub github_token: String,
}

/// Project's details
#[derive(Debug, Clone, Default)]
pub struct Project {
    pub name: String,
    pub accepted_at: Option<Date>,
    pub maturity: Option<String>,
    pub foundation: Foundation,
}

/// Foundation's details
#[derive(Debug, Clone, Default)]
pub struct Foundation {
    pub foundation_id: String,
    pub landscape_url: Option<String>,
}

/// Check sets define a set of checks that will be run on a given repository.
/// Multiple check sets can be assigned to a repository.
#[derive(Debug, Clone, PartialEq, Eq, Hash, ValueEnum, Serialize, Deserialize, ToSql)]
#[serde(rename_all = "kebab-case")]
#[postgres(name = "check_set")]
pub enum CheckSet {
    #[postgres(name = "code")]
    Code,
    #[postgres(name = "code-lite")]
    CodeLite,
    #[postgres(name = "community")]
    Community,
    #[postgres(name = "docs")]
    Docs,
}

impl fmt::Display for CheckSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Self::Code => "CODE",
            Self::CodeLite => "CODE-LITE",
            Self::Community => "COMMUNITY",
            Self::Docs => "DOCS",
        };
        write!(f, "{output}")
    }
}

/// CLOMonitor core linter (Linter implementation).
pub struct CoreLinter;

#[allow(clippy::new_without_default)]
impl CoreLinter {
    /// Create a new CoreLinter instance.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Linter for CoreLinter {
    async fn lint(&self, li: &LinterInput) -> Result<Report> {
        // Prepare check input
        let ci = CheckInput::new(li).await?;

        // Run some async checks concurrently
        let (contributing, summary_table, trademark_disclaimer) = tokio::join!(
            run_async!(contributing, &ci),
            run_async!(summary_table, &ci),
            run_async!(trademark_disclaimer, &ci),
        );

        // Run some sync checks needed in advance
        let spdx_id = run!(license_spdx_id, &ci);
        let mut spdx_id_value: Option<String> = None;
        if let Some(r) = &spdx_id {
            spdx_id_value.clone_from(&r.value);
        }

        // Run the remaining sync checks and build report
        let mut report = Report {
            documentation: Documentation {
                adopters: run!(adopters, &ci),
                changelog: run!(changelog, &ci),
                code_of_conduct: run!(code_of_conduct, &ci),
                contributing,
                governance: run!(governance, &ci),
                maintainers: run!(maintainers, &ci),
                readme: run!(readme, &ci),
                roadmap: run!(roadmap, &ci),
                summary_table,
                website: run!(website, &ci),
            },
            license: License {
                license_approved: license_approved::check(&ci, spdx_id_value),
                license_scanning: run!(license_scanning, &ci),
                license_spdx_id: spdx_id,
            },
            best_practices: BestPractices {
                artifacthub_badge: run!(artifacthub_badge, &ci),
                cla: run!(cla, &ci),
                community_meeting: run!(community_meeting, &ci),
                dco: run!(dco, &ci),
                github_discussions: run!(github_discussions, &ci),
                openssf_badge: run!(openssf_badge, &ci),
                openssf_scorecard_badge: run!(openssf_scorecard_badge, &ci),
                recent_release: run!(recent_release, &ci),
                slack_presence: run!(slack_presence, &ci),
            },
            security: Security {
                binary_artifacts: run!(binary_artifacts, &ci),
                code_review: run!(code_review, &ci),
                dangerous_workflow: run!(dangerous_workflow, &ci),
                dependencies_policy: run!(dependencies_policy, &ci),
                dependency_update_tool: run!(dependency_update_tool, &ci),
                maintained: run!(maintained, &ci),
                sbom: run!(sbom, &ci),
                security_insights: run!(security_insights, &ci),
                security_policy: run!(security_policy, &ci),
                signed_releases: run!(signed_releases, &ci),
                token_permissions: run!(token_permissions, &ci),
            },
            legal: Legal {
                trademark_disclaimer,
            },
        };
        report.apply_exemptions();

        Ok(report)
    }
}
