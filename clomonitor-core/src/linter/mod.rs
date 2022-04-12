use super::config::*;
use anyhow::Result;
use check::{
    metadata::{Metadata, METADATA_FILE},
    *,
};
use clap::ArgEnum;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod check;
pub use check::CheckOutput;

/// Linter configuration options.
#[derive(Debug)]
pub struct LintOptions {
    pub root: PathBuf,
    pub url: String,
    pub check_sets: Vec<CheckSet>,
}

/// Credentials used by LintServices.
#[derive(Debug, Default)]
pub struct LintCredentials {
    pub github_token: Option<String>,
}

/// Services used by the linter to perform some of the checks.
#[derive(Debug)]
#[non_exhaustive]
pub struct LintServices {
    pub http_client: reqwest::Client,
    pub github_client: octocrab::Octocrab,
}

impl LintServices {
    /// Create a new LintServices instance.
    pub fn new(creds: LintCredentials) -> Result<Self> {
        // Setup GitHub client
        let mut octocrab_builder = octocrab::Octocrab::builder();
        if let Some(token) = creds.github_token {
            octocrab_builder = octocrab_builder.personal_token(token);
        }

        Ok(Self {
            http_client: reqwest::Client::new(),
            github_client: octocrab_builder.build()?,
        })
    }
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
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct License {
    pub approved: Option<CheckOutput<bool>>,
    pub scanning: Option<CheckOutput>,
    pub spdx_id: Option<CheckOutput<String>>,
}

/// BestPractices section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BestPractices {
    pub artifacthub_badge: Option<CheckOutput>,
    pub cla: Option<CheckOutput>,
    pub community_meeting: Option<CheckOutput>,
    pub dco: Option<CheckOutput>,
    pub openssf_badge: Option<CheckOutput>,
    pub recent_release: Option<CheckOutput>,
    pub slack_presence: Option<CheckOutput>,
}

/// Security section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Security {
    pub sbom: Option<CheckOutput>,
    pub security_policy: Option<CheckOutput>,
}

/// Legal section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Legal {
    pub trademark_disclaimer: Option<CheckOutput>,
}

/// Lint the path provided and return a report.
pub async fn lint(opts: &LintOptions, svc: &LintServices) -> Result<Report> {
    // Get CLOMonitor metadata
    let cm_md = Metadata::from(&opts.root.join(METADATA_FILE))?;

    // Get Github metadata
    let gh_md = github::get_repo_metadata(&svc.github_client, &opts.url).await?;

    // Prepare check input
    let input = CheckInput {
        opts,
        svc,
        cm_md,
        gh_md,
    };

    // Run some async checks
    let (
        changelog,
        cla,
        code_of_conduct,
        contributing,
        dco,
        recent_release,
        sbom,
        security_policy,
        trademark_disclaimer,
    ) = tokio::join!(
        run_async_check(CHANGELOG, changelog, &input),
        run_async_check(CLA, cla, &input),
        run_async_check(CODE_OF_CONDUCT, code_of_conduct, &input),
        run_async_check(CONTRIBUTING, contributing, &input),
        run_async_check(DCO, dco, &input),
        run_async_check(RECENT_RELEASE, recent_release, &input),
        run_async_check(SBOM, sbom, &input),
        run_async_check(SECURITY_POLICY, security_policy, &input),
        run_async_check(TRADEMARK_DISCLAIMER, trademark_disclaimer, &input),
    );

    // Run some sync checks
    let spdx_id = run_check(LICENSE_SPDX, license, &input);
    let mut spdx_id_value: &Option<String> = &None;
    if let Some(r) = &spdx_id {
        spdx_id_value = &r.value;
    }

    // Build report and return it
    let mut report = Report {
        documentation: Documentation {
            adopters: run_check(ADOPTERS, adopters, &input),
            changelog,
            code_of_conduct,
            contributing,
            governance: run_check(GOVERNANCE, governance, &input),
            maintainers: run_check(MAINTAINERS, maintainers, &input),
            readme: run_check(README, readme, &input),
            roadmap: run_check(ROADMAP, roadmap, &input),
            website: run_check(WEBSITE, website, &input),
        },
        license: License {
            approved: license_approved(spdx_id_value, &input),
            scanning: run_check(LICENSE_SCANNING, license_scanning, &input),
            spdx_id,
        },
        best_practices: BestPractices {
            artifacthub_badge: run_check(ARTIFACTHUB_BADGE, artifacthub_badge, &input),
            cla,
            community_meeting: run_check(COMMUNITY_MEETING, community_meeting, &input),
            dco,
            openssf_badge: run_check(OPENSSF_BADGE, openssf_badge, &input),
            recent_release,
            slack_presence: run_check(SLACK_PRESENCE, slack_presence, &input),
        },
        security: Security {
            sbom,
            security_policy,
        },
        legal: Legal {
            trademark_disclaimer,
        },
    };

    apply_exemptions(&mut report);
    Ok(report)
}

/// Apply inter-checks exemptions.
fn apply_exemptions(report: &mut Report) {
    let passed = |o: &Option<CheckOutput>| -> bool {
        match o {
            Some(o) => o.passed || o.exempt,
            None => false,
        }
    };

    // CLA / DCO
    if passed(&report.best_practices.cla) && !passed(&report.best_practices.dco) {
        report.best_practices.dco = Some(CheckOutput {
            exempt: true,
            exemption_reason: Some("CLA check passed".to_string()),
            ..Default::default()
        });
    }
    if passed(&report.best_practices.dco) && !passed(&report.best_practices.cla) {
        report.best_practices.cla = Some(CheckOutput {
            exempt: true,
            exemption_reason: Some("DCO check passed".to_string()),
            ..Default::default()
        });
    }
}
