use super::util::helpers::readme_matches;
use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::RegexSet;

/// Check identifier.
pub(crate) const ID: CheckId = "community_meeting";

/// Check score weight.
pub(crate) const WEIGHT: usize = 3;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Community];

lazy_static! {
    #[rustfmt::skip]
    static ref README_REF: RegexSet = RegexSet::new([
        r"(?im)^#+.*meeting.*$",
        r"(?i)(community|developer|development) \[?(call|event|meeting|session)",
        r"(?i)(weekly|biweekly|monthly) \[?meeting",
        r"(?i)meeting minutes",
    ]).expect("exprs in README_REF to be valid");
}

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // Reference in README file
    if readme_matches(&input.li.root, &README_REF)? {
        return Ok(CheckOutput::passed());
    };

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_ref_match() {
        assert!(README_REF.is_match("# Project Status Meetings"));
        assert!(README_REF.is_match("# Community meeting"));
        assert!(README_REF.is_match("# Community [meeting](...)"));
        assert!(README_REF.is_match("## Developer call"));
        assert!(README_REF.is_match("development event"));
        assert!(README_REF.is_match("community session"));
        assert!(README_REF.is_match("the meeting minutes below"));
        assert!(README_REF.is_match("Weekly meeting"));
    }
}
