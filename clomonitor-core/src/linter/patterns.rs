// Documentation
pub(crate) static ADOPTERS: [&str; 1] = ["adopters*"];
pub(crate) static CODE_OF_CONDUCT: [&str; 2] = ["code*of*conduct.md", "docs/code*of*conduct.md"];
pub(crate) static CONTRIBUTING: [&str; 2] = ["contributing*", "docs/contributing*"];
pub(crate) static CHANGELOG: [&str; 1] = ["changelog*"];
pub(crate) static GOVERNANCE: [&str; 2] = ["governance*", "docs/governance*"];
pub(crate) static MAINTAINERS: [&str; 7] = [
    "maintainers*",
    "docs/maintainers*",
    "owners*",
    "docs/owners*",
    "codeowners*",
    "docs/codeowners*",
    ".github/codeowners*",
];
pub(crate) static README: [&str; 1] = ["README*"];
pub(crate) static ROADMAP: [&str; 1] = ["roadmap*"];

// License
pub(crate) static LICENSE: [&str; 2] = ["LICENSE*", "COPYING*"];
pub(crate) static FOSSA_BADGE: [&str; 1] = [r"https://app.fossa.*/api/projects/.*"];

// Best practices
pub(crate) static ARTIFACTHUB_BADGE: [&str; 1] = [r"https://artifacthub.io/badge/repository/.*"];
pub(crate) static COMMUNITY_MEETING: [&str; 3] = [
    r"(?i)(community|developer|development) (call|event|meeting|session)",
    r"(?i)(weekly|biweekly|monthly) meeting",
    r"(?i)meeting minutes",
];
pub(crate) static OPENSSF_BADGE: [&str; 1] =
    [r"https://bestpractices.coreinfrastructure.org/projects/\d+"];

// Security
pub(crate) static SECURITY_POLICY: [&str; 3] = ["security*", "docs/security*", ".github/security*"];
