use std::{collections::HashMap, sync::LazyLock};

use crate::linter::check::{CheckConfig, CheckId, Datasource};

use self::datasource::afdocs;

pub(crate) mod adopters;
pub(crate) mod analytics;
pub(crate) mod artifacthub_badge;
pub(crate) mod authentication;
pub(crate) mod binary_artifacts;
pub(crate) mod changelog;
pub(crate) mod cla;
pub(crate) mod code_of_conduct;
pub(crate) mod code_review;
pub(crate) mod community_meeting;
pub(crate) mod content_discoverability;
pub(crate) mod content_structure;
pub(crate) mod contributing;
pub(crate) mod dangerous_workflow;
pub(crate) mod datasource;
pub(crate) mod dco;
pub(crate) mod dependencies_policy;
pub(crate) mod dependency_update_tool;
pub(crate) mod github_discussions;
pub(crate) mod governance;
pub(crate) mod license_approved;
pub(crate) mod license_scanning;
pub(crate) mod license_spdx_id;
pub(crate) mod maintained;
pub(crate) mod maintainers;
pub(crate) mod markdown_availability;
pub(crate) mod observability;
pub(crate) mod openssf_badge;
pub(crate) mod openssf_scorecard_badge;
pub(crate) mod page_size;
pub(crate) mod readme;
pub(crate) mod recent_release;
pub(crate) mod roadmap;
pub(crate) mod sbom;
pub(crate) mod security_insights;
pub(crate) mod security_policy;
pub(crate) mod signed_releases;
pub(crate) mod slack_presence;
pub(crate) mod summary_table;
pub(crate) mod token_permissions;
pub(crate) mod trademark_disclaimer;
pub(crate) mod url_stability;
pub(crate) mod util;
pub(crate) mod website;

pub(crate) static CHECKS: LazyLock<HashMap<CheckId, CheckConfig>> = LazyLock::new(|| {
    let mut checks = HashMap::new();

    macro_rules! register_check {
        ($check:ident) => {
            checks.insert(
                $check::ID,
                CheckConfig {
                    weight: $check::WEIGHT,
                    check_sets: $check::CHECK_SETS.to_vec(),
                    datasource: None,
                },
            );
        };
        ($check:ident, scorecard = $scorecard_name:expr) => {
            checks.insert(
                $check::ID,
                CheckConfig {
                    weight: $check::WEIGHT,
                    check_sets: $check::CHECK_SETS.to_vec(),
                    datasource: Some(Datasource::Scorecard {
                        name: $scorecard_name.to_string(),
                    }),
                },
            );
        };
        ($check:ident, afdocs = $category:expr) => {
            checks.insert(
                $check::ID,
                CheckConfig {
                    weight: $check::WEIGHT,
                    check_sets: $check::CHECK_SETS.to_vec(),
                    datasource: Some(Datasource::Afdocs {
                        category: $category,
                    }),
                },
            );
        };
    }

    register_check!(adopters);
    register_check!(analytics);
    register_check!(artifacthub_badge);
    register_check!(authentication, afdocs = afdocs::AUTHENTICATION);
    register_check!(binary_artifacts, scorecard = "Binary-Artifacts");
    register_check!(changelog);
    register_check!(cla);
    register_check!(code_of_conduct);
    register_check!(code_review, scorecard = "Code-Review");
    register_check!(community_meeting);
    register_check!(
        content_discoverability,
        afdocs = afdocs::CONTENT_DISCOVERABILITY
    );
    register_check!(content_structure, afdocs = afdocs::CONTENT_STRUCTURE);
    register_check!(contributing);
    register_check!(dangerous_workflow, scorecard = "Dangerous-Workflow");
    register_check!(dco);
    register_check!(dependencies_policy);
    register_check!(dependency_update_tool, scorecard = "Dependency-Update-Tool");
    register_check!(github_discussions);
    register_check!(governance);
    register_check!(license_approved);
    register_check!(license_scanning);
    register_check!(license_spdx_id);
    register_check!(maintained, scorecard = "Maintained");
    register_check!(maintainers);
    register_check!(
        markdown_availability,
        afdocs = afdocs::MARKDOWN_AVAILABILITY
    );
    register_check!(observability, afdocs = afdocs::OBSERVABILITY);
    register_check!(openssf_badge);
    register_check!(openssf_scorecard_badge);
    register_check!(page_size, afdocs = afdocs::PAGE_SIZE);
    register_check!(readme);
    register_check!(recent_release);
    register_check!(roadmap);
    register_check!(sbom);
    register_check!(security_insights);
    register_check!(security_policy);
    register_check!(signed_releases, scorecard = "Signed-Releases");
    register_check!(slack_presence);
    register_check!(summary_table);
    register_check!(token_permissions, scorecard = "Token-Permissions");
    register_check!(trademark_disclaimer);
    register_check!(url_stability, afdocs = afdocs::URL_STABILITY);
    register_check!(website);

    checks
});
