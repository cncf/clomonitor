use super::{
    check, check::github, check::path::Globs, check_result::CheckResult, metadata::*, patterns::*,
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
    pub legal: Legal,
}

/// Documentation section of the report.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Documentation {
    pub adopters: CheckResult,
    pub code_of_conduct: CheckResult,
    pub contributing: CheckResult,
    pub changelog: CheckResult,
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
    pub trademark_footer: CheckResult,
}

/// Lint the path provided and return a report.
pub async fn lint(options: LintOptions<'_>) -> Result<Report, Error> {
    // Get CLOMonitor metadata
    let md = Metadata::from(options.root.join(METADATA_FILE))?;

    // Get Github metadata
    let gh_md = github::get_metadata(options.url).await?;

    // Async checks: documentation, best_practices, security, legal
    let (documentation, best_practices, security, legal) = tokio::try_join!(
        lint_documentation(options.root, options.url, &gh_md),
        lint_best_practices(options.root, options.url),
        lint_security(options.root, &gh_md),
        lint_legal(&gh_md),
    )?;

    Ok(Report {
        documentation,
        license: lint_license(options.root, &md, &gh_md)?,
        best_practices,
        security,
        legal,
    })
}

/// Run documentation checks and prepare the report's documentation section.
async fn lint_documentation(
    root: &Path,
    repo_url: &str,
    gh_md: &Repository,
) -> Result<Documentation, Error> {
    // Adopters
    let adopters = check::path::exists(Globs {
        root,
        patterns: ADOPTERS_FILE,
        case_sensitive: false,
    })? || check::content::matches(
        Globs {
            root,
            patterns: README_FILE,
            case_sensitive: true,
        },
        &*ADOPTERS_IN_README,
    )?;

    // Code of conduct
    let code_of_conduct =
        check::path::exists(Globs {
            root,
            patterns: CODE_OF_CONDUCT_FILE,
            case_sensitive: false,
        })? || check::content::matches(
            Globs {
                root,
                patterns: README_FILE,
                case_sensitive: true,
            },
            &*CODE_OF_CONDUCT_IN_README,
        )? || check::github::has_default_community_health_file(gh_md, "CODE_OF_CONDUCT.md").await?;

    // Contributing
    let contributing =
        check::path::exists(Globs {
            root,
            patterns: CONTRIBUTING_FILE,
            case_sensitive: false,
        })? || check::content::matches(
            Globs {
                root,
                patterns: README_FILE,
                case_sensitive: true,
            },
            &*CONTRIBUTING_IN_README,
        )? || check::github::has_default_community_health_file(gh_md, "CONTRIBUTING.md").await?;

    // Changelog
    let changelog =
        check::path::exists(Globs {
            root,
            patterns: CHANGELOG_FILE,
            case_sensitive: false,
        })? || check::content::matches(
            Globs {
                root,
                patterns: README_FILE,
                case_sensitive: true,
            },
            &*CHANGELOG_IN_README,
        )? || check::github::last_release_body_matches(repo_url, &*CHANGELOG_IN_GH_RELEASE).await?;

    // Governance
    let governance = check::path::exists(Globs {
        root,
        patterns: GOVERNANCE_FILE,
        case_sensitive: false,
    })? || check::content::matches(
        Globs {
            root,
            patterns: README_FILE,
            case_sensitive: true,
        },
        &*GOVERNANCE_IN_README,
    )?;

    // Maintainers
    let maintainers = check::path::exists(Globs {
        root,
        patterns: MAINTAINERS_FILE,
        case_sensitive: false,
    })? || check::content::matches(
        Globs {
            root,
            patterns: README_FILE,
            case_sensitive: true,
        },
        &*MAINTAINERS_IN_README,
    )?;

    // Readme
    let readme = check::path::exists(Globs {
        root,
        patterns: README_FILE,
        case_sensitive: true,
    })?;

    // Roadmap
    let roadmap = check::path::exists(Globs {
        root,
        patterns: ROADMAP_FILE,
        case_sensitive: false,
    })? || check::content::matches(
        Globs {
            root,
            patterns: README_FILE,
            case_sensitive: true,
        },
        &*ROADMAP_IN_README,
    )?;

    // Website
    let website = match &gh_md.homepage {
        Some(url) => !url.is_empty(),
        None => false,
    };

    Ok(Documentation {
        adopters: adopters.into(),
        code_of_conduct: code_of_conduct.into(),
        contributing: contributing.into(),
        changelog: changelog.into(),
        governance: governance.into(),
        maintainers: maintainers.into(),
        readme: readme.into(),
        roadmap: roadmap.into(),
        website: website.into(),
    })
}

/// Run license checks and prepare the report's license section.
fn lint_license(root: &Path, md: &Option<Metadata>, gh_md: &Repository) -> Result<License, Error> {
    // SPDX id
    let mut spdx_id = check::license::detect(Globs {
        root,
        patterns: LICENSE_FILE,
        case_sensitive: true,
    })?;
    if spdx_id.is_none() {
        if let Some(license) = &gh_md.license {
            if license.spdx_id != "NOASSERTION" {
                spdx_id = Some(license.spdx_id.to_owned());
            }
        }
    }

    // Approved
    let mut approved: Option<bool> = None;
    if let Some(spdx_id) = &spdx_id {
        approved = Some(check::license::is_approved(spdx_id))
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
        scanning_url = check::content::find(
            Globs {
                root,
                patterns: README_FILE,
                case_sensitive: true,
            },
            vec![&*FOSSA_URL, &*SNYK_URL],
        )?;
    }

    Ok(License {
        approved: (approved.unwrap_or(false), approved).into(),
        scanning: CheckResult::from_url(scanning_url),
        spdx_id: spdx_id.into(),
    })
}

/// Run best practices checks and prepare the report's best practices section.
async fn lint_best_practices(root: &Path, repo_url: &str) -> Result<BestPractices, Error> {
    // Artifact Hub badge
    let artifacthub_badge = check::content::matches(
        Globs {
            root,
            patterns: README_FILE,
            case_sensitive: true,
        },
        &*ARTIFACTHUB_BADGE_URL,
    )?;

    // Community meeting
    let community_meeting = check::content::matches(
        Globs {
            root,
            patterns: README_FILE,
            case_sensitive: true,
        },
        &*COMMUNITY_MEETING_TEXT,
    )?;

    // DCO
    let dco = check::git::commits_have_dco_signature(root).unwrap_or(false)
        || check::github::last_pr_has_dco_check(repo_url).await?;

    // OpenSSF badge
    let openssf_badge = check::content::matches(
        Globs {
            root,
            patterns: README_FILE,
            case_sensitive: true,
        },
        &*OPENSSF_BADGE_URL,
    )?;

    // Recent release
    let recent_release = check::github::has_recent_release(repo_url).await?;

    Ok(BestPractices {
        artifacthub_badge: artifacthub_badge.into(),
        community_meeting: community_meeting.into(),
        dco: dco.into(),
        openssf_badge: openssf_badge.into(),
        recent_release: recent_release.into(),
    })
}

/// Run security checks and prepare the report's security section.
async fn lint_security(root: &Path, gh_md: &Repository) -> Result<Security, Error> {
    // Security policy
    let security_policy =
        check::path::exists(Globs {
            root,
            patterns: SECURITY_POLICY_FILE,
            case_sensitive: false,
        })? || check::content::matches(
            Globs {
                root,
                patterns: README_FILE,
                case_sensitive: true,
            },
            &*SECURITY_POLICY_IN_README,
        )? || check::github::has_default_community_health_file(gh_md, "SECURITY.md").await?;

    Ok(Security {
        security_policy: security_policy.into(),
    })
}

/// Run legal checks and prepare the report's legal section.
async fn lint_legal(gh_md: &Repository) -> Result<Legal, Error> {
    // Trademark footer
    let mut trademark_footer: bool = false;
    if let Some(url) = &gh_md.homepage {
        if !url.is_empty() {
            trademark_footer = check::content::remote_matches(url, &*TRADEMARK_DISCLAIMER).await?;
        }
    }

    Ok(Legal {
        trademark_footer: trademark_footer.into(),
    })
}
