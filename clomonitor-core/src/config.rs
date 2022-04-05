use crate::linter::CheckSet;
use lazy_static::lazy_static;
use std::collections::HashMap;

// Checks identifiers
pub const ADOPTERS: &str = "adopters";
pub const ARTIFACTHUB_BADGE: &str = "artifacthub_badge";
pub const CHANGELOG: &str = "changelog";
pub const CLA: &str = "cla";
pub const CODE_OF_CONDUCT: &str = "code_of_conduct";
pub const COMMUNITY_MEETING: &str = "community_meeting";
pub const CONTRIBUTING: &str = "contributing";
pub const DCO: &str = "dco";
pub const GOVERNANCE: &str = "governance";
pub const LICENSE_APPROVED: &str = "license_approved";
pub const LICENSE_SCANNING: &str = "license_scanning";
pub const LICENSE_SPDX: &str = "license_spdx_id";
pub const MAINTAINERS: &str = "maintainers";
pub const OPENSSF_BADGE: &str = "openssf_badge";
pub const README: &str = "readme";
pub const RECENT_RELEASE: &str = "recent_release";
pub const ROADMAP: &str = "roadmap";
pub const SBOM: &str = "sbom";
pub const SECURITY_POLICY: &str = "security_policy";
pub const SLACK_PRESENCE: &str = "slack_presence";
pub const TRADEMARK_DISCLAIMER: &str = "trademark_disclaimer";
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
        m.insert(ARTIFACTHUB_BADGE, 1);
        m.insert(CLA, 1);
        m.insert(COMMUNITY_MEETING, 3);
        m.insert(DCO, 1);
        m.insert(OPENSSF_BADGE, 10);
        m.insert(RECENT_RELEASE, 3);
        m.insert(SLACK_PRESENCE, 1);

        // Security
        m.insert(SBOM, 2);
        m.insert(SECURITY_POLICY, 13);

        // Legal
        m.insert(TRADEMARK_DISCLAIMER, 5);

        m
    };
}

// Checks sets
lazy_static! {
    pub static ref CHECKSET: HashMap<CheckSet, Vec<&'static str>> = {
        let mut m = HashMap::new();

        m.insert(
            CheckSet::Code,
            vec![
                ARTIFACTHUB_BADGE,
                CHANGELOG,
                CLA,
                CONTRIBUTING,
                DCO,
                LICENSE_SPDX,
                LICENSE_APPROVED,
                LICENSE_SCANNING,
                MAINTAINERS,
                OPENSSF_BADGE,
                README,
                RECENT_RELEASE,
                SBOM,
                SECURITY_POLICY,
            ],
        );
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
        m.insert(
            CheckSet::Community,
            vec![
                ADOPTERS,
                CODE_OF_CONDUCT,
                COMMUNITY_MEETING,
                CONTRIBUTING,
                GOVERNANCE,
                README,
                ROADMAP,
                SECURITY_POLICY,
                SLACK_PRESENCE,
                TRADEMARK_DISCLAIMER,
                WEBSITE,
            ],
        );
        m.insert(CheckSet::Docs, vec![LICENSE_SPDX, LICENSE_APPROVED, README]);

        m
    };
}
