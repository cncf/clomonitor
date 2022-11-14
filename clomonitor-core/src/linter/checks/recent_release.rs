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
