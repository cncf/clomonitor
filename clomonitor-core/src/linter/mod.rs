use self::check::scorecard::scorecard;
use super::config::*;
use anyhow::{format_err, Context, Result};
use async_trait::async_trait;
use check::{
    metadata::{Metadata, METADATA_FILE},
    *,
};
use clap::ArgEnum;
#[cfg(feature = "mocks")]
use mockall::automock;
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};
use which::which;

mod check;
mod report;
pub use check::CheckOutput;
pub use report::*;

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
#[derive(Debug, Clone)]
pub struct LinterInput {
    pub root: PathBuf,
    pub url: String,
    pub check_sets: Vec<CheckSet>,
    pub github_token: String,
}

/// Check sets define a set of checks that will be run on a given repository.
/// Multiple check sets can be assigned to a repository.
#[derive(Debug, Clone, PartialEq, Eq, Hash, ArgEnum, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CheckSet {
    Code,
    CodeLite,
    Community,
    Docs,
}

/// CLOMonitor core linter (Linter implementation).
pub struct CoreLinter;

#[allow(clippy::new_without_default)]
impl CoreLinter {
    /// Create a new CoreLinter instance.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Linter for CoreLinter {
    async fn lint(&self, li: &LinterInput) -> Result<Report> {
        // Check if required external tools are available
        if which("scorecard").is_err() {
            return Err(format_err!(
                "scorecard not found in PATH (https://github.com/ossf/scorecard#installation)"
            ));
        }

        // Setup linter services
        let svc = CoreLinterServices::new(&li.github_token)?;

        // Get CLOMonitor metadata
        let cm_md = Metadata::from(&li.root.join(METADATA_FILE))?;

        // The next both actions (get GitHub metadata and get scorecard) make use
        // of the GitHub token, which when used concurrently, may trigger some
        // GitHub secondary rate limits. So they should not be run concurrently.

        // Get Github metadata
        let gh_md = github::metadata(&svc.http_client_gh, &li.url).await?;

        // Get OpenSSF scorecard
        let scorecard = scorecard(&li.url, &li.github_token)
            .await
            .context("error running scorecard command");

        // Prepare check input
        let ci = CheckInput {
            svc: &svc,
            li,
            cm_md: cm_md.as_ref(),
            gh_md: &gh_md,
            scorecard: &scorecard,
        };

        // Run some async checks
        let (analytics, contributing, trademark_disclaimer) = tokio::join!(
            run_async_check(ANALYTICS, analytics, &ci),
            run_async_check(CONTRIBUTING, contributing, &ci),
            run_async_check(TRADEMARK_DISCLAIMER, trademark_disclaimer, &ci),
        );

        // Run some sync checks
        let spdx_id = run_check(LICENSE_SPDX, license, &ci);
        let mut spdx_id_value: Option<String> = None;
        if let Some(r) = &spdx_id {
            spdx_id_value = r.value.clone();
        }

        // Build report
        let mut report = Report {
            documentation: Documentation {
                adopters: run_check(ADOPTERS, adopters, &ci),
                changelog: run_check(CHANGELOG, changelog, &ci),
                code_of_conduct: run_check(CODE_OF_CONDUCT, code_of_conduct, &ci),
                contributing,
                governance: run_check(GOVERNANCE, governance, &ci),
                maintainers: run_check(MAINTAINERS, maintainers, &ci),
                readme: run_check(README, readme, &ci),
                roadmap: run_check(ROADMAP, roadmap, &ci),
                website: run_check(WEBSITE, website, &ci),
            },
            license: License {
                license_approved: license_approved(spdx_id_value, &ci),
                license_scanning: run_check(LICENSE_SCANNING, license_scanning, &ci),
                license_spdx_id: spdx_id,
            },
            best_practices: BestPractices {
                analytics,
                artifacthub_badge: run_check(ARTIFACTHUB_BADGE, artifacthub_badge, &ci),
                cla: run_check(CLA, cla, &ci),
                community_meeting: run_check(COMMUNITY_MEETING, community_meeting, &ci),
                dco: run_check(DCO, dco, &ci),
                github_discussions: run_check(GITHUB_DISCUSSIONS, github_discussions, &ci),
                openssf_badge: run_check(OPENSSF_BADGE, openssf_badge, &ci),
                recent_release: run_check(RECENT_RELEASE, recent_release, &ci),
                slack_presence: run_check(SLACK_PRESENCE, slack_presence, &ci),
            },
            security: Security {
                binary_artifacts: run_check(BINARY_ARTIFACTS, binary_artifacts, &ci),
                code_review: run_check(CODE_REVIEW, code_review, &ci),
                dangerous_workflow: run_check(DANGEROUS_WORKFLOW, dangerous_workflow, &ci),
                dependency_update_tool: run_check(
                    DEPENDENCY_UPDATE_TOOL,
                    dependency_update_tool,
                    &ci,
                ),
                maintained: run_check(MAINTAINED, maintained, &ci),
                sbom: run_check(SBOM, sbom, &ci),
                security_policy: run_check(SECURITY_POLICY, security_policy, &ci),
                signed_releases: run_check(SIGNED_RELEASES, signed_releases, &ci),
                token_permissions: run_check(TOKEN_PERMISSIONS, token_permissions, &ci),
            },
            legal: Legal {
                trademark_disclaimer,
            },
        };
        report.apply_exemptions();

        Ok(report)
    }
}

/// Services used by the core linter to perform some of the checks.
#[derive(Debug, Clone)]
pub(crate) struct CoreLinterServices {
    pub http_client: reqwest::Client,
    pub http_client_gh: reqwest::Client,
}

impl CoreLinterServices {
    /// Create a new CoreLinterServices instance.
    pub fn new(github_token: &str) -> Result<Self> {
        Ok(Self {
            http_client: reqwest::Client::new(),
            http_client_gh: setup_github_http_client(github_token)?,
        })
    }
}
// Setup a new authenticated http client to interact with the GitHub API.
fn setup_github_http_client(github_token: &str) -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .user_agent("clomonitor")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_token))
                    .expect("header value only uses visible ascii chars"),
            ))
            .collect(),
        )
        .build()
}
