use self::check::scorecard::scorecard;
use super::config::*;
use anyhow::{format_err, Result};
use check::{
    metadata::{Metadata, METADATA_FILE},
    *,
};
use clap::ArgEnum;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use which::which;

mod check;
pub use check::CheckOutput;

/// Check sets define a set of checks that will be run on a given repository.
/// Multiple check sets can be assigned to a repository.
#[derive(Debug, Clone, PartialEq, Eq, Hash, ArgEnum, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CheckSet {
    Code,
    CodeLite,
    Community,
    Docs,
}

/// Linter configuration options.
#[derive(Debug, Clone)]
pub struct LintOptions {
    pub root: PathBuf,
    pub url: String,
    pub check_sets: Vec<CheckSet>,
    pub github_token: String,
}

/// Services used by the linter to perform some of the checks.
#[derive(Debug, Clone)]
pub struct LintServices {
    pub http_client: reqwest::Client,
    pub http_client_gh: reqwest::Client,
}

/// Options used to setup the Github client.
#[derive(Debug, Clone, Default)]
pub struct GithubOptions {
    pub token: String,
    pub api_url: Option<String>,
}

impl LintServices {
    /// Create a new LintServices instance.
    pub fn new(gh_opts: &GithubOptions) -> Result<Self> {
        // Setup http client
        let http_client = reqwest::Client::new();

        // Setup authenticated http client for Github API
        let http_client_gh = reqwest::Client::builder()
            .user_agent("clomonitor")
            .default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", gh_opts.token))
                        .unwrap(),
                ))
                .collect(),
            )
            .build()?;

        Ok(Self {
            http_client,
            http_client_gh,
        })
    }
}

/// Linter report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub documentation: Documentation,
    pub license: License,
    pub best_practices: BestPractices,
    pub security: Security,
    pub legal: Legal,
}

/// Documentation section of the report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Documentation {
    pub adopters: Option<CheckOutput>,
    pub changelog: Option<CheckOutput>,
    pub code_of_conduct: Option<CheckOutput>,
    pub contributing: Option<CheckOutput>,
    pub governance: Option<CheckOutput>,
    pub maintainers: Option<CheckOutput>,
    pub readme: Option<CheckOutput>,
    pub roadmap: Option<CheckOutput>,
    pub website: Option<CheckOutput>,
}

/// License section of the report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub license_approved: Option<CheckOutput<bool>>,
    pub license_scanning: Option<CheckOutput>,
    pub license_spdx_id: Option<CheckOutput<String>>,
}

/// BestPractices section of the report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestPractices {
    pub artifacthub_badge: Option<CheckOutput>,
    pub cla: Option<CheckOutput>,
    pub community_meeting: Option<CheckOutput>,
    pub dco: Option<CheckOutput>,
    pub ga4: Option<CheckOutput>,
    pub github_discussions: Option<CheckOutput>,
    pub openssf_badge: Option<CheckOutput>,
    pub recent_release: Option<CheckOutput>,
    pub slack_presence: Option<CheckOutput>,
}

/// Security section of the report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Security {
    pub binary_artifacts: Option<CheckOutput>,
    pub branch_protection: Option<CheckOutput>,
    pub code_review: Option<CheckOutput>,
    pub dangerous_workflow: Option<CheckOutput>,
    pub dependency_update_tool: Option<CheckOutput>,
    pub maintained: Option<CheckOutput>,
    pub sbom: Option<CheckOutput>,
    pub security_policy: Option<CheckOutput>,
    pub signed_releases: Option<CheckOutput>,
    pub token_permissions: Option<CheckOutput>,
    pub vulnerabilities: Option<CheckOutput>,
}

/// Legal section of the report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Legal {
    pub trademark_disclaimer: Option<CheckOutput>,
}

/// Lint the path provided and return a report.
pub async fn lint(opts: &LintOptions, svc: &LintServices) -> Result<Report> {
    // Check if required external tools are available
    if which("scorecard").is_err() {
        return Err(format_err!(
            "scorecard not found in PATH (https://github.com/ossf/scorecard#installation)"
        ));
    }

    // Get CLOMonitor metadata
    let cm_md = Metadata::from(&opts.root.join(METADATA_FILE))?;

    // Get Github metadata and OpenSSF scorecard
    let (gh_md, scorecard) = tokio::try_join!(
        github::metadata(&svc.http_client_gh, &opts.url),
        scorecard(&opts.url, &opts.github_token),
    )?;

    // Prepare check input
    let input = CheckInput {
        opts,
        svc,
        cm_md: cm_md.as_ref(),
        gh_md: &gh_md,
        scorecard: &scorecard,
    };

    // Run some async checks
    let (contributing, ga4, trademark_disclaimer) = tokio::join!(
        run_async_check(CONTRIBUTING, contributing, &input),
        run_async_check(GA4, ga4, &input),
        run_async_check(TRADEMARK_DISCLAIMER, trademark_disclaimer, &input),
    );

    // Run some sync checks
    let spdx_id = run_check(LICENSE_SPDX, license, &input);
    let mut spdx_id_value: Option<String> = None;
    if let Some(r) = &spdx_id {
        spdx_id_value = r.value.clone();
    }

    // Build report
    let mut report = Report {
        documentation: Documentation {
            adopters: run_check(ADOPTERS, adopters, &input),
            changelog: run_check(CHANGELOG, changelog, &input),
            code_of_conduct: run_check(CODE_OF_CONDUCT, code_of_conduct, &input),
            contributing,
            governance: run_check(GOVERNANCE, governance, &input),
            maintainers: run_check(MAINTAINERS, maintainers, &input),
            readme: run_check(README, readme, &input),
            roadmap: run_check(ROADMAP, roadmap, &input),
            website: run_check(WEBSITE, website, &input),
        },
        license: License {
            license_approved: license_approved(spdx_id_value, &input),
            license_scanning: run_check(LICENSE_SCANNING, license_scanning, &input),
            license_spdx_id: spdx_id,
        },
        best_practices: BestPractices {
            artifacthub_badge: run_check(ARTIFACTHUB_BADGE, artifacthub_badge, &input),
            cla: run_check(CLA, cla, &input),
            community_meeting: run_check(COMMUNITY_MEETING, community_meeting, &input),
            dco: run_check(DCO, dco, &input),
            ga4,
            github_discussions: run_check(GITHUB_DISCUSSIONS, github_discussions, &input),
            openssf_badge: run_check(OPENSSF_BADGE, openssf_badge, &input),
            recent_release: run_check(RECENT_RELEASE, recent_release, &input),
            slack_presence: run_check(SLACK_PRESENCE, slack_presence, &input),
        },
        security: Security {
            binary_artifacts: run_check(BINARY_ARTIFACTS, binary_artifacts, &input),
            branch_protection: run_check(BRANCH_PROTECTION, branch_protection, &input),
            code_review: run_check(CODE_REVIEW, code_review, &input),
            dangerous_workflow: run_check(DANGEROUS_WORKFLOW, dangerous_workflow, &input),
            dependency_update_tool: run_check(
                DEPENDENCY_UPDATE_TOOL,
                dependency_update_tool,
                &input,
            ),
            maintained: run_check(MAINTAINED, maintained, &input),
            sbom: run_check(SBOM, sbom, &input),
            security_policy: run_check(SECURITY_POLICY, security_policy, &input),
            signed_releases: run_check(SIGNED_RELEASES, signed_releases, &input),
            token_permissions: run_check(TOKEN_PERMISSIONS, token_permissions, &input),
            vulnerabilities: run_check(VULNERABILITIES, vulnerabilities, &input),
        },
        legal: Legal {
            trademark_disclaimer,
        },
    };

    apply_exemptions(&mut report);

    Ok(report)
}

/// Apply inter-checks exemptions.
fn apply_exemptions(report: &mut Report) {
    let passed = |o: Option<&CheckOutput>| -> bool {
        match o {
            Some(o) => o.passed || o.exempt,
            None => false,
        }
    };

    // CLA / DCO
    if passed(report.best_practices.cla.as_ref()) && !passed(report.best_practices.dco.as_ref()) {
        report.best_practices.dco = Some(CheckOutput {
            exempt: true,
            exemption_reason: Some("CLA check passed".to_string()),
            ..Default::default()
        });
    }
    if passed(report.best_practices.dco.as_ref()) && !passed(report.best_practices.cla.as_ref()) {
        report.best_practices.cla = Some(CheckOutput {
            exempt: true,
            exemption_reason: Some("DCO check passed".to_string()),
            ..Default::default()
        });
    }

    // Slack presence / GitHub discussions
    if passed(report.best_practices.slack_presence.as_ref())
        && !passed(report.best_practices.github_discussions.as_ref())
    {
        report.best_practices.github_discussions = Some(CheckOutput {
            exempt: true,
            exemption_reason: Some("Slack presence check passed".to_string()),
            ..Default::default()
        });
    }
    if passed(report.best_practices.github_discussions.as_ref())
        && !passed(report.best_practices.slack_presence.as_ref())
    {
        report.best_practices.slack_presence = Some(CheckOutput {
            exempt: true,
            exemption_reason: Some("GitHub discussions check passed".to_string()),
            ..Default::default()
        });
    }
}
