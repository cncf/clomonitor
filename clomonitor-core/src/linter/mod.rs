use super::config::*;
use anyhow::Error;
use check::{
    metadata::{Metadata, METADATA_FILE},
    *,
};
use clap::ArgEnum;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod check;
pub use check::CheckResult;

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

/// Linter configuration options.
pub struct LintOptions {
    pub check_sets: Vec<CheckSet>,
    pub root: PathBuf,
    pub url: String,
    pub github_token: Option<String>,
}

/// Linter report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Report {
    pub documentation: Documentation,
    pub license: License,
    pub best_practices: BestPractices,
    pub security: Security,
    pub legal: Legal,
}

/// Documentation section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Documentation {
    pub adopters: Option<CheckResult>,
    pub changelog: Option<CheckResult>,
    pub code_of_conduct: Option<CheckResult>,
    pub contributing: Option<CheckResult>,
    pub governance: Option<CheckResult>,
    pub maintainers: Option<CheckResult>,
    pub readme: Option<CheckResult>,
    pub roadmap: Option<CheckResult>,
    pub website: Option<CheckResult>,
}

/// License section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct License {
    pub approved: Option<CheckResult<bool>>,
    pub scanning: Option<CheckResult>,
    pub spdx_id: Option<CheckResult<String>>,
}

/// BestPractices section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BestPractices {
    pub artifacthub_badge: Option<CheckResult>,
    pub community_meeting: Option<CheckResult>,
    pub dco: Option<CheckResult>,
    pub openssf_badge: Option<CheckResult>,
    pub recent_release: Option<CheckResult>,
    pub slack_presence: Option<CheckResult>,
}

/// Security section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Security {
    pub sbom: Option<CheckResult>,
    pub security_policy: Option<CheckResult>,
}

/// Legal section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Legal {
    pub trademark_disclaimer: Option<CheckResult>,
}

/// Lint the path provided and return a report.
pub async fn lint(lint_opts: LintOptions) -> Result<Report, Error> {
    // Initialize Github API client
    let mut builder = octocrab::Octocrab::builder();
    if let Some(token) = &lint_opts.github_token {
        builder = builder.personal_token(token.to_string());
    }
    octocrab::initialise(builder)?;

    // Get CLOMonitor metadata
    let md = Metadata::from(&lint_opts.root.join(METADATA_FILE))?;

    // Get Github metadata
    let gh_md = github::get_repo_metadata(&lint_opts.url).await?;

    // Prepare check options
    let check_opts = CheckOptions {
        check_sets: lint_opts.check_sets,
        root: lint_opts.root,
        url: lint_opts.url,
        md,
        gh_md,
    };

    // Async checks
    let (
        changelog,
        code_of_conduct,
        contributing,
        dco,
        recent_release,
        sbom,
        security_policy,
        trademark_disclaimer,
    ) = tokio::try_join!(
        run_async_check(CHANGELOG, changelog, &check_opts),
        run_async_check(CODE_OF_CONDUCT, code_of_conduct, &check_opts),
        run_async_check(CONTRIBUTING, contributing, &check_opts),
        run_async_check(DCO, dco, &check_opts),
        run_async_check(RECENT_RELEASE, recent_release, &check_opts),
        run_async_check(SBOM, sbom, &check_opts),
        run_async_check(SECURITY_POLICY, security_policy, &check_opts),
        run_async_check(TRADEMARK_DISCLAIMER, trademark_disclaimer, &check_opts),
    )?;

    // Sync checks
    let spdx_id = run_check(LICENSE_SPDX, license, &check_opts)?;
    let mut spdx_id_value: &Option<String> = &None;
    if let Some(r) = &spdx_id {
        spdx_id_value = &r.value;
    }

    // Build report and return it
    Ok(Report {
        documentation: Documentation {
            adopters: run_check(ADOPTERS, adopters, &check_opts)?,
            changelog,
            code_of_conduct,
            contributing,
            governance: run_check(GOVERNANCE, governance, &check_opts)?,
            maintainers: run_check(MAINTAINERS, maintainers, &check_opts)?,
            readme: run_check(README, readme, &check_opts)?,
            roadmap: run_check(ROADMAP, roadmap, &check_opts)?,
            website: run_check(WEBSITE, website, &check_opts)?,
        },
        license: License {
            approved: license_approved(spdx_id_value, &check_opts)?,
            scanning: run_check(LICENSE_SCANNING, license_scanning, &check_opts)?,
            spdx_id,
        },
        best_practices: BestPractices {
            artifacthub_badge: run_check(ARTIFACTHUB_BADGE, artifacthub_badge, &check_opts)?,
            community_meeting: run_check(COMMUNITY_MEETING, community_meeting, &check_opts)?,
            dco,
            openssf_badge: run_check(OPENSSF_BADGE, openssf_badge, &check_opts)?,
            recent_release,
            slack_presence: run_check(SLACK_PRESENCE, slack_presence, &check_opts)?,
        },
        security: Security {
            sbom,
            security_policy,
        },
        legal: Legal {
            trademark_disclaimer,
        },
    })
}
