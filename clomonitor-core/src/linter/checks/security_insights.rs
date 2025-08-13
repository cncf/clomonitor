use std::path::Path;

use anyhow::{Result, format_err};

use crate::linter::{CheckId, CheckOutput, CheckSet, check::CheckInput};

use super::datasource::{github, security_insights::SECURITY_INSIGHTS_MANIFEST_FILE};

/// Check identifier.
pub(crate) const ID: CheckId = "security_insights";

/// Check score weight.
pub(crate) const WEIGHT: usize = 1;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

/// Check main function.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    let output = match input
        .security_insights
        .as_ref()
        .map_err(|e| format_err!("{e:?}"))?
    {
        Some(_) => {
            let url = github::build_url(
                Path::new(SECURITY_INSIGHTS_MANIFEST_FILE),
                &input.gh_md.owner.login,
                &input.gh_md.name,
                &github::default_branch(input.gh_md.default_branch_ref.as_ref()),
            );
            CheckOutput::passed().url(Some(url))
        }
        None => CheckOutput::not_passed(),
    };
    Ok(output)
}
