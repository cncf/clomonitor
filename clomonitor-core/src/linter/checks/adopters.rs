use std::sync::LazyLock;

use anyhow::Result;
use regex::RegexSet;

use crate::linter::{
    CheckSet,
    check::{CheckId, CheckInput, CheckOutput},
};

use super::util::helpers::find_file_or_readme_ref;

/// Check identifier.
pub(crate) const ID: CheckId = "adopters";

/// Check score weight.
pub(crate) const WEIGHT: usize = 1;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Community];

/// Patterns used to locate a file in the repository.
const FILE_PATTERNS: [&str; 2] = ["adopters*", "users*"];

pub(crate) static README_REF: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        r"(?im)^#+.*adopters.*$",
        r"(?im)^adopters$",
        r"(?i)\[.*adopters.*\]\(.*\)",
    ])
    .expect("exprs in README_REF to be valid")
});

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README file
    find_file_or_readme_ref(input, &FILE_PATTERNS, &README_REF)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_ref_match() {
        assert!(README_REF.is_match("# Adopters"));
        assert!(README_REF.is_match(
            r"
...
## Project adopters and others
...
            "
        ));
        assert!(README_REF.is_match(
            r"
...
Adopters
--------
...
            "
        ));
        assert!(README_REF.is_match("[Project adopters](...)"));
    }
}
