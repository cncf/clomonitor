use std::sync::LazyLock;

use anyhow::Result;
use regex::RegexSet;

use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};

use super::util::helpers::find_file_or_readme_ref;

/// Check identifier.
pub(crate) const ID: CheckId = "code_of_conduct";

/// Check score weight.
pub(crate) const WEIGHT: usize = 2;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Community];

/// Patterns used to locate a file in the repository.
pub(crate) static FILE_PATTERNS: [&str; 3] = [
    "code*of*conduct*",
    ".github/code*of*conduct*",
    "docs/code*of*conduct*",
];

static README_REF: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        r"(?im)^#+.*code of conduct.*$",
        r"(?im)^code of conduct$",
        r"(?i)\[.*code of conduct.*\]\(.*\)",
    ])
    .expect("exprs in README_REF to be valid")
});

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README file
    let output = find_file_or_readme_ref(input, &FILE_PATTERNS, &README_REF)?;
    if output.passed {
        return Ok(output);
    }

    // File in Github (default community health file, for example)
    if let Some(coc) = &input.gh_md.code_of_conduct
        && coc.url.is_some()
    {
        return Ok(CheckOutput::passed().url(coc.url.clone()));
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_ref_match() {
        assert!(README_REF.is_match("# Code of conduct"));
        assert!(README_REF.is_match(
            r"
...
## Project code of conduct and others
...
            "
        ));
        assert!(README_REF.is_match(
            r"
...
Code of Conduct
---------------
...
            "
        ));
        assert!(README_REF.is_match("[code of conduct](...)"));
    }
}
