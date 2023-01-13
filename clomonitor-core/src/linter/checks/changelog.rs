use super::util::{github, helpers::find_file_or_readme_ref};
use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::RegexSet;

/// Check identifier.
pub(crate) const ID: CheckId = "changelog";

/// Check score weight.
pub(crate) const WEIGHT: usize = 1;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

/// Patterns used to locate a file in the repository.
pub(crate) static FILE_PATTERNS: [&str; 1] = ["changelog*"];

lazy_static! {
    #[rustfmt::skip]
    static ref README_REF: RegexSet = RegexSet::new([
        r"(?im)^#+.*changelog.*$",
        r"(?im)^changelog$",
        r"(?i)\[.*changelog.*\]\(.*\)",
    ]).expect("exprs in README_REF to be valid");

    #[rustfmt::skip]
    static ref RELEASE_REF: RegexSet = RegexSet::new([
        r"(?i)changelog",
        r"(?i)changes",
    ]).expect("exprs in RELEASE_REF to be valid");
}

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README file
    let r = find_file_or_readme_ref(input, &FILE_PATTERNS, &README_REF)?;
    if r.passed {
        return Ok(r);
    }

    // Reference in last release
    if github::latest_release_description_matches(&input.gh_md, &RELEASE_REF) {
        return Ok(CheckOutput::passed());
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_ref_match() {
        assert!(README_REF.is_match("# Changelog"));
        assert!(README_REF.is_match(
            r"
...
## Project changelog and others
...
            "
        ));
        assert!(README_REF.is_match(
            r"
...
Changelog
=========
...
            "
        ));
        assert!(README_REF.is_match("[Project changelog](...)"));
    }

    #[test]
    fn release_ref_match() {
        assert!(RELEASE_REF.is_match("# Changelog"));
        assert!(RELEASE_REF.is_match("Below you can find the changelog"));
    }
}
