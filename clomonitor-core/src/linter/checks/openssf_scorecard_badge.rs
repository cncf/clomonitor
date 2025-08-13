use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;

use crate::linter::{
    CheckSet,
    check::{CheckId, CheckInput, CheckOutput},
};

use super::util::helpers::readme_capture;

/// Check identifier.
pub(crate) const ID: CheckId = "openssf_scorecard_badge";

/// Check score weight.
pub(crate) const WEIGHT: usize = 5;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

static OPENSSF_SCORECARD_URL_OLD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(https://api.securityscorecards.dev/projects/github.com/[^/]+/[^/]+)/badge")
        .expect("exprs in OPENSSF_SCORECARD_URL_OLD to be valid")
});

static OPENSSF_SCORECARD_URL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(https://api.scorecard.dev/projects/github.com/[^/]+/[^/]+)/badge")
        .expect("exprs in OPENSSF_SCORECARD_URL to be valid")
});

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // Reference in README file
    if let Some(url) = readme_capture(
        &input.li.root,
        &[&OPENSSF_SCORECARD_URL, &OPENSSF_SCORECARD_URL_OLD],
    )? {
        return Ok(CheckOutput::passed().url(Some(url)));
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_openssf_scorecard_url_extract() {
        assert_eq!(
            OPENSSF_SCORECARD_URL_OLD.captures("[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/owner/repo/badge)](https://api.securityscorecards.dev/projects/github.com/owner/repo)").unwrap()[1].to_string(),
            "https://api.securityscorecards.dev/projects/github.com/owner/repo"
        );
    }
    #[test]
    fn new_old_openssf_scorecard_url_extract() {
        assert_eq!(
            OPENSSF_SCORECARD_URL.captures("[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/owner/repo/badge)](https://scorecard.dev/viewer/?uri=github.com/owner/repo)").unwrap()[1].to_string(),
            "https://api.scorecard.dev/projects/github.com/owner/repo"
        );
    }
}
