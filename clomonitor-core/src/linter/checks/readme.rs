//! Patterns used to locate a file in the repository.

use anyhow::Result;

use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};

use super::{
    datasource::github,
    util::{helpers::readme_globs, path},
};

/// Check identifier.
pub(crate) const ID: CheckId = "readme";

/// Check score weight.
pub(crate) const WEIGHT: usize = 10;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 4] = [
    CheckSet::Code,
    CheckSet::CodeLite,
    CheckSet::Community,
    CheckSet::Docs,
];

/// Patterns used to locate a file in the repository.
pub(crate) static FILE_PATTERNS: [&str; 3] = ["README*", ".github/README*", "docs/README*"];

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo
    if let Some(path) = path::find(&readme_globs(&input.li.root))? {
        let url = github::build_url(
            &path,
            &input.gh_md.owner.login,
            &input.gh_md.name,
            &github::default_branch(input.gh_md.default_branch_ref.as_ref()),
        );
        return Ok(CheckOutput::passed().url(Some(url)));
    }

    Ok(CheckOutput::not_passed())
}
