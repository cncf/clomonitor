use super::{
    check::{self, github, metadata::*, CheckOptions, CheckResult},
    LintOptions,
};
use anyhow::Error;
use serde::{Deserialize, Serialize};

/// A linter report for a repository of kind secondary.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Report {
    pub documentation: Documentation,
    pub license: License,
}

/// Documentation section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Documentation {
    pub contributing: CheckResult,
    pub maintainers: CheckResult,
    pub readme: CheckResult,
}

/// License section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct License {
    pub approved: CheckResult<bool>,
    pub spdx_id: CheckResult<String>,
}

/// Lint the path provided and return a report.
pub async fn lint(opts: LintOptions) -> Result<Report, Error> {
    // Get CLOMonitor metadata
    let md = Metadata::from(&opts.root.join(METADATA_FILE))?;

    // Get Github metadata
    let gh_md = github::get_metadata(&opts.url).await?;

    // Prepare check options
    let opts = CheckOptions {
        root: opts.root,
        url: opts.url,
        md,
        gh_md,
    };

    // Async checks
    let (contributing,) = tokio::try_join!(check::contributing(&opts),)?;

    // Sync checks
    let spdx_id = check::license(&opts)?;

    // Build report and return it
    Ok(Report {
        documentation: Documentation {
            contributing,
            maintainers: check::maintainers(&opts)?,
            readme: check::readme(&opts)?,
        },
        license: License {
            approved: check::license_approved(&spdx_id.value)?,
            spdx_id,
        },
    })
}
