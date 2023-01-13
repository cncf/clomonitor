use super::util::github;
use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use anyhow::Result;
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};

/// Check identifier.
pub(crate) const ID: CheckId = "recent_release";

/// Check score weight.
pub(crate) const WEIGHT: usize = 3;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 2] = [CheckSet::Code, CheckSet::CodeLite];

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // Recent release (< 1 year old) in GitHub
    if let Some(latest_release) = github::latest_release(&input.gh_md) {
        let created_at = OffsetDateTime::parse(&latest_release.created_at, &Rfc3339)?;
        let one_year_ago = (OffsetDateTime::now_utc() - Duration::days(365)).unix_timestamp();
        if created_at.unix_timestamp() > one_year_ago {
            return Ok(CheckOutput::passed().url(Some(latest_release.url.clone())));
        }
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linter::{
        util::github::md::{
            MdRepository, MdRepositoryReleases, MdRepositoryReleasesNodes,
            MdRepositoryReleasesNodesReleaseAssets,
        },
        LinterInput,
    };
    use anyhow::format_err;

    #[test]
    fn not_passed_no_release_found() {
        assert_eq!(
            check(&CheckInput {
                li: &LinterInput::default(),
                cm_md: None,
                gh_md: MdRepository {
                    ..MdRepository::default()
                },
                scorecard: Err(format_err!("no scorecard available")),
            })
            .unwrap(),
            CheckOutput::not_passed(),
        );
    }

    #[test]
    fn not_passed_no_recent_release_found() {
        let two_years_ago = (OffsetDateTime::now_utc() - Duration::days(365 * 2))
            .format(&Rfc3339)
            .unwrap();

        assert_eq!(
            check(&CheckInput {
                li: &LinterInput::default(),
                cm_md: None,
                gh_md: MdRepository {
                    releases: MdRepositoryReleases {
                        nodes: Some(vec![Some(MdRepositoryReleasesNodes {
                            created_at: two_years_ago,
                            description: None,
                            is_prerelease: false,
                            release_assets: MdRepositoryReleasesNodesReleaseAssets { nodes: None },
                            url: "release_url".to_string(),
                        })]),
                    },
                    ..MdRepository::default()
                },
                scorecard: Err(format_err!("no scorecard available")),
            })
            .unwrap(),
            CheckOutput::not_passed(),
        );
    }

    #[test]
    fn passed_recent_release_found() {
        let one_week_ago = (OffsetDateTime::now_utc() - Duration::days(7))
            .format(&Rfc3339)
            .unwrap();

        assert_eq!(
            check(&CheckInput {
                li: &LinterInput::default(),
                cm_md: None,
                gh_md: MdRepository {
                    releases: MdRepositoryReleases {
                        nodes: Some(vec![Some(MdRepositoryReleasesNodes {
                            created_at: one_week_ago,
                            description: None,
                            is_prerelease: false,
                            release_assets: MdRepositoryReleasesNodesReleaseAssets { nodes: None },
                            url: "release_url".to_string(),
                        })]),
                    },
                    ..MdRepository::default()
                },
                scorecard: Err(format_err!("no scorecard available")),
            })
            .unwrap(),
            CheckOutput::passed().url(Some("release_url".to_string())),
        );
    }
}
