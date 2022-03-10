use super::{
    check, check::github, check::path::Globs, check_result::CheckResult, patterns::*, LintOptions,
};
use anyhow::Error;
use octocrab::models::Repository;
use serde::{Deserialize, Serialize};
use std::path::Path;

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
pub async fn lint(options: LintOptions<'_>) -> Result<Report, Error> {
    // Get Github metadata
    let gh_md = github::get_metadata(options.url).await?;

    // Async checks: documentation
    let (documentation,) = tokio::try_join!(lint_documentation(options.root, &gh_md),)?;

    Ok(Report {
        documentation,
        license: lint_license(options.root)?,
    })
}

/// Run documentation checks and prepare the report's documentation section.
async fn lint_documentation(root: &Path, gh_md: &Repository) -> Result<Documentation, Error> {
    // Contributing
    let contributing =
        check::path::exists(Globs {
            root,
            patterns: CONTRIBUTING_FILE,
            case_sensitive: false,
        })? || check::github::has_default_community_health_file(gh_md, "CONTRIBUTING.md").await?;

    // Maintainers
    let maintainers = check::path::exists(Globs {
        root,
        patterns: MAINTAINERS_FILE,
        case_sensitive: false,
    })?;

    // Readme
    let readme = check::path::exists(Globs {
        root,
        patterns: README_FILE,
        case_sensitive: true,
    })?;

    Ok(Documentation {
        contributing: contributing.into(),
        maintainers: maintainers.into(),
        readme: readme.into(),
    })
}

/// Run license checks and prepare the report's license section.
fn lint_license(root: &Path) -> Result<License, Error> {
    // SPDX id
    let spdx_id = check::license::detect(Globs {
        root,
        patterns: LICENSE_FILE,
        case_sensitive: true,
    })?;

    // Approved
    let mut approved: Option<bool> = None;
    if let Some(spdx_id) = &spdx_id {
        approved = Some(check::license::is_approved(spdx_id))
    }

    Ok(License {
        approved: (approved.unwrap_or(false), approved).into(),
        spdx_id: spdx_id.into(),
    })
}
