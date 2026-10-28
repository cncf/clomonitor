use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;
use reqwest::Url;

use crate::linter::{
    CheckSet,
    check::{CheckId, CheckInput, CheckOutput},
};

use super::util::{content, helpers::readme_globs};

/// Check identifier.
pub(crate) const ID: CheckId = "license_scanning";

/// Check score weight.
pub(crate) const WEIGHT: usize = 5;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

pub(crate) static FOSSA_URL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(https://app.fossa.(?:io|com)/projects/[^"'\)\s]+)"#)
        .expect("exprs in FOSSA_URL to be valid")
});

pub(crate) static SNYK_URL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(https://snyk.io/test/github/[^/\s]+/[^/"'\)\s]+)"#)
        .expect("exprs in SNYK_URL to be valid")
});

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput<String>> {
    // Scanning url in metadata file
    if let Some(url) = input
        .cm_md
        .as_ref()
        .and_then(|md| md.license_scanning.as_ref())
        .and_then(|ls| ls.url.as_ref())
    {
        return Ok(passed(url.clone()));
    }

    // Reference in README file
    if let Some(url) = content::find(&readme_globs(&input.li.root), &[&FOSSA_URL, &SNYK_URL])? {
        return Ok(passed(url));
    }

    Ok(CheckOutput::not_passed())
}

/// Build a passed check output for the license scanning url provided.
fn passed(url: String) -> CheckOutput<String> {
    let tool = tool(&url).to_string();
    CheckOutput::passed().url(Some(url)).value(Some(tool))
}

/// Identify the license scanning tool from the url provided.
fn tool(url: &str) -> &'static str {
    let host = Url::parse(url)
        .ok()
        .and_then(|url| url.host_str().map(str::to_lowercase));
    let is_domain = |domain: &str| {
        host.as_deref()
            .is_some_and(|host| host == domain || host.ends_with(&format!(".{domain}")))
    };

    if is_domain("fossa.com") || is_domain("fossa.io") {
        "FOSSA"
    } else if is_domain("snyk.io") {
        "Snyk"
    } else {
        "Other"
    }
}

#[cfg(test)]
mod tests {
    use anyhow::format_err;

    use crate::linter::{
        LinterInput,
        datasource::github::md::MdRepository,
        metadata::{LicenseScanning, Metadata},
    };

    use super::*;

    #[test]
    fn not_passed_no_md_found() {
        assert_eq!(
            check(&CheckInput {
                li: &LinterInput::default(),
                cm_md: None,
                gh_md: MdRepository::default(),
                afdocs: None,
                scorecard: Err(format_err!("no scorecard available")),
                security_insights: Ok(None),
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
                    agent_readiness: None,
                    exemptions: None,
                    license_scanning: None,
                }),
                gh_md: MdRepository::default(),
                afdocs: None,
                scorecard: Err(format_err!("no scorecard available")),
                security_insights: Ok(None),
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
                    agent_readiness: None,
                    exemptions: None,
                    license_scanning: Some(LicenseScanning {
                        url: Some("license_scanning_url".to_string()),
                    }),
                }),
                gh_md: MdRepository::default(),
                afdocs: None,
                scorecard: Err(format_err!("no scorecard available")),
                security_insights: Ok(None),
            })
            .unwrap(),
            CheckOutput::passed()
                .url(Some("license_scanning_url".to_string()))
                .value(Some("Other".to_string())),
        );
    }

    #[test]
    fn passed_license_scanning_info_found_known_tool() {
        assert_eq!(
            check(&CheckInput {
                li: &LinterInput::default(),
                cm_md: Some(Metadata {
                    agent_readiness: None,
                    exemptions: None,
                    license_scanning: Some(LicenseScanning {
                        url: Some("https://app.snyk.io/org/org/projects".to_string()),
                    }),
                }),
                gh_md: MdRepository::default(),
                afdocs: None,
                scorecard: Err(format_err!("no scorecard available")),
                security_insights: Ok(None),
            })
            .unwrap(),
            CheckOutput::passed()
                .url(Some("https://app.snyk.io/org/org/projects".to_string()))
                .value(Some("Snyk".to_string())),
        );
    }

    #[test]
    fn tool_fossa() {
        assert_eq!(tool("https://app.fossa.com/projects/custom%2B162"), "FOSSA");
        assert_eq!(
            tool("https://app.fossa.io/projects/git%2Bgithub.com"),
            "FOSSA"
        );
        assert_eq!(tool("https://fossa.com/projects"), "FOSSA");
    }

    #[test]
    fn tool_snyk() {
        assert_eq!(tool("https://snyk.io/test/github/username/repo"), "Snyk");
        assert_eq!(tool("https://APP.SNYK.IO/org/org/projects"), "Snyk");
    }

    #[test]
    fn tool_other() {
        assert_eq!(tool("https://license-scanning-results.url"), "Other");
        assert_eq!(tool("https://notfossa.com/projects"), "Other");
        assert_eq!(tool("https://fossa.com.example.org/projects"), "Other");
        assert_eq!(tool("license_scanning_url"), "Other");
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
    fn fossa_url_extract_md_ref() {
        assert_eq!(
            FOSSA_URL
                .captures(
                    "[fossa]: https://app.fossa.io/projects/git%2Bgithub.com%2Fjaegertracing%2Fjaeger?ref=badge_shield\n[openssf]:",
                )
                .unwrap()[1]
                .to_string(),
            "https://app.fossa.io/projects/git%2Bgithub.com%2Fjaegertracing%2Fjaeger?ref=badge_shield"
        );
    }

    #[test]
    fn fossa_url_extract_rst() {
        assert_eq!(
            FOSSA_URL
                .captures(
                    "    :target: https://app.fossa.com/projects/custom%2B162%2Fgit%40github.com%3Acilium%2Fcilium.git?ref=badge_shield\n\n.. |gateway-api|",
                )
                .unwrap()[1]
                .to_string(),
            "https://app.fossa.com/projects/custom%2B162%2Fgit%40github.com%3Acilium%2Fcilium.git?ref=badge_shield"
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

    #[test]
    fn snyk_url_extract_md_ref() {
        assert_eq!(
            SNYK_URL
                .captures("[snyk]: https://snyk.io/test/github/username/repo\n[next-badge]:",)
                .unwrap()[1]
                .to_string(),
            "https://snyk.io/test/github/username/repo"
        );
    }

    #[test]
    fn snyk_url_extract_md_link() {
        assert_eq!(
            SNYK_URL
                .captures("We use [Snyk](https://snyk.io/test/github/username/repo) to test")
                .unwrap()[1]
                .to_string(),
            "https://snyk.io/test/github/username/repo"
        );
    }
}
