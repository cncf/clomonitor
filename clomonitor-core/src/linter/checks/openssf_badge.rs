use super::util::helpers::readme_capture;
use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

/// Check identifier.
pub(crate) const ID: CheckId = "openssf_badge";

/// Check score weight.
pub(crate) const WEIGHT: usize = 5;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

lazy_static! {
    #[rustfmt::skip]
    static ref OPENSSF_URL: Regex = Regex::new(
        r"(https://www.bestpractices.dev/projects/\d+)",
    ).expect("exprs in OPENSSF_URL to be valid");

    #[rustfmt::skip]
    static ref OPENSSF_URL_LEGACY: Regex = Regex::new(
        r"(https://bestpractices.coreinfrastructure.org/projects/\d+)",
    ).expect("exprs in OPENSSF_URL_LEGACY to be valid");
}

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // Reference in README file
    if let Some(url) = readme_capture(&input.li.root, &[&OPENSSF_URL, &OPENSSF_URL_LEGACY])? {
        return Ok(CheckOutput::passed().url(Some(url)));
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openssf_url_extract() {
        assert_eq!(
            OPENSSF_URL.captures("[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/4106/badge)](https://www.bestpractices.dev/projects/4106)").unwrap()[1].to_string(),
            "https://www.bestpractices.dev/projects/4106"
        );
    }

    #[test]
    fn openssf_url_legacy_extract() {
        assert_eq!(
            OPENSSF_URL_LEGACY.captures("[![CII Best Practices](https://bestpractices.coreinfrastructure.org/projects/4106/badge)](https://bestpractices.coreinfrastructure.org/projects/4106)").unwrap()[1].to_string(),
            "https://bestpractices.coreinfrastructure.org/projects/4106"
        );
    }
}
