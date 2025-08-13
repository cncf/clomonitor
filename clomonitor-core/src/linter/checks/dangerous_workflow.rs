use anyhow::Result;

use crate::linter::{
    CheckSet,
    check::{CheckId, CheckInput, CheckOutput},
};

use super::datasource::scorecard;

/// Check identifier.
pub(crate) const ID: CheckId = "dangerous_workflow";

/// Check score weight.
pub(crate) const WEIGHT: usize = 2;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

/// Check main function.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    Ok(scorecard::get_check(&input.scorecard, ID).into())
}
