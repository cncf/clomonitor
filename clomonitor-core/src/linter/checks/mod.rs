use crate::linter::check::{CheckConfig, CheckId};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub(crate) mod adopters;
pub(crate) mod analytics;
pub(crate) mod annual_review;
pub(crate) mod artifacthub_badge;
pub(crate) mod binary_artifacts;
pub(crate) mod changelog;
pub(crate) mod cla;
pub(crate) mod code_of_conduct;
pub(crate) mod code_review;
pub(crate) mod community_meeting;
pub(crate) mod contributing;
pub(crate) mod dangerous_workflow;
pub(crate) mod dco;
pub(crate) mod dependency_update_tool;
pub(crate) mod github_discussions;
pub(crate) mod governance;
pub(crate) mod license_approved;
pub(crate) mod license_scanning;
pub(crate) mod license_spdx_id;
pub(crate) mod maintained;
pub(crate) mod maintainers;
pub(crate) mod openssf_badge;
pub(crate) mod openssf_scorecard_badge;
pub(crate) mod readme;
pub(crate) mod recent_release;
pub(crate) mod roadmap;
pub(crate) mod sbom;
pub(crate) mod security_policy;
pub(crate) mod signed_releases;
pub(crate) mod slack_presence;
pub(crate) mod summary_table;
pub(crate) mod token_permissions;
pub(crate) mod trademark_disclaimer;
pub(crate) mod util;
pub(crate) mod website;

lazy_static! {
    pub(crate) static ref CHECKS: HashMap<CheckId, CheckConfig> = {
        let mut checks = HashMap::new();

        macro_rules! register_check {
            ($check:ident) => {
                checks.insert(
                    $check::ID,
                    CheckConfig {
                        weight: $check::WEIGHT,
                        check_sets: $check::CHECK_SETS.to_vec(),
                        scorecard_name: None,
                    },
                );
            };
            ($check:ident, $scorecard_name:expr) => {
                checks.insert(
                    $check::ID,
                    CheckConfig {
                        weight: $check::WEIGHT,
                        check_sets: $check::CHECK_SETS.to_vec(),
                        scorecard_name: Some($scorecard_name.to_string()),
                    },
                );
            };
        }

        register_check!(adopters);
        register_check!(analytics);
        register_check!(annual_review);
        register_check!(artifacthub_badge);
        register_check!(binary_artifacts, "Binary-Artifacts");
        register_check!(changelog);
        register_check!(cla);
        register_check!(code_of_conduct);
        register_check!(code_review, "Code-Review");
        register_check!(community_meeting);
        register_check!(contributing);
        register_check!(dangerous_workflow, "Dangerous-Workflow");
        register_check!(dco);
        register_check!(dependency_update_tool, "Dependency-Update-Tool");
        register_check!(github_discussions);
        register_check!(governance);
        register_check!(license_approved);
        register_check!(license_scanning);
        register_check!(license_spdx_id);
        register_check!(maintained, "Maintained");
        register_check!(maintainers);
        register_check!(openssf_badge);
        register_check!(openssf_scorecard_badge);
        register_check!(readme);
        register_check!(recent_release);
        register_check!(roadmap);
        register_check!(sbom);
        register_check!(security_policy);
        register_check!(signed_releases, "Signed-Releases");
        register_check!(slack_presence);
        register_check!(summary_table);
        register_check!(token_permissions, "Token-Permissions");
        register_check!(trademark_disclaimer);
        register_check!(website);

        checks
    };
}
