use crate::linter::{check::CheckInput, CheckId, CheckOutput, CheckSet};
use anyhow::{format_err, Result};

/// Check identifier.
pub(crate) const ID: CheckId = "self_assessment";

/// Check score weight.
pub(crate) const WEIGHT: usize = 2;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    if let Some(evidence_url) = input
        .security_insights
        .as_ref()
        .map_err(|e| format_err!("{e:?}"))?
        .as_ref()
        .and_then(|si| si.security_artifacts.as_ref())
        .and_then(|sa| sa.self_assessment.as_ref())
        .and_then(|sa| sa.evidence_url.as_ref())
        .and_then(|urls| urls.first())
    {
        return Ok(CheckOutput::passed().url(Some(evidence_url.clone())));
    }
    Ok(CheckOutput::not_passed())
}
