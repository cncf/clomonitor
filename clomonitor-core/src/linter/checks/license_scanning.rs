use super::util::{content, helpers::readme_globs};
use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

/// Check identifier.
pub(crate) const ID: CheckId = "license_scanning";

/// Check score weight.
pub(crate) const WEIGHT: usize = 5;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

lazy_static! {
    #[rustfmt::skip]
    pub(crate) static ref FOSSA_URL: Regex = Regex::new(
        r#"(https://app.fossa.(?:io|com)/projects/[^"'\)]+)"#
    ).expect("exprs in FOSSA_URL to be valid");

    #[rustfmt::skip]
    pub(crate) static ref SNYK_URL: Regex = Regex::new(
        r#"(https://snyk.io/test/github/[^/]+/[^/"]+)"#
    ).expect("exprs in SNYK_URL to be valid");
}

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // Scanning url in metadata file
    if let Some(url) = input
        .cm_md
        .as_ref()
        .and_then(|md| md.license_scanning.as_ref())
        .and_then(|ls| ls.url.as_ref())
    {
        return Ok(CheckOutput::passed().url(Some(url.to_owned())));
    }

    // Reference in README file
    if let Some(url) = content::find(&readme_globs(&input.li.root), &[&FOSSA_URL, &SNYK_URL])? {
        return Ok(CheckOutput::passed().url(Some(url)));
    };

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linter::{
        metadata::{LicenseScanning, Metadata},
        util::github::md::MdRepository,
        LinterInput,
    };
    use anyhow::format_err;

    #[test]
    fn not_passed_no_md_found() {
        assert_eq!(
            check(&CheckInput {
                li: &LinterInput::default(),
                cm_md: None,
                gh_md: MdRepository::default(),
                scorecard: Err(format_err!("no scorecard available")),
            })
            .unwrap(),
            CheckOutput::not_passed(),
        );
    }

    #[test]
    fn not_passed_no_license_scanning_info_found() {
        assert_eq!(
            check(&CheckInput {
                li: &LinterInput::default(),
                cm_md: Some(Metadata {
                    exemptions: None,
                    license_scanning: None,
                }),
                gh_md: MdRepository::default(),
                scorecard: Err(format_err!("no scorecard available")),
            })
            .unwrap(),
            CheckOutput::not_passed(),
        );
    }

    #[test]
    fn passed_license_scanning_info_found() {
        assert_eq!(
            check(&CheckInput {
                li: &LinterInput::default(),
                cm_md: Some(Metadata {
                    exemptions: None,
                    license_scanning: Some(LicenseScanning {
                        url: Some("license_scanning_url".to_string()),
                    }),
                }),
                gh_md: MdRepository::default(),
                scorecard: Err(format_err!("no scorecard available")),
            })
            .unwrap(),
            CheckOutput::passed().url(Some("license_scanning_url".to_string())),
        );
    }

    #[test]
    fn fossa_url_extract() {
        assert_eq!(
            FOSSA_URL.captures("[![Licenses](https://app.fossa.io/api/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fartifacthub%2Fhub.svg?type=shield)](https://app.fossa.io/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fartifacthub%2Fhub?ref=badge_shield)").unwrap()[1].to_string(),
            "https://app.fossa.io/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fartifacthub%2Fhub?ref=badge_shield"
        );
        assert_eq!(
            FOSSA_URL.captures(r#"<a href="https://app.fossa.io/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fkedacore%2Fkeda?ref=badge_shield"><img src="https://app.fossa.io/api/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fkedacore%2Fkeda.svg?type=shield"></a>"#).unwrap()[1].to_string(),
            "https://app.fossa.io/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fkedacore%2Fkeda?ref=badge_shield"
        );
    }

    #[test]
    fn snyk_url_extract() {
        assert_eq!(
            SNYK_URL.captures("[![Known Vulnerabilities](https://snyk.io/test/github/{username}/{repo}/badge.svg)](https://snyk.io/test/github/{username}/{repo})").unwrap()[1].to_string(),
            "https://snyk.io/test/github/{username}/{repo}"
        );
        assert_eq!(
            SNYK_URL
                .captures(r#"<a href="https://snyk.io/test/github/{username}/{repo}">"#)
                .unwrap()[1]
                .to_string(),
            "https://snyk.io/test/github/{username}/{repo}"
        );
    }
}
