use super::{
    check::{self, Globs},
    patterns::*,
    LintOptions,
};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// A linter report for a repository of kind primary.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Report {
    pub documentation: Documentation,
    pub license: License,
    pub best_practices: BestPractices,
    pub security: Security,
}

/// Documentation section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Documentation {
    pub adopters: bool,
    pub code_of_conduct: bool,
    pub contributing: bool,
    pub changelog: bool,
    pub governance: bool,
    pub maintainers: bool,
    pub readme: bool,
    pub roadmap: bool,
    pub website: bool,
}

/// License section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct License {
    pub approved: Option<bool>,
    pub fossa_badge: bool,
    pub spdx_id: Option<String>,
}

/// BestPractices section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BestPractices {
    pub artifacthub_badge: bool,
    pub community_meeting: bool,
    pub openssf_badge: bool,
}

/// Security section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Security {
    pub security_policy: bool,
}

/// Lint the path provided and return a report.
pub async fn lint(options: LintOptions<'_>) -> Result<Report, Error> {
    Ok(Report {
        documentation: lint_documentation(options.root, options.url).await?,
        license: lint_license(options.root)?,
        best_practices: lint_best_practices(options.root)?,
        security: lint_security(options.root)?,
    })
}

/// Run documentation checks and prepare the report's documentation section.
async fn lint_documentation(root: &Path, repo_url: &str) -> Result<Documentation, Error> {
    Ok(Documentation {
        adopters: check::path_exists(Globs {
            root,
            patterns: ADOPTERS,
            case_sensitive: false,
        })?,
        code_of_conduct: check::path_exists(Globs {
            root,
            patterns: CODE_OF_CONDUCT,
            case_sensitive: false,
        })?,
        contributing: check::path_exists(Globs {
            root,
            patterns: CONTRIBUTING,
            case_sensitive: false,
        })?,
        changelog: check::path_exists(Globs {
            root,
            patterns: CHANGELOG,
            case_sensitive: false,
        })?,
        governance: check::path_exists(Globs {
            root,
            patterns: GOVERNANCE,
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
        roadmap: check::path_exists(Globs {
            root,
            patterns: ROADMAP,
            case_sensitive: false,
        })?,
        website: check::has_website(repo_url).await,
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

    Ok(License {
        approved,
        fossa_badge: check::content_matches(
            Globs {
                root,
                patterns: README,
                case_sensitive: true,
            },
            FOSSA_BADGE,
        )?,
        spdx_id,
    })
}

/// Run best practices checks and prepare the report's best practices section.
fn lint_best_practices(root: &Path) -> Result<BestPractices, Error> {
    Ok(BestPractices {
        artifacthub_badge: check::content_matches(
            Globs {
                root,
                patterns: README,
                case_sensitive: true,
            },
            ARTIFACTHUB_BADGE,
        )?,
        community_meeting: check::content_matches(
            Globs {
                root,
                patterns: README,
                case_sensitive: true,
            },
            COMMUNITY_MEETING,
        )?,
        openssf_badge: check::content_matches(
            Globs {
                root,
                patterns: README,
                case_sensitive: true,
            },
            OPENSSF_BADGE,
        )?,
    })
}

/// Run security checks and prepare the report's security section.
fn lint_security(root: &Path) -> Result<Security, Error> {
    Ok(Security {
        security_policy: check::path_exists(Globs {
            root,
            patterns: SECURITY_POLICY,
            case_sensitive: false,
        })?,
    })
}
