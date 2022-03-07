use lazy_static::lazy_static;
use regex::{Regex, RegexSet};

// Globs

#[rustfmt::skip]
pub(crate) static ADOPTERS_FILE: [&str; 2] = [
    "adopters*",
    "users.md",
];

#[rustfmt::skip]
pub(crate) static CODE_OF_CONDUCT_FILE: [&str; 2] = [
    "code*of*conduct.md",
    "docs/code*of*conduct.md",
];

#[rustfmt::skip]
pub(crate) static CONTRIBUTING_FILE: [&str; 2] = [
    "contributing*",
    "docs/contributing*",
];

#[rustfmt::skip]
pub(crate) static CHANGELOG_FILE: [&str; 1] = [
    "changelog*",
];

#[rustfmt::skip]
pub(crate) static GOVERNANCE_FILE: [&str; 2] = [
    "governance*",
    "docs/governance*",
];

#[rustfmt::skip]
pub(crate) static LICENSE_FILE: [&str; 2] = [
    "LICENSE*",
    "COPYING*",
];

#[rustfmt::skip]
pub(crate) static MAINTAINERS_FILE: [&str; 7] = [
    "maintainers*",
    "docs/maintainers*",
    "owners*",
    "docs/owners*",
    "codeowners*",
    "docs/codeowners*",
    ".github/codeowners*",
];

#[rustfmt::skip]
pub(crate) static README_FILE: [&str; 1] = [
    "README*",
];

#[rustfmt::skip]
pub(crate) static ROADMAP_FILE: [&str; 1] = [
    "roadmap*",
];

#[rustfmt::skip]
pub(crate) static SECURITY_POLICY_FILE: [&str; 3] = [
    "security*",
    "docs/security*",".github/security*",
];

// Regular expressions

lazy_static! {
    #[rustfmt::skip]
    pub(crate) static ref ADOPTERS_HEADER: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*adopters.*$",
        r"(?im)^adopters$",
    ]).expect("invalid exprs in ADOPTERS_HEADER");

    #[rustfmt::skip]
    pub(crate) static ref ARTIFACTHUB_BADGE_URL: RegexSet = RegexSet::new(vec![
        r"https://artifacthub.io/badge/repository/.*"
    ]).expect("invalid exprs in ARTIFACTHUB_BADGE_URL");

    #[rustfmt::skip]
    pub(crate) static ref CHANGELOG_HEADER: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*changelog.*$",
        r"(?im)^changelog$",
    ]).expect("invalid exprs in CHANGELOG_HEADER");

    #[rustfmt::skip]
    pub(crate) static ref CHANGELOG_RELEASE: RegexSet = RegexSet::new(vec![
        r"(?i)changelog",
    ]).expect("invalid exprs in CHANGELOG_RELEASE");

    #[rustfmt::skip]
    pub(crate) static ref CODE_OF_CONDUCT_HEADER: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*code of conduct.*$",
        r"(?im)^code of conduct$",
    ]).expect("invalid exprs in CODE_OF_CONDUCT_HEADER");

    #[rustfmt::skip]
    pub(crate) static ref COMMUNITY_MEETING_TEXT: RegexSet = RegexSet::new(vec![
        r"(?i)(community|developer|development) (call|event|meeting|session)",
        r"(?i)(weekly|biweekly|monthly) meeting",
        r"(?i)meeting minutes",
    ]).expect("invalid exprs in COMMUNITY_MEETING_TEXT");

    #[rustfmt::skip]
    pub(crate) static ref CONTRIBUTING_HEADER: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*contributing.*$",
        r"(?im)^contributing$",
    ]).expect("invalid exprs in CONTRIBUTING_HEADER");

    #[rustfmt::skip]
    pub(crate) static ref DCO: RegexSet = RegexSet::new(vec![
        r"DCO",
    ]).expect("invalid exprs in DCO");

    #[rustfmt::skip]
    pub(crate) static ref FOSSA_URL: Regex = Regex::new(
        r#"(https://app.fossa.(?:io|com)/projects/[^"'\)]+)"#
    ).expect("invalid exprs in FOSSA_URL");

    #[rustfmt::skip]
    pub(crate) static ref GOVERNANCE_HEADER: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*governance.*$",
        r"(?im)^governance$",
    ]).expect("invalid exprs in GOVERNANCE_HEADER");

    #[rustfmt::skip]
    pub(crate) static ref OPENSSF_BADGE_URL: RegexSet = RegexSet::new(vec![
        r"https://bestpractices.coreinfrastructure.org/projects/\d+",
    ]).expect("invalid exprs in OPENSSF_BADGE_URL");

    #[rustfmt::skip]
    pub(crate) static ref ROADMAP_HEADER: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*roadmap.*$",
        r"(?im)^roadmap$",
    ]).expect("invalid exprs in ROADMAP_HEADER");

    #[rustfmt::skip]
    pub(crate) static ref SECURITY_POLICY_HEADER: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*security.*$",
        r"(?im)^security$",
    ]).expect("invalid exprs in SECURITY_POLICY_HEADER");

    #[rustfmt::skip]
    pub(crate) static ref SNYK_URL: Regex = Regex::new(
        r#"(https://snyk.io/test/github/[^/]+/[^/"]+)"#
    ).expect("invalid exprs in SNYK_URL");

    #[rustfmt::skip]
    pub(crate) static ref TRADEMARK_FOOTER: RegexSet = RegexSet::new(vec![
        r"https://(?:w{3}\.)?linuxfoundation.org/trademark-usage",
    ]).expect("invalid exprs in TRADEMARK_FOOTER");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Regular expressions

    #[test]
    fn adopters_header_match() {
        assert!(ADOPTERS_HEADER.is_match("# Adopters"));
        assert!(ADOPTERS_HEADER.is_match(
            r"
...
## Project adopters and others
...
            "
        ));
        assert!(ADOPTERS_HEADER.is_match(
            r"
...
Adopters
--------
...
            "
        ));
    }

    #[test]
    fn artifacthub_badge_url_match() {
        assert!(ARTIFACTHUB_BADGE_URL.is_match("[![Artifact HUB](https://img.shields.io/endpoint?url=https://artifacthub.io/badge/repository/artifact-hub)](https://artifacthub.io/packages/helm/artifact-hub/artifact-hub)"));
    }

    #[test]
    fn changelog_header_match() {
        assert!(CHANGELOG_HEADER.is_match("# Changelog"));
        assert!(CHANGELOG_HEADER.is_match(
            r"
...
## Project changelog and others
...
            "
        ));
        assert!(CHANGELOG_HEADER.is_match(
            r"
...
Changelog
=========
...
            "
        ));
    }

    #[test]
    fn changelog_release_match() {
        assert!(CHANGELOG_RELEASE.is_match("# Changelog"));
        assert!(CHANGELOG_RELEASE.is_match("Below you can find the changelog"));
    }

    #[test]
    fn code_of_conduct_header_match() {
        assert!(CODE_OF_CONDUCT_HEADER.is_match("# Code of conduct"));
        assert!(CODE_OF_CONDUCT_HEADER.is_match(
            r"
...
## Project code of conduct and others
...
            "
        ));
        assert!(CODE_OF_CONDUCT_HEADER.is_match(
            r"
...
Code of Conduct
---------------
...
            "
        ));
    }

    #[test]
    fn community_meeting_text_match() {
        assert!(COMMUNITY_MEETING_TEXT.is_match("# Community meeting"));
        assert!(COMMUNITY_MEETING_TEXT.is_match("## Developer call"));
        assert!(COMMUNITY_MEETING_TEXT.is_match("development event"));
        assert!(COMMUNITY_MEETING_TEXT.is_match("community session"));
        assert!(COMMUNITY_MEETING_TEXT.is_match("the meeting minutes below"));
        assert!(COMMUNITY_MEETING_TEXT.is_match("Weekly meeting"));
    }

    #[test]
    fn contributing_header_match() {
        assert!(CONTRIBUTING_HEADER.is_match("# Contributing"));
        assert!(CONTRIBUTING_HEADER.is_match(
            r"
...
## Some stuff, contributing and others
...
            "
        ));
        assert!(CONTRIBUTING_HEADER.is_match(
            r"
...
Contributing
------------
...
            "
        ));
    }

    #[test]
    fn governance_header_match() {
        assert!(GOVERNANCE_HEADER.is_match("# Governance"));
        assert!(GOVERNANCE_HEADER.is_match(
            r"
...
## Project governance and others
...
            "
        ));
        assert!(GOVERNANCE_HEADER.is_match(
            r"
...
Governance
----------
...
            "
        ));
    }

    #[test]
    fn license_scanning_url_extract_fossa() {
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
    fn license_scanning_url_extract_snyk() {
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
    fn openssf_badge_url_match() {
        assert!(OPENSSF_BADGE_URL.is_match("[![CII Best Practices](https://bestpractices.coreinfrastructure.org/projects/4106/badge)](https://bestpractices.coreinfrastructure.org/projects/4106)"));
    }

    #[test]
    fn roadmap_header_match() {
        assert!(ROADMAP_HEADER.is_match("# Roadmap"));
        assert!(ROADMAP_HEADER.is_match(
            r"
...
## Project roadmap and others
...
            "
        ));
        assert!(ROADMAP_HEADER.is_match(
            r"
...
Roadmap
-------
...
            "
        ));
    }

    #[test]
    fn security_polity_header_match() {
        assert!(SECURITY_POLICY_HEADER.is_match("# Security"));
        assert!(SECURITY_POLICY_HEADER.is_match(
            r"
...
## Project security and others
...
            "
        ));
        assert!(SECURITY_POLICY_HEADER.is_match(
            r"
...
Security
--------
...
            "
        ));
    }

    #[test]
    fn trademark_footer_match() {
        assert!(TRADEMARK_FOOTER.is_match("https://www.linuxfoundation.org/trademark-usage"));
        assert!(TRADEMARK_FOOTER.is_match("https://linuxfoundation.org/trademark-usage"));
    }
}
