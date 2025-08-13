use anyhow::Result;
use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};

use crate::linter::{
    CheckSet,
    check::{CheckId, CheckInput, CheckOutput},
};

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
    use anyhow::format_err;

    use crate::linter::{
        LinterInput,
        datasource::github::md::{
            MdRepository, MdRepositoryDiscussions, MdRepositoryDiscussionsNodes,
        },
    };

    use super::*;

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
                security_insights: Ok(None),
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
                security_insights: Ok(None),
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
                security_insights: Ok(None),
            })
            .unwrap(),
            CheckOutput::passed().url(Some("discussion_url".to_string())),
        );
    }
}
