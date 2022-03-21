use super::{
    check::{self, github, metadata::*, CheckOptions, CheckResult},
    LintOptions,
};
use anyhow::Error;
use serde::{Deserialize, Serialize};

/// A linter report for a repository of kind primary.
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
    pub adopters: CheckResult,
    pub changelog: CheckResult,
    pub code_of_conduct: CheckResult,
    pub contributing: CheckResult,
    pub governance: CheckResult,
    pub maintainers: CheckResult,
    pub readme: CheckResult,
    pub roadmap: CheckResult,
    pub website: CheckResult,
}

/// License section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct License {
    pub approved: CheckResult<bool>,
    pub scanning: CheckResult,
    pub spdx_id: CheckResult<String>,
}

/// BestPractices section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BestPractices {
    pub artifacthub_badge: CheckResult,
    pub community_meeting: CheckResult,
    pub dco: CheckResult,
    pub openssf_badge: CheckResult,
    pub recent_release: CheckResult,
    pub slack_presence: CheckResult,
}

/// Security section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Security {
    pub security_policy: CheckResult,
}

/// Legal section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Legal {
    pub trademark_disclaimer: CheckResult,
}

/// Lint the path provided and return a report.
pub async fn lint(opts: LintOptions) -> Result<Report, Error> {
    // Get CLOMonitor metadata
    let md = Metadata::from(&opts.root.join(METADATA_FILE))?;

    // Get Github metadata
    let gh_md = github::get_repo_metadata(&opts.url).await?;

    // Prepare check options
    let opts = CheckOptions {
        root: opts.root,
        url: opts.url,
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
        security_policy,
        trademark_disclaimer,
    ) = tokio::try_join!(
        check::changelog(&opts),
        check::code_of_conduct(&opts),
        check::contributing(&opts),
        check::dco(&opts),
        check::recent_release(&opts),
        check::security_policy(&opts),
        check::trademark_disclaimer(&opts),
    )?;

    // Sync checks
    let spdx_id = check::license(&opts)?;

    // Build report and return it
    Ok(Report {
        documentation: Documentation {
            adopters: check::adopters(&opts)?,
            changelog,
            code_of_conduct,
            contributing,
            governance: check::governance(&opts)?,
            maintainers: check::maintainers(&opts)?,
            readme: check::readme(&opts)?,
            roadmap: check::roadmap(&opts)?,
            website: check::website(&opts)?,
        },
        license: License {
            approved: check::license_approved(&spdx_id.value, &opts)?,
            scanning: check::license_scanning(&opts)?,
            spdx_id,
        },
        best_practices: BestPractices {
            artifacthub_badge: check::artifacthub_badge(&opts)?,
            community_meeting: check::community_meeting(&opts)?,
            dco,
            openssf_badge: check::openssf_badge(&opts)?,
            recent_release,
            slack_presence: check::slack_presence(&opts)?,
        },
        security: Security { security_policy },
        legal: Legal {
            trademark_disclaimer,
        },
    })
}
