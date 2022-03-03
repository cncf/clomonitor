// Documentation
pub(crate) static ADOPTERS_FILE: [&str; 2] = ["adopters*", "USERS.md"];
pub(crate) static ADOPTERS_HEADER: [&str; 1] = [r"(?im)^#+.*adopters.*$"];
pub(crate) static CODE_OF_CONDUCT_FILE: [&str; 2] =
    ["code*of*conduct.md", "docs/code*of*conduct.md"];
pub(crate) static CODE_OF_CONDUCT_HEADER: [&str; 1] = [r"(?im)^#+.*code of conduct.*$"];
pub(crate) static CONTRIBUTING_FILE: [&str; 2] = ["contributing*", "docs/contributing*"];
pub(crate) static CHANGELOG_FILE: [&str; 1] = ["changelog*"];
pub(crate) static CHANGELOG_HEADER: [&str; 1] = [r"(?im)^#+.*changelog.*$"];
pub(crate) static CHANGELOG_RELEASE: [&str; 1] = [r"(?i)changelog"];
pub(crate) static GOVERNANCE_FILE: [&str; 2] = ["governance*", "docs/governance*"];
pub(crate) static GOVERNANCE_HEADER: [&str; 1] = [r"(?im)^#+.*governance.*$"];
pub(crate) static MAINTAINERS_FILE: [&str; 7] = [
    "maintainers*",
    "docs/maintainers*",
    "owners*",
    "docs/owners*",
    "codeowners*",
    "docs/codeowners*",
    ".github/codeowners*",
];
pub(crate) static README_FILE: [&str; 1] = ["README*"];
pub(crate) static ROADMAP_FILE: [&str; 1] = ["roadmap*"];
pub(crate) static ROADMAP_HEADER: [&str; 1] = [r"(?im)^#+.*roadmap.*$"];

// License
pub(crate) static LICENSE_FILE: [&str; 2] = ["LICENSE*", "COPYING*"];
pub(crate) static LICENSE_SCANNING_URL: [&str; 2] = [FOSSA_URL, SNYK_URL];
static FOSSA_URL: &str = r#"(https://app.fossa.(?:io|com)/projects/[^"'\)]+)"#;
static SNYK_URL: &str = r#"(https://snyk.io/test/github/[^/]+/[^/"]+)"#;

// Best practices
pub(crate) static ARTIFACTHUB_BADGE_URL: [&str; 1] =
    [r"https://artifacthub.io/badge/repository/.*"];
pub(crate) static COMMUNITY_MEETING_TEXT: [&str; 3] = [
    r"(?i)(community|developer|development) (call|event|meeting|session)",
    r"(?i)(weekly|biweekly|monthly) meeting",
    r"(?i)meeting minutes",
];
pub(crate) static OPENSSF_BADGE_URL: [&str; 1] =
    [r"https://bestpractices.coreinfrastructure.org/projects/\d+"];
pub(crate) static TRADEMARK_FOOTER: [&str; 1] =
    [r"https://(?:w{3}\.)?linuxfoundation.org/trademark-usage"];

// Security
pub(crate) static SECURITY_POLICY_FILE: [&str; 3] =
    ["security*", "docs/security*", ".github/security*"];
pub(crate) static SECURITY_POLICY_HEADER: [&str; 1] = [r"(?im)^#+.*security.*$"];

#[cfg(test)]
mod tests {
    use super::*;
    use regex::{Regex, RegexSet};

    #[test]
    fn adopters_header_match() {
        let re = RegexSet::new(ADOPTERS_HEADER).unwrap();
        assert!(re.is_match("# Adopters"));
        assert!(re.is_match("## Project adopters and others"));
    }

    #[test]
    fn artifacthub_badge_url_match() {
        let re = RegexSet::new(ARTIFACTHUB_BADGE_URL).unwrap();
        assert!(re.is_match("[![Artifact HUB](https://img.shields.io/endpoint?url=https://artifacthub.io/badge/repository/artifact-hub)](https://artifacthub.io/packages/helm/artifact-hub/artifact-hub)"));
    }

    #[test]
    fn code_of_conduct_header_match() {
        let re = RegexSet::new(CODE_OF_CONDUCT_HEADER).unwrap();
        assert!(re.is_match("# Code of conduct"));
        assert!(re.is_match("## Project code of conduct and others"));
    }

    #[test]
    fn changelog_header_match() {
        let re = RegexSet::new(CHANGELOG_HEADER).unwrap();
        assert!(re.is_match("# Changelog"));
        assert!(re.is_match("## Project changelog and others"));
    }

    #[test]
    fn changelog_release_match() {
        let re = RegexSet::new(CHANGELOG_RELEASE).unwrap();
        assert!(re.is_match("# Changelog"));
        assert!(re.is_match("Below you can find the changelog"));
    }

    #[test]
    fn community_meeting_text_match() {
        let re = RegexSet::new(COMMUNITY_MEETING_TEXT).unwrap();
        assert!(re.is_match("# Community meeting"));
        assert!(re.is_match("## Developer call"));
        assert!(re.is_match("development event"));
        assert!(re.is_match("community session"));
        assert!(re.is_match("the meeting minutes below"));
        assert!(re.is_match("Weekly meeting"));
    }

    #[test]
    fn governance_header_match() {
        let re = RegexSet::new(GOVERNANCE_HEADER).unwrap();
        assert!(re.is_match("# Governance"));
        assert!(re.is_match("## Project governance and others"));
    }

    #[test]
    fn license_scanning_url_match() {
        let re = RegexSet::new(LICENSE_SCANNING_URL).unwrap();
        assert!(re.is_match("[![Licenses](https://app.fossa.io/api/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fartifacthub%2Fhub.svg?type=shield)](https://app.fossa.io/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fartifacthub%2Fhub?ref=badge_shield)"));
        assert!(re.is_match("[![Known Vulnerabilities](https://snyk.io/test/github/{username}/{repo}/badge.svg)](https://snyk.io/test/github/{username}/{repo})"));
        assert!(re.is_match(r#"<a href="https://app.fossa.io/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fkedacore%2Fkeda?ref=badge_shield"><img src="https://app.fossa.io/api/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fkedacore%2Fkeda.svg?type=shield"></a>"#));
        assert!(re.is_match(r#"<a href="https://snyk.io/test/github/{username}/{repo}">"#));
    }

    #[test]
    fn license_scanning_url_extract_fossa() {
        let re = Regex::new(FOSSA_URL).unwrap();
        assert_eq!(
            re.captures("[![Licenses](https://app.fossa.io/api/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fartifacthub%2Fhub.svg?type=shield)](https://app.fossa.io/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fartifacthub%2Fhub?ref=badge_shield)").unwrap()[1].to_string(),
            "https://app.fossa.io/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fartifacthub%2Fhub?ref=badge_shield"
        );
        assert_eq!(
            re.captures(r#"<a href="https://app.fossa.io/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fkedacore%2Fkeda?ref=badge_shield"><img src="https://app.fossa.io/api/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fkedacore%2Fkeda.svg?type=shield"></a>"#).unwrap()[1].to_string(),
            "https://app.fossa.io/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fkedacore%2Fkeda?ref=badge_shield"
        );
    }

    #[test]
    fn license_scanning_url_extract_snyk() {
        let re = Regex::new(SNYK_URL).unwrap();
        assert_eq!(
            re.captures("[![Known Vulnerabilities](https://snyk.io/test/github/{username}/{repo}/badge.svg)](https://snyk.io/test/github/{username}/{repo})").unwrap()[1].to_string(),
            "https://snyk.io/test/github/{username}/{repo}"
        );
        assert_eq!(
            re.captures(r#"<a href="https://snyk.io/test/github/{username}/{repo}">"#)
                .unwrap()[1]
                .to_string(),
            "https://snyk.io/test/github/{username}/{repo}"
        );
    }

    #[test]
    fn openssf_badge_url_match() {
        let re = RegexSet::new(OPENSSF_BADGE_URL).unwrap();
        assert!(re.is_match("[![CII Best Practices](https://bestpractices.coreinfrastructure.org/projects/4106/badge)](https://bestpractices.coreinfrastructure.org/projects/4106)"));
    }

    #[test]
    fn roadmap_header_match() {
        let re = RegexSet::new(ROADMAP_HEADER).unwrap();
        assert!(re.is_match("# Roadmap"));
        assert!(re.is_match("## Project roadmap and others"));
    }

    #[test]
    fn security_polity_header_match() {
        let re = RegexSet::new(SECURITY_POLICY_HEADER).unwrap();
        assert!(re.is_match("# Security"));
        assert!(re.is_match("## Project security and others"));
    }

    #[test]
    fn trademark_footer_match() {
        let re = RegexSet::new(TRADEMARK_FOOTER).unwrap();
        assert!(re.is_match("https://www.linuxfoundation.org/trademark-usage"));
        assert!(re.is_match("https://linuxfoundation.org/trademark-usage"));
    }
}
