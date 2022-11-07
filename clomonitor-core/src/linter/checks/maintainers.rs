use super::util::helpers::find_file_or_readme_ref;
use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::RegexSet;

/// Check identifier.
pub(crate) const ID: CheckId = "maintainers";

/// Check score weight.
pub(crate) const WEIGHT: usize = 3;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 2] = [CheckSet::Code, CheckSet::CodeLite];

/// Patterns used to locate a file in the repository.
pub(crate) static FILE_PATTERNS: [&str; 7] = [
    "maintainers*",
    "docs/maintainers*",
    "owners*",
    "docs/owners*",
    "codeowners*",
    ".github/codeowners*",
    "docs/codeowners*",
];

lazy_static! {
    #[rustfmt::skip]
    static ref README_REF: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*maintainers.*$",
        r"(?im)^maintainers$",
        r"(?i)\[.*maintainers.*\]\(.*\)",
    ]).expect("exprs in README_REF to be valid");
}

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
        assert!(README_REF.is_match("# Maintainers"));
        assert!(README_REF.is_match(
            r"
...
## Project maintainers and others
...
            "
        ));
        assert!(README_REF.is_match(
            r"
...
Maintainers
----------
...
            "
        ));
        assert!(README_REF.is_match("[Project maintainers](...)"));
    }
}
