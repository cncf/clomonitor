use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use anyhow::Result;
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};

/// Check identifier.
pub(crate) const ID: CheckId = "github_discussions";

/// Check score weight.
pub(crate) const WEIGHT: usize = 0;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Community];

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    if let Some(latest_discussion) = input
        .gh_md
        .discussions
        .nodes
        .as_ref()
        .and_then(|nodes| nodes.iter().flatten().next())
    {
        let created_at = OffsetDateTime::parse(&latest_discussion.created_at, &Rfc3339)?;
        let one_year_ago = (OffsetDateTime::now_utc() - Duration::days(365)).unix_timestamp();
        if created_at.unix_timestamp() > one_year_ago {
            return Ok(CheckOutput::passed().url(Some(latest_discussion.url.clone())));
        }
    }
    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linter::{
        util::github::md::{MdRepository, MdRepositoryDiscussions, MdRepositoryDiscussionsNodes},
        LinterInput,
    };
    use anyhow::format_err;

    #[test]
    fn not_passed_no_discussion_found() {
        assert_eq!(
            check(&CheckInput {
                li: &LinterInput::default(),
                cm_md: None,
                gh_md: MdRepository {
                    discussions: MdRepositoryDiscussions { nodes: None },
                    ..MdRepository::default()
                },
                scorecard: Err(format_err!("no scorecard available")),
            })
            .unwrap(),
            CheckOutput::not_passed(),
        );
    }

    #[test]
    fn not_passed_no_recent_discussion_found() {
        let two_years_ago = (OffsetDateTime::now_utc() - Duration::days(365 * 2))
            .format(&Rfc3339)
            .unwrap();

        assert_eq!(
            check(&CheckInput {
                li: &LinterInput::default(),
                cm_md: None,
                gh_md: MdRepository {
                    discussions: MdRepositoryDiscussions {
                        nodes: Some(vec![Some(MdRepositoryDiscussionsNodes {
                            created_at: two_years_ago,
                            url: "discussion_url".to_string(),
                        })])
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
    fn passed_recent_discussion_found() {
        let one_week_ago = (OffsetDateTime::now_utc() - Duration::days(7))
            .format(&Rfc3339)
            .unwrap();

        assert_eq!(
            check(&CheckInput {
                li: &LinterInput::default(),
                cm_md: None,
                gh_md: MdRepository {
                    discussions: MdRepositoryDiscussions {
                        nodes: Some(vec![Some(MdRepositoryDiscussionsNodes {
                            created_at: one_week_ago,
                            url: "discussion_url".to_string(),
                        })])
                    },
                    ..MdRepository::default()
                },
                scorecard: Err(format_err!("no scorecard available")),
            })
            .unwrap(),
            CheckOutput::passed().url(Some("discussion_url".to_string())),
        );
    }
}
