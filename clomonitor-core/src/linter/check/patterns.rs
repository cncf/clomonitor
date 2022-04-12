use lazy_static::lazy_static;
use regex::{Regex, RegexSet};

// Globs

#[rustfmt::skip]
pub(crate) static ADOPTERS_FILE: [&str; 2] = [
    "adopters*",
    "users*",
];

#[rustfmt::skip]
pub(crate) static CHANGELOG_FILE: [&str; 1] = [
    "changelog*",
];

#[rustfmt::skip]
pub(crate) static CODE_OF_CONDUCT_FILE: [&str; 3] = [
    "code*of*conduct*",
    ".github/code*of*conduct*",
    "docs/code*of*conduct*",
];

#[rustfmt::skip]
pub(crate) static CONTRIBUTING_FILE: [&str; 3] = [
    "contributing*",
    ".github/contributing*",
    "docs/contributing*",
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
    ".github/codeowners*",
    "docs/codeowners*",
];

#[rustfmt::skip]
pub(crate) static README_FILE: [&str; 3] = [
    "README*",
    ".github/README*",
    "docs/README*",
];

#[rustfmt::skip]
pub(crate) static ROADMAP_FILE: [&str; 1] = [
    "roadmap*",
];

#[rustfmt::skip]
pub(crate) static SECURITY_POLICY_FILE: [&str; 3] = [
    "security*",
    ".github/security*",
    "docs/security*",
];

// Regular expressions

lazy_static! {
    #[rustfmt::skip]
    pub(crate) static ref ADOPTERS_IN_README: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*adopters.*$",
        r"(?im)^adopters$",
        r"(?i)\[.*adopters.*\]\(.*\)",
    ]).expect("invalid exprs in ADOPTERS_IN_README");

    #[rustfmt::skip]
    pub(crate) static ref ARTIFACTHUB_URL: Regex = Regex::new(
        r#"(https://artifacthub.io/packages/[^"'\)]+)"#
    ).expect("invalid exprs in ARTIFACTHUB_URL");

    #[rustfmt::skip]
    pub(crate) static ref CHANGELOG_IN_GH_RELEASE: RegexSet = RegexSet::new(vec![
        r"(?i)changelog",
        r"(?i)changes",
    ]).expect("invalid exprs in CHANGELOG_IN_GH_RELEASE");

    #[rustfmt::skip]
    pub(crate) static ref CHANGELOG_IN_README: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*changelog.*$",
        r"(?im)^changelog$",
        r"(?i)\[.*changelog.*\]\(.*\)",
    ]).expect("invalid exprs in CHANGELOG_IN_README");

    #[rustfmt::skip]
    pub(crate) static ref CLA_IN_GH: RegexSet = RegexSet::new(vec![
        r"(?i)cncf-cla",
        r"(?i)cla/linuxfoundation",
        r"(?i)easycla",
        r"(?i)license/cla",
        r"(?i)cla/google",
    ]).expect("invalid exprs in CLA_IN_GH");

    #[rustfmt::skip]
    pub(crate) static ref CODE_OF_CONDUCT_IN_README: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*code of conduct.*$",
        r"(?im)^code of conduct$",
        r"(?i)\[.*code of conduct.*\]\(.*\)",
    ]).expect("invalid exprs in CODE_OF_CONDUCT_IN_README");

    #[rustfmt::skip]
    pub(crate) static ref COMMUNITY_MEETING_TEXT: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*meeting.*$",
        r"(?i)(community|developer|development) \[?(call|event|meeting|session)",
        r"(?i)(weekly|biweekly|monthly) \[?meeting",
        r"(?i)meeting minutes",
    ]).expect("invalid exprs in COMMUNITY_MEETING_TEXT");

    #[rustfmt::skip]
    pub(crate) static ref CONTRIBUTING_IN_README: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*contributing.*$",
        r"(?im)^contributing$",
        r"(?i)\[.*contributing.*\]\(.*\)",
    ]).expect("invalid exprs in CONTRIBUTING_IN_README");

    #[rustfmt::skip]
    pub(crate) static ref DCO_IN_GH: RegexSet = RegexSet::new(vec![
        r"(?i)dco",
    ]).expect("invalid exprs in DCO_IN_GH");

    #[rustfmt::skip]
    pub(crate) static ref FOSSA_URL: Regex = Regex::new(
        r#"(https://app.fossa.(?:io|com)/projects/[^"'\)]+)"#
    ).expect("invalid exprs in FOSSA_URL");

    #[rustfmt::skip]
    pub(crate) static ref GITHUB_REPO_URL: Regex = Regex::new(
        "^https://github.com/(?P<org>[^/]+)/(?P<repo>[^/]+)/?$"
    ).expect("invalid exprs in GITHUB_REPO_URL");

    #[rustfmt::skip]
    pub(crate) static ref GOVERNANCE_IN_README: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*governance.*$",
        r"(?im)^governance$",
        r"(?i)\[.*governance.*\]\(.*\)",
    ]).expect("invalid exprs in GOVERNANCE_IN_README");

    #[rustfmt::skip]
    pub(crate) static ref MAINTAINERS_IN_README: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*maintainers.*$",
        r"(?im)^maintainers$",
        r"(?i)\[.*maintainers.*\]\(.*\)",
    ]).expect("invalid exprs in MAINTAINERS_IN_README");

    #[rustfmt::skip]
    pub(crate) static ref OPENSSF_URL: Regex = Regex::new(
        r"(https://bestpractices.coreinfrastructure.org/projects/\d+)",
    ).expect("invalid exprs in OPENSSF_URL");

    #[rustfmt::skip]
    pub(crate) static ref ROADMAP_IN_README: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*roadmap.*$",
        r"(?im)^roadmap$",
        r"(?i)\[.*roadmap.*\]\(.*\)",
    ]).expect("invalid exprs in ROADMAP_IN_README");

    #[rustfmt::skip]
    pub(crate) static ref SBOM_IN_GH_RELEASE: RegexSet = RegexSet::new(vec![
        r"(?i)sbom",
    ]).expect("invalid exprs in SBOM_IN_GH_RELEASE");

    #[rustfmt::skip]
    pub(crate) static ref SBOM_IN_README: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*sbom.*$",
        r"(?im)^#+.*software bill of materials.*$",
        r"(?im)^sbom$",
        r"(?im)^software bill of materials$",
    ]).expect("invalid exprs in SBOM_IN_README");

    #[rustfmt::skip]
    pub(crate) static ref SECURITY_POLICY_IN_README: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*security.*$",
        r"(?im)^security$",
        r"(?i)\[.*security.*\]\(.*\)",
    ]).expect("invalid exprs in SECURITY_POLICY_IN_README");

    #[rustfmt::skip]
    pub(crate) static ref SLACK_IN_README: RegexSet = RegexSet::new(vec![
        r"(?i)https?://cloud-native.slack.com",
        r"(?i)https?://slack.cncf.io",
        r"(?i)https?://kubernetes.slack.com",
        r"(?i)https?://slack.k8s.io",
    ]).expect("invalid exprs in SLACK_IN_README");

    #[rustfmt::skip]
    pub(crate) static ref SNYK_URL: Regex = Regex::new(
        r#"(https://snyk.io/test/github/[^/]+/[^/"]+)"#
    ).expect("invalid exprs in SNYK_URL");

    #[rustfmt::skip]
    pub(crate) static ref TRADEMARK_DISCLAIMER_IN_WEBSITE: RegexSet = RegexSet::new(vec![
        r"https://(?:w{3}\.)?linuxfoundation.org/trademark-usage",
        r"The Linux Foundation.* has registered trademarks and uses trademarks",
    ]).expect("invalid exprs in TRADEMARK_DISCLAIMER");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Regular expressions

    #[test]
    fn adopters_in_readme_match() {
        assert!(ADOPTERS_IN_README.is_match("# Adopters"));
        assert!(ADOPTERS_IN_README.is_match(
            r"
...
## Project adopters and others
...
            "
        ));
        assert!(ADOPTERS_IN_README.is_match(
            r"
...
Adopters
--------
...
            "
        ));
        assert!(ADOPTERS_IN_README.is_match("[Project adopters](...)"));
    }

    #[test]
    fn artifacthub_url_extract() {
        assert_eq!(
            ARTIFACTHUB_URL.captures(r#"[![Artifact HUB]("https://img.shields.io/endpoint?url=https://artifacthub.io/badge/repository/artifact-hub)](https://artifacthub.io/packages/helm/artifact-hub/artifact-hub)"#).unwrap()[1].to_string(),
            "https://artifacthub.io/packages/helm/artifact-hub/artifact-hub"
        );
    }

    #[test]
    fn changelog_in_gh_release_match() {
        assert!(CHANGELOG_IN_GH_RELEASE.is_match("# Changelog"));
        assert!(CHANGELOG_IN_GH_RELEASE.is_match("Below you can find the changelog"));
    }

    #[test]
    fn changelog_in_readme_match() {
        assert!(CHANGELOG_IN_README.is_match("# Changelog"));
        assert!(CHANGELOG_IN_README.is_match(
            r"
...
## Project changelog and others
...
            "
        ));
        assert!(CHANGELOG_IN_README.is_match(
            r"
...
Changelog
=========
...
            "
        ));
        assert!(CHANGELOG_IN_README.is_match("[Project changelog](...)"));
    }

    #[test]
    fn cla_in_gh_match() {
        assert!(CLA_IN_GH.is_match(r"EasyCLA"));
        assert!(CLA_IN_GH.is_match(r"cncf-cla"));
        assert!(CLA_IN_GH.is_match(r"cla/linuxfoundation"));
        assert!(CLA_IN_GH.is_match(r"license/cla"));
        assert!(CLA_IN_GH.is_match(r"cla/google"));
    }

    #[test]
    fn code_of_conduct_in_readme_match() {
        assert!(CODE_OF_CONDUCT_IN_README.is_match("# Code of conduct"));
        assert!(CODE_OF_CONDUCT_IN_README.is_match(
            r"
...
## Project code of conduct and others
...
            "
        ));
        assert!(CODE_OF_CONDUCT_IN_README.is_match(
            r"
...
Code of Conduct
---------------
...
            "
        ));
        assert!(CODE_OF_CONDUCT_IN_README.is_match("[code of conduct](...)"));
    }

    #[test]
    fn community_meeting_text_match() {
        assert!(COMMUNITY_MEETING_TEXT.is_match("# Project Status Meetings"));
        assert!(COMMUNITY_MEETING_TEXT.is_match("# Community meeting"));
        assert!(COMMUNITY_MEETING_TEXT.is_match("# Community [meeting](...)"));
        assert!(COMMUNITY_MEETING_TEXT.is_match("## Developer call"));
        assert!(COMMUNITY_MEETING_TEXT.is_match("development event"));
        assert!(COMMUNITY_MEETING_TEXT.is_match("community session"));
        assert!(COMMUNITY_MEETING_TEXT.is_match("the meeting minutes below"));
        assert!(COMMUNITY_MEETING_TEXT.is_match("Weekly meeting"));
    }

    #[test]
    fn contributing_in_readme_match() {
        assert!(CONTRIBUTING_IN_README.is_match("# Contributing"));
        assert!(CONTRIBUTING_IN_README.is_match(
            r"
...
## Some stuff, contributing and others
...
            "
        ));
        assert!(CONTRIBUTING_IN_README.is_match(
            r"
...
Contributing
------------
...
            "
        ));
        assert!(CONTRIBUTING_IN_README.is_match("[Project contributing](...)"));
    }

    #[test]
    fn dco_in_gh_match() {
        assert!(DCO_IN_GH.is_match(r"DCO"));
    }

    #[test]
    fn github_repo_url_match() {
        assert!(GITHUB_REPO_URL.is_match("https://github.com/owner/repo"));
        assert!(GITHUB_REPO_URL.is_match("https://github.com/owner/repo/"));
    }

    #[test]
    fn governance_in_readme_match() {
        assert!(GOVERNANCE_IN_README.is_match("# Governance"));
        assert!(GOVERNANCE_IN_README.is_match(
            r"
...
## Project governance and others
...
            "
        ));
        assert!(GOVERNANCE_IN_README.is_match(
            r"
...
Governance
----------
...
            "
        ));
        assert!(GOVERNANCE_IN_README.is_match("[Project governance](...)"));
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
    fn maintainers_in_readme_match() {
        assert!(MAINTAINERS_IN_README.is_match("# Maintainers"));
        assert!(MAINTAINERS_IN_README.is_match(
            r"
...
## Project maintainers and others
...
            "
        ));
        assert!(MAINTAINERS_IN_README.is_match(
            r"
...
Maintainers
----------
...
            "
        ));
        assert!(MAINTAINERS_IN_README.is_match("[Project maintainers](...)"));
    }

    #[test]
    fn openssf_url_extract() {
        assert_eq!(
            OPENSSF_URL.captures("[![CII Best Practices](https://bestpractices.coreinfrastructure.org/projects/4106/badge)](https://bestpractices.coreinfrastructure.org/projects/4106)").unwrap()[1].to_string(),
            "https://bestpractices.coreinfrastructure.org/projects/4106"
        );
    }

    #[test]
    fn roadmap_in_readme_match() {
        assert!(ROADMAP_IN_README.is_match("# Roadmap"));
        assert!(ROADMAP_IN_README.is_match(
            r"
...
## Project roadmap and others
...
            "
        ));
        assert!(ROADMAP_IN_README.is_match(
            r"
...
Roadmap
-------
...
            "
        ));
        assert!(ROADMAP_IN_README.is_match("[Project roadmap](...)"));
    }

    #[test]
    fn sbom_in_gh_release_match() {
        assert!(SBOM_IN_GH_RELEASE.is_match("flux_0.28.2_sbom.spdx.json"));
    }

    #[test]
    fn sbom_in_readme_match() {
        assert!(SBOM_IN_README.is_match("# SBOM"));
        assert!(SBOM_IN_README.is_match("# Software Bill of Materials"));
        assert!(SBOM_IN_README.is_match(
            r"
...
## Project SBOM
...
            "
        ));
        assert!(SBOM_IN_README.is_match(
            r"
...
Software Bill of Materials
--------------------------
...
            "
        ));
    }

    #[test]
    fn security_polity_in_readme_match() {
        assert!(SECURITY_POLICY_IN_README.is_match("# Security"));
        assert!(SECURITY_POLICY_IN_README.is_match(
            r"
...
## Project security and others
...
            "
        ));
        assert!(SECURITY_POLICY_IN_README.is_match(
            r"
...
Security
--------
...
            "
        ));
        assert!(SECURITY_POLICY_IN_README.is_match("[Project security policy](...)"));
    }

    #[test]
    fn slack_match() {
        assert!(SLACK_IN_README.is_match("Joining [CNCF slack](https://cloud-native.slack.com)"));
        assert!(SLACK_IN_README.is_match("Visit [http://slack.cncf.io/](http://slack.cncf.io/)"));
        assert!(SLACK_IN_README.is_match("[KEDA](https://kubernetes.slack.com/messages/CKZJ36A5D)"));
        assert!(SLACK_IN_README.is_match("[Kubernetes Slack](https://slack.k8s.io/)"));
    }

    #[test]
    fn trademark_disclaimer_match() {
        assert!(TRADEMARK_DISCLAIMER_IN_WEBSITE
            .is_match("https://www.linuxfoundation.org/trademark-usage"));
        assert!(
            TRADEMARK_DISCLAIMER_IN_WEBSITE.is_match("https://linuxfoundation.org/trademark-usage")
        );
        assert!(TRADEMARK_DISCLAIMER_IN_WEBSITE.is_match(
            "The Linux Foundation® (TLF) has registered trademarks and uses trademarks."
        ));
    }
}
