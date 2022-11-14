use super::util::helpers::find_file_or_readme_ref;
use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::RegexSet;

/// Check identifier.
pub(crate) const ID: CheckId = "security_policy";

/// Check score weight.
pub(crate) const WEIGHT: usize = 3;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 2] = [CheckSet::Code, CheckSet::Community];

/// Patterns used to locate a file in the repository.
static FILE_PATTERNS: [&str; 3] = ["security*", ".github/security*", "docs/security*"];

lazy_static! {
    #[rustfmt::skip]
    static ref README_REF: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*security.*$",
        r"(?im)^security$",
        r"(?i)\[.*security.*\]\(.*\)",
    ]).expect("exprs in README_REF to be valid");
}

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README file
    let r = find_file_or_readme_ref(input, &FILE_PATTERNS, &README_REF)?;
    if r.passed {
        return Ok(r);
    }

    // File in Github (default community health file, for example)
    if let Some(url) = input.gh_md.security_policy_url.as_ref() {
        return Ok(CheckOutput::passed().url(Some(url.to_owned())));
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_ref_match() {
        assert!(README_REF.is_match("# Security"));
        assert!(README_REF.is_match(
            r"
...
## Project security and others
...
            "
        ));
        assert!(README_REF.is_match(
            r"
...
Security
--------
...
            "
        ));
        assert!(README_REF.is_match("[Project security policy](...)"));
    }
}
