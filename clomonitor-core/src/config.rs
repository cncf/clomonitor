use crate::linter::CheckSet;
use lazy_static::lazy_static;
use std::collections::HashMap;

// Checks identifiers
pub const ADOPTERS: &str = "adopters";
pub const ANALYTICS: &str = "analytics";
pub const ARTIFACTHUB_BADGE: &str = "artifacthub_badge";
pub const BINARY_ARTIFACTS: &str = "binary_artifacts";
pub const CHANGELOG: &str = "changelog";
pub const CLA: &str = "cla";
pub const CODE_OF_CONDUCT: &str = "code_of_conduct";
pub const CODE_REVIEW: &str = "code_review";
pub const COMMUNITY_MEETING: &str = "community_meeting";
pub const CONTRIBUTING: &str = "contributing";
pub const DANGEROUS_WORKFLOW: &str = "dangerous_workflow";
pub const DEPENDENCY_UPDATE_TOOL: &str = "dependency_update_tool";
pub const DCO: &str = "dco";
pub const GITHUB_DISCUSSIONS: &str = "github_discussions";
pub const GOVERNANCE: &str = "governance";
pub const LICENSE_APPROVED: &str = "license_approved";
pub const LICENSE_SCANNING: &str = "license_scanning";
pub const LICENSE_SPDX: &str = "license_spdx_id";
pub const MAINTAINED: &str = "maintained";
pub const MAINTAINERS: &str = "maintainers";
pub const OPENSSF_BADGE: &str = "openssf_badge";
pub const README: &str = "readme";
pub const RECENT_RELEASE: &str = "recent_release";
pub const ROADMAP: &str = "roadmap";
pub const SBOM: &str = "sbom";
pub const SECURITY_POLICY: &str = "security_policy";
pub const SLACK_PRESENCE: &str = "slack_presence";
pub const SIGNED_RELEASES: &str = "signed_releases";
pub const TRADEMARK_DISCLAIMER: &str = "trademark_disclaimer";
pub const TOKEN_PERMISSIONS: &str = "token_permissions";
pub const WEBSITE: &str = "website";

// Checks weights
lazy_static! {
    pub static ref CHECK_WEIGHT: HashMap<&'static str, usize> = {
        let mut m = HashMap::new();

        // Documentation
        m.insert(ADOPTERS, 1);
        m.insert(CHANGELOG, 1);
        m.insert(CODE_OF_CONDUCT, 2);
        m.insert(CONTRIBUTING, 4);
        m.insert(GOVERNANCE, 3);
        m.insert(MAINTAINERS, 3);
        m.insert(README, 10);
        m.insert(ROADMAP, 1);
        m.insert(WEBSITE, 5);

        // License
        m.insert(LICENSE_APPROVED, 10);
        m.insert(LICENSE_SCANNING, 5);
        m.insert(LICENSE_SPDX, 5);

        // Best practices
        m.insert(ANALYTICS, 1);
        m.insert(ARTIFACTHUB_BADGE, 1);
        m.insert(CLA, 1);
        m.insert(COMMUNITY_MEETING, 3);
        m.insert(DCO, 1);
        m.insert(GITHUB_DISCUSSIONS, 0);
        m.insert(OPENSSF_BADGE, 10);
        m.insert(RECENT_RELEASE, 3);
        m.insert(SLACK_PRESENCE, 0);

        // Security
        m.insert(BINARY_ARTIFACTS, 2);
        m.insert(CODE_REVIEW, 3);
        m.insert(DANGEROUS_WORKFLOW, 2);
        m.insert(DEPENDENCY_UPDATE_TOOL, 2);
        m.insert(MAINTAINED, 3);
        m.insert(SBOM, 1);
        m.insert(SECURITY_POLICY, 3);
        m.insert(SIGNED_RELEASES, 2);
        m.insert(TOKEN_PERMISSIONS, 2);

        // Legal
        m.insert(TRADEMARK_DISCLAIMER, 5);

        m
    };
}

// Checks sets
lazy_static! {
    pub static ref CHECKSET: HashMap<CheckSet, Vec<&'static str>> = {
        let mut m = HashMap::new();

        // Code
        m.insert(
            CheckSet::Code,
            vec![
                ARTIFACTHUB_BADGE,
                BINARY_ARTIFACTS,
                CHANGELOG,
                CLA,
                CODE_REVIEW,
                CONTRIBUTING,
                DANGEROUS_WORKFLOW,
                DCO,
                DEPENDENCY_UPDATE_TOOL,
                LICENSE_SPDX,
                LICENSE_APPROVED,
                LICENSE_SCANNING,
                MAINTAINED,
                MAINTAINERS,
                OPENSSF_BADGE,
                README,
                RECENT_RELEASE,
                SBOM,
                SECURITY_POLICY,
                SIGNED_RELEASES,
                TOKEN_PERMISSIONS,
            ],
        );

        // CodeLite
        m.insert(
            CheckSet::CodeLite,
            vec![
                CONTRIBUTING,
                CLA,
                DCO,
                LICENSE_SPDX,
                LICENSE_APPROVED,
                MAINTAINERS,
                README,
                RECENT_RELEASE,
            ],
        );

        // Community
        m.insert(
            CheckSet::Community,
            vec![
                ADOPTERS,
                ANALYTICS,
                CODE_OF_CONDUCT,
                COMMUNITY_MEETING,
                CONTRIBUTING,
                GITHUB_DISCUSSIONS,
                GOVERNANCE,
                README,
                ROADMAP,
                SECURITY_POLICY,
                SLACK_PRESENCE,
                TRADEMARK_DISCLAIMER,
                WEBSITE,
            ],
        );

        // Docs
        m.insert(CheckSet::Docs, vec![LICENSE_SPDX, LICENSE_APPROVED, README]);

        m
    };
}

// OpenSSF Scorecard checks mapping
lazy_static! {
    pub static ref SCORECARD_CHECK: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();

        m.insert(BINARY_ARTIFACTS, "Binary-Artifacts");
        m.insert(CODE_REVIEW, "Code-Review");
        m.insert(DANGEROUS_WORKFLOW, "Dangerous-Workflow");
        m.insert(DEPENDENCY_UPDATE_TOOL, "Dependency-Update-Tool");
        m.insert(MAINTAINED, "Maintained");
        m.insert(SIGNED_RELEASES, "Signed-Releases");
        m.insert(TOKEN_PERMISSIONS, "Token-Permissions");

        m
    };
}
