use super::{
    check::{self, Globs},
    github,
    metadata::*,
    patterns::*,
    LintOptions,
};
use anyhow::Error;
use octocrab::models::Repository;
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
    pub scanning: Option<String>,
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
    // Get CLOMonitor metadata
    let md = Metadata::from(options.root.join(METADATA_FILE))?;

    // Run some async expressions and wait for them to complete
    let (gh_md, best_practices) = tokio::try_join!(
        github::get_metadata(options.url),
        lint_best_practices(options.root, options.url)
    )?;

    Ok(Report {
        documentation: lint_documentation(options.root, &gh_md)?,
        license: lint_license(options.root, &md)?,
        best_practices,
        security: lint_security(options.root)?,
    })
}

/// Run documentation checks and prepare the report's documentation section.
fn lint_documentation(root: &Path, gh_md: &Repository) -> Result<Documentation, Error> {
    // Adopters
    let adopters = check::path_exists(Globs {
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
    )?;

    // Code of conduct
    let code_of_conduct = check::path_exists(Globs {
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
    )?;

    // Contributing
    let contributing = check::path_exists(Globs {
        root,
        patterns: CONTRIBUTING_FILE,
        case_sensitive: false,
    })?;

    // Changelog
    let changelog = check::path_exists(Globs {
        root,
        patterns: CHANGELOG_FILE,
        case_sensitive: false,
    })? || check::content_matches(
        Globs {
            root,
            patterns: README_FILE,
            case_sensitive: true,
        },
        CHANGELOG_HEADER,
    )?;

    // Governance
    let governance = check::path_exists(Globs {
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
    )?;

    // Maintainers
    let maintainers = check::path_exists(Globs {
        root,
        patterns: MAINTAINERS_FILE,
        case_sensitive: false,
    })?;

    // Readme
    let readme = check::path_exists(Globs {
        root,
        patterns: README_FILE,
        case_sensitive: true,
    })?;

    // Roadmap
    let roadmap = check::path_exists(Globs {
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
    )?;

    // Website
    let website = match &gh_md.homepage {
        Some(url) => !url.is_empty(),
        None => false,
    };

    Ok(Documentation {
        adopters,
        code_of_conduct,
        contributing,
        changelog,
        governance,
        maintainers,
        readme,
        roadmap,
        website,
    })
}

/// Run license checks and prepare the report's license section.
fn lint_license(root: &Path, md: &Option<Metadata>) -> Result<License, Error> {
    // SPDX id
    let spdx_id = check::license(Globs {
        root,
        patterns: LICENSE_FILE,
        case_sensitive: true,
    })?;

    // Approved
    let mut approved: Option<bool> = None;
    if let Some(spdx_id) = &spdx_id {
        approved = Some(check::is_approved_license(spdx_id))
    }

    // Scanning url
    let mut scanning_url: Option<String> = None;
    if let Some(md) = md {
        if let Some(license_scanning) = &md.license_scanning {
            if let Some(url) = &license_scanning.url {
                scanning_url = Some(url.to_owned())
            }
        }
    }
    if scanning_url.is_none() {
        scanning_url = check::content_find(
            Globs {
                root,
                patterns: README_FILE,
                case_sensitive: true,
            },
            LICENSE_SCANNING_URL,
        )?;
    }

    Ok(License {
        approved,
        scanning: scanning_url,
        spdx_id,
    })
}

/// Run best practices checks and prepare the report's best practices section.
async fn lint_best_practices(root: &Path, repo_url: &str) -> Result<BestPractices, Error> {
    // Artifact Hub badge
    let artifacthub_badge = check::content_matches(
        Globs {
            root,
            patterns: README_FILE,
            case_sensitive: true,
        },
        ARTIFACTHUB_BADGE_URL,
    )?;

    // Community meeting
    let community_meeting = check::content_matches(
        Globs {
            root,
            patterns: README_FILE,
            case_sensitive: true,
        },
        COMMUNITY_MEETING_TEXT,
    )?;

    // OpenSSF badge
    let openssf_badge = check::content_matches(
        Globs {
            root,
            patterns: README_FILE,
            case_sensitive: true,
        },
        OPENSSF_BADGE_URL,
    )?;

    // Async checks: recent_release
    let (recent_release,) = tokio::try_join!(github::has_recent_release(repo_url))?;

    Ok(BestPractices {
        artifacthub_badge,
        community_meeting,
        openssf_badge,
        recent_release,
    })
}

/// Run security checks and prepare the report's security section.
fn lint_security(root: &Path) -> Result<Security, Error> {
    // Security policy
    let security_policy = check::path_exists(Globs {
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
    )?;

    Ok(Security { security_policy })
}
