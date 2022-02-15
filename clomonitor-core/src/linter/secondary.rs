use super::{
    check::{self, Globs},
    patterns::*,
};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// A linter report for a repository of kind secondary.
#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub documentation: Documentation,
    pub license: License,
}

/// Documentation section of the report.
#[derive(Debug, Serialize, Deserialize)]
pub struct Documentation {
    pub contributing: bool,
    pub maintainers: bool,
    pub readme: bool,
}

/// License section of the report.
#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    pub approved: Option<bool>,
    pub spdx_id: Option<String>,
}

/// Lint the path provided and return a report.
pub fn lint(root: &Path) -> Result<Report, Error> {
    Ok(Report {
        documentation: lint_documentation(root)?,
        license: lint_license(root)?,
    })
}

/// Run documentation checks and prepare the report's documentation section.
fn lint_documentation(root: &Path) -> Result<Documentation, Error> {
    Ok(Documentation {
        contributing: check::path_exists(Globs {
            root,
            patterns: CONTRIBUTING,
            case_sensitive: false,
        })?,
        maintainers: check::path_exists(Globs {
            root,
            patterns: MAINTAINERS,
            case_sensitive: false,
        })?,
        readme: check::path_exists(Globs {
            root,
            patterns: README,
            case_sensitive: true,
        })?,
    })
}

/// Run license checks and prepare the report's license section.
fn lint_license(root: &Path) -> Result<License, Error> {
    let spdx_id = check::license(Globs {
        root,
        patterns: LICENSE,
        case_sensitive: true,
    })?;

    let mut approved: Option<bool> = None;
    if let Some(spdx_id) = &spdx_id {
        approved = Some(check::is_approved_license(spdx_id))
    }

    Ok(License { approved, spdx_id })
}
