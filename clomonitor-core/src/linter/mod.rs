mod check;

use anyhow::{format_err, Error};
use check::Globs;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Supported linters.
#[derive(Debug)]
pub enum Linter {
    Core = 0,
}

impl std::convert::TryFrom<i32> for Linter {
    type Error = Error;

    fn try_from(linter_id: i32) -> Result<Self, Self::Error> {
        match linter_id {
            0 => Ok(Linter::Core),
            _ => Err(format_err!("invalid linter id")),
        }
    }
}

/// A linter report.
#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub documentation: Documentation,
    pub license: License,
    pub best_practices: BestPractices,
    pub security: Security,
}

/// Documentation section of a linter report.
#[derive(Debug, Serialize, Deserialize)]
pub struct Documentation {
    pub adopters: bool,
    pub code_of_conduct: bool,
    pub contributing: bool,
    pub changelog: bool,
    pub governance: bool,
    pub maintainers: bool,
    pub readme: bool,
    pub roadmap: bool,
}

/// License section of a linter report.
#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    pub approved: Option<bool>,
    pub fossa_badge: bool,
    pub spdx_id: Option<String>,
}

/// BestPractices section of a linter report.
#[derive(Debug, Serialize, Deserialize)]
pub struct BestPractices {
    pub community_meeting: bool,
    pub openssf_badge: bool,
}

/// Security section of a linter report.
#[derive(Debug, Serialize, Deserialize)]
pub struct Security {
    pub security_policy: bool,
}

/// Lint the path provided and return a linter report.
pub fn lint(root: &Path) -> Result<Report, Error> {
    Ok(Report {
        documentation: lint_documentation(root)?,
        license: lint_license(root)?,
        best_practices: lint_best_practices(root)?,
        security: lint_security(root)?,
    })
}

/// Run documentation checks and prepare the report's documentation section.
fn lint_documentation(root: &Path) -> Result<Documentation, Error> {
    Ok(Documentation {
        adopters: check::path_exists(Globs {
            root,
            patterns: vec!["adopters*"],
            case_sensitive: false,
        })?,
        code_of_conduct: check::path_exists(Globs {
            root,
            patterns: vec!["code*of*conduct.md", "docs/code*of*conduct.md"],
            case_sensitive: false,
        })?,
        contributing: check::path_exists(Globs {
            root,
            patterns: vec!["contributing*", "docs/contributing*"],
            case_sensitive: false,
        })?,
        changelog: check::path_exists(Globs {
            root,
            patterns: vec!["changelog*"],
            case_sensitive: false,
        })?,
        governance: check::path_exists(Globs {
            root,
            patterns: vec!["governance*", "docs/governance*"],
            case_sensitive: false,
        })?,
        maintainers: check::path_exists(Globs {
            root,
            patterns: vec![
                "maintainers*",
                "docs/maintainers*",
                "owners*",
                "docs/owners*",
                "codeowners*",
                "docs/codeowners*",
                ".github/codeowners*",
            ],
            case_sensitive: false,
        })?,
        readme: check::path_exists(Globs {
            root,
            patterns: vec!["README*"],
            case_sensitive: true,
        })?,
        roadmap: check::path_exists(Globs {
            root,
            patterns: vec!["roadmap*"],
            case_sensitive: false,
        })?,
    })
}

/// Run license checks and prepare the report's license section.
fn lint_license(root: &Path) -> Result<License, Error> {
    let spdx_id = check::license(Globs {
        root,
        patterns: vec!["LICENSE*", "COPYING*"],
        case_sensitive: true,
    })?;

    let mut approved: Option<bool> = None;
    if let Some(spdx_id) = &spdx_id {
        approved = Some(check::is_approved_license(spdx_id))
    }

    Ok(License {
        approved,
        fossa_badge: check::content_matches(
            Globs {
                root,
                patterns: vec!["README*"],
                case_sensitive: true,
            },
            vec![r"https://app.fossa.*/api/projects/.*"],
        )?,
        spdx_id,
    })
}

/// Run best practices checks and prepare the report's best practices section.
fn lint_best_practices(root: &Path) -> Result<BestPractices, Error> {
    Ok(BestPractices {
        community_meeting: check::content_matches(
            Globs {
                root,
                patterns: vec!["README*"],
                case_sensitive: true,
            },
            vec![
                r"(?i)(community|developer|development) (call|event|meeting|session)",
                r"(?i)(weekly|biweekly|monthly) meeting",
                r"(?i)meeting minutes",
            ],
        )?,
        openssf_badge: check::content_matches(
            Globs {
                root,
                patterns: vec!["README*"],
                case_sensitive: true,
            },
            vec![r"https://bestpractices.coreinfrastructure.org/projects/\d+"],
        )?,
    })
}

/// Run security checks and prepare the report's security section.
fn lint_security(root: &Path) -> Result<Security, Error> {
    Ok(Security {
        security_policy: check::path_exists(Globs {
            root,
            patterns: vec!["security*", "docs/security*", ".github/security*"],
            case_sensitive: false,
        })?,
    })
}
