use anyhow::{Result, format_err};

use crate::linter::{CheckId, CheckOutput, CheckSet, check::CheckInput};

/// Check identifier.
pub(crate) const ID: CheckId = "dependencies_policy";

/// Check score weight.
pub(crate) const WEIGHT: usize = 2;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    if let Some(policy_url) = input
        .security_insights
        .as_ref()
        .map_err(|e| format_err!("{e:?}"))?
        .as_ref()
        .and_then(|si| si.dependency_policy_url())
    {
        return Ok(CheckOutput::passed().url(Some(policy_url.to_owned())));
    }
    Ok(CheckOutput::not_passed())
}
