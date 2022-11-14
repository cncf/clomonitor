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
