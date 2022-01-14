use crate::check::Globs;
use serde::Serialize;
use std::error::Error;
use std::path::Path;

mod check;

/// A linter report.
#[derive(Debug, Serialize)]
pub struct Report {
    pub documentation: Documentation,
    pub license: License,
    pub quality: Quality,
    pub security: Security,
}

/// Documentation section of a linter report.
#[derive(Debug, Serialize)]
pub struct Documentation {
    pub code_of_conduct: bool,
    pub contributing: bool,
    pub changelog: bool,
    pub governance: bool,
    pub readme: bool,
}

/// License section of a linter report.
#[derive(Debug, Serialize)]
pub struct License {
    pub spdx_id: Option<String>,
}

/// Quality section of a linter report.
#[derive(Debug, Serialize)]
pub struct Quality {
    pub fossa: bool,
    pub openssf_badge: bool,
}

/// Security section of a linter report.
#[derive(Debug, Serialize)]
pub struct Security {
    pub security_policy: bool,
}

/// Run the linter in the path provided and return a report.
pub fn lint(root: &Path) -> Result<Report, Box<dyn Error>> {
    Ok(Report {
        documentation: Documentation {
            code_of_conduct: check::path_exists(Globs {
                root,
                patterns: vec!["code*of*conduct.md", "docs/code*of*conduct.md"],
                case_sensitive: false,
            })?,
            contributing: check::path_exists(Globs {
                root,
                patterns: vec!["CONTRIBUTING.md", "docs/CONTRIBUTING.md"],
                case_sensitive: false,
            })?,
            changelog: check::path_exists(Globs {
                root,
                patterns: vec!["CHANGELOG*"],
                case_sensitive: false,
            })?,
            governance: check::path_exists(Globs {
                root,
                patterns: vec!["GOVERNANCE*", "docs/GOVERNANCE*"],
                case_sensitive: false,
            })?,
            readme: check::path_exists(Globs {
                root,
                patterns: vec!["README*"],
                case_sensitive: true,
            })?,
        },
        license: check::license(Globs {
            root,
            patterns: vec!["LICENSE*", "COPYING*"],
            case_sensitive: true,
        })?,
        quality: Quality {
            fossa: check::path_exists(Globs {
                root,
                patterns: vec![".github/workflows/fossa.y*ml"],
                case_sensitive: false,
            })?,
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
                patterns: vec!["SECURITY.md", "docs/SECURITY.md", ".github/SECURITY.md"],
                case_sensitive: true,
            })?,
        },
    })
}
