use anyhow::Result;

use crate::linter::{
    CheckSet,
    check::{CheckId, CheckInput, CheckOutput},
};

use super::datasource::afdocs;

/// Check identifier.
pub(crate) const ID: CheckId = "content_structure";

/// Check score weight.
pub(crate) const WEIGHT: usize = 4;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Community];

/// Check main function.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    Ok(afdocs::get_category(input.afdocs.as_ref(), ID).into())
}
