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
    pub recent_release: bool,
}

/// Security section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Security {
    pub security_policy: bool,
}

/// Lint the path provided and return a report.
pub async fn lint(options: LintOptions<'_>) -> Result<Report, Error> {
    let result = tokio::try_join!(
        lint_documentation(options.root, options.url),
        lint_best_practices(options.root, options.url),
    );
    match result {
        Ok((documentation, best_practices)) => Ok(Report {
            documentation,
            license: lint_license(options.root)?,
            best_practices,
            security: lint_security(options.root)?,
        }),
        Err(err) => Err(err),
    }
}

/// Run documentation checks and prepare the report's documentation section.
async fn lint_documentation(root: &Path, repo_url: &str) -> Result<Documentation, Error> {
    let result = tokio::try_join!(check::has_website(repo_url));
    match result {
        Ok((website,)) => Ok(Documentation {
            adopters: check::path_exists(Globs {
                root,
                patterns: ADOPTERS_FILE,
                case_sensitive: false,
            })? || check::content_matches(
                Globs {
                    root,
                    patterns: README_FILE,
                    case_sensitive: true,
                },
                ADOPTERS_HEADER,
            )?,
            code_of_conduct: check::path_exists(Globs {
                root,
                patterns: CODE_OF_CONDUCT_FILE,
                case_sensitive: false,
            })? || check::content_matches(
                Globs {
                    root,
                    patterns: README_FILE,
                    case_sensitive: true,
                },
                CODE_OF_CONDUCT_HEADER,
            )?,
            contributing: check::path_exists(Globs {
                root,
                patterns: CONTRIBUTING_FILE,
                case_sensitive: false,
            })?,
            changelog: check::path_exists(Globs {
                root,
                patterns: CHANGELOG_FILE,
                case_sensitive: false,
            })?,
            governance: check::path_exists(Globs {
                root,
                patterns: GOVERNANCE_FILE,
                case_sensitive: false,
            })? || check::content_matches(
                Globs {
                    root,
                    patterns: README_FILE,
                    case_sensitive: true,
                },
                GOVERNANCE_HEADER,
            )?,
            maintainers: check::path_exists(Globs {
                root,
                patterns: MAINTAINERS_FILE,
                case_sensitive: false,
            })?,
            readme: check::path_exists(Globs {
                root,
                patterns: README_FILE,
                case_sensitive: true,
            })?,
            roadmap: check::path_exists(Globs {
                root,
                patterns: ROADMAP_FILE,
                case_sensitive: false,
            })? || check::content_matches(
                Globs {
                    root,
                    patterns: README_FILE,
                    case_sensitive: true,
                },
                ROADMAP_HEADER,
            )?,
            website,
        }),
        Err(err) => Err(err),
    }
}

/// Run license checks and prepare the report's license section.
fn lint_license(root: &Path) -> Result<License, Error> {
    let spdx_id = check::license(Globs {
        root,
        patterns: LICENSE_FILE,
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
                patterns: README_FILE,
                case_sensitive: true,
            },
            FOSSA_BADGE_URL,
        )?,
        spdx_id,
    })
}

/// Run best practices checks and prepare the report's best practices section.
async fn lint_best_practices(root: &Path, repo_url: &str) -> Result<BestPractices, Error> {
    let result = tokio::try_join!(check::has_recent_release(repo_url));
    match result {
        Ok((recent_release,)) => Ok(BestPractices {
            artifacthub_badge: check::content_matches(
                Globs {
                    root,
                    patterns: README_FILE,
                    case_sensitive: true,
                },
                ARTIFACTHUB_BADGE_URL,
            )?,
            community_meeting: check::content_matches(
                Globs {
                    root,
                    patterns: README_FILE,
                    case_sensitive: true,
                },
                COMMUNITY_MEETING_TEXT,
            )?,
            openssf_badge: check::content_matches(
                Globs {
                    root,
                    patterns: README_FILE,
                    case_sensitive: true,
                },
                OPENSSF_BADGE_URL,
            )?,
            recent_release,
        }),
        Err(err) => Err(err),
    }
}

/// Run security checks and prepare the report's security section.
fn lint_security(root: &Path) -> Result<Security, Error> {
    Ok(Security {
        security_policy: check::path_exists(Globs {
            root,
            patterns: SECURITY_POLICY_FILE,
            case_sensitive: false,
        })? || check::content_matches(
            Globs {
                root,
                patterns: README_FILE,
                case_sensitive: true,
            },
            SECURITY_POLICY_HEADER,
        )?,
    })
}
