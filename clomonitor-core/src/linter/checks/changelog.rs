use std::sync::LazyLock;

use anyhow::Result;
use regex::RegexSet;

use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};

use super::{datasource::github, util::helpers::find_file_or_readme_ref};

/// Check identifier.
pub(crate) const ID: CheckId = "changelog";

/// Check score weight.
pub(crate) const WEIGHT: usize = 1;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

/// Patterns used to locate a file in the repository.
pub(crate) static FILE_PATTERNS: [&str; 1] = ["changelog*"];

static README_REF: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        r"(?im)^#+.*changelog.*$",
        r"(?im)^changelog$",
        r"(?i)\[.*changelog.*\]\(.*\)",
    ])
    .expect("exprs in README_REF to be valid")
});

static RELEASE_REF: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([r"(?i)changelog", r"(?i)changes"]).expect("exprs in RELEASE_REF to be valid")
});

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README file
    let output = find_file_or_readme_ref(input, &FILE_PATTERNS, &README_REF)?;
    if output.passed {
        return Ok(output);
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
