use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use anyhow::Result;

/// Check identifier.
pub(crate) const ID: CheckId = "website";

/// Check score weight.
pub(crate) const WEIGHT: usize = 4;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Community];

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // Website in Github
    if let Some(url) = &input.gh_md.homepage_url {
        if !url.is_empty() {
            return Ok(CheckOutput::passed().url(Some(url.to_string())));
        }
    }

    Ok(CheckOutput::not_passed())
}
