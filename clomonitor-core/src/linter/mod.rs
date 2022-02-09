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
    pub quality: Quality,
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
    pub spdx_id: Option<String>,
}

/// Quality section of a linter report.
#[derive(Debug, Serialize, Deserialize)]
pub struct Quality {
    pub fossa_badge: bool,
    pub openssf_badge: bool,
}

/// Security section of a linter report.
#[derive(Debug, Serialize, Deserialize)]
pub struct Security {
    pub security_policy: bool,
}

/// Lint the path provided and return a report.
pub fn lint(root: &Path) -> Result<Report, Error> {
    Ok(Report {
        documentation: Documentation {
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
        },
        license: check::license(Globs {
            root,
            patterns: vec!["LICENSE*", "COPYING*"],
            case_sensitive: true,
        })?,
        quality: Quality {
            fossa_badge: check::content_matches(
                Globs {
                    root,
                    patterns: vec!["README*"],
                    case_sensitive: true,
                },
                vec![r"https://app.fossa.*/api/projects/.*"],
            )?,
            openssf_badge: check::content_matches(
                Globs {
                    root,
                    patterns: vec!["README*"],
                    case_sensitive: true,
                },
                vec![r"https://bestpractices.coreinfrastructure.org/projects/\d+"],
            )?,
        },
        security: Security {
            security_policy: check::path_exists(Globs {
                root,
                patterns: vec!["security*", "docs/security*", ".github/security*"],
                case_sensitive: false,
            })?,
        },
    })
}
