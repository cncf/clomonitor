use super::util::helpers::readme_matches;
use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::RegexSet;

/// Check identifier.
pub(crate) const ID: CheckId = "slack_presence";

/// Check score weight.
pub(crate) const WEIGHT: usize = 0;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Community];

lazy_static! {
    #[rustfmt::skip]
    static ref README_REF: RegexSet = RegexSet::new(vec![
        r"(?i)https?://cloud-native.slack.com",
        r"(?i)https?://slack.cncf.io",
        r"(?i)https?://kubernetes.slack.com",
        r"(?i)https?://slack.k8s.io",
    ]).expect("exprs in README_REF to be valid");
}

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // Reference in README file
    if readme_matches(&input.li.root, &README_REF)? {
        return Ok(CheckOutput::passed());
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_ref_match() {
        assert!(README_REF.is_match("Joining [CNCF slack](https://cloud-native.slack.com)"));
        assert!(README_REF.is_match("Visit [http://slack.cncf.io/](http://slack.cncf.io/)"));
        assert!(README_REF.is_match("[KEDA](https://kubernetes.slack.com/messages/CKZJ36A5D)"));
        assert!(README_REF.is_match("[Kubernetes Slack](https://slack.k8s.io/)"));
    }
}
