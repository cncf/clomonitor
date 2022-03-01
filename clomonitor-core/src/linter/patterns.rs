// Documentation
pub(crate) static ADOPTERS_FILE: [&str; 1] = ["adopters*"];
pub(crate) static ADOPTERS_HEADER: [&str; 1] = [r"(?im)^#+.*adopters.*$"];
pub(crate) static CODE_OF_CONDUCT_FILE: [&str; 2] =
    ["code*of*conduct.md", "docs/code*of*conduct.md"];
pub(crate) static CODE_OF_CONDUCT_HEADER: [&str; 1] = [r"(?im)^#+.*code of conduct.*$"];
pub(crate) static CONTRIBUTING_FILE: [&str; 2] = ["contributing*", "docs/contributing*"];
pub(crate) static CHANGELOG_FILE: [&str; 1] = ["changelog*"];
pub(crate) static CHANGELOG_HEADER: [&str; 1] = [r"(?im)^#+.*changelog.*$"];
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
pub(crate) static LICENSE_SCANNING_URL: [&str; 2] = [
    r"\[!\[.*\]\(https://app.fossa.*/api/projects/.*\)\]\((.*)\)",
    r"\[!\[.*\]\(https://snyk.io/test/github/[^/]+/[^/]+/badge.svg\)\]\((.*)\)",
];

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
    [r"https://www.linuxfoundation.org/trademark-usage"];

// Security
pub(crate) static SECURITY_POLICY_FILE: [&str; 3] =
    ["security*", "docs/security*", ".github/security*"];
pub(crate) static SECURITY_POLICY_HEADER: [&str; 1] = [r"(?im)^#+.*security.*$"];
