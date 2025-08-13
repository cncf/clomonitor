use std::sync::LazyLock;

use anyhow::Result;
use regex::RegexSet;

use crate::linter::{
    CheckSet,
    check::{CheckId, CheckInput, CheckOutput},
};

use super::datasource::github;

/// Check identifier.
pub(crate) const ID: CheckId = "cla";

/// Check score weight.
pub(crate) const WEIGHT: usize = 1;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 2] = [CheckSet::Code, CheckSet::CodeLite];

static CHECK_REF: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        r"(?i)cncf-cla",
        r"(?i)cla/linuxfoundation",
        r"(?i)easycla",
        r"(?i)license/cla",
        r"(?i)cla/google",
    ])
    .expect("exprs in CHECK_REF to be valid")
});

/// Check main function.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // CLA check in Github
    if github::has_check(&input.gh_md, &CHECK_REF) {
        return Ok(CheckOutput::passed());
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ref_match() {
        assert!(CHECK_REF.is_match(r"EasyCLA"));
        assert!(CHECK_REF.is_match(r"cncf-cla"));
        assert!(CHECK_REF.is_match(r"cla/linuxfoundation"));
        assert!(CHECK_REF.is_match(r"license/cla"));
        assert!(CHECK_REF.is_match(r"cla/google"));
    }
}
