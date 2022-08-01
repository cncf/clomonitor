use self::{
    path::Globs,
    scorecard::{Scorecard, ScorecardCheck},
};
use super::{LintOptions, LintServices};
use crate::{config::*, linter::CheckSet};
use anyhow::Result;
use metadata::{Exemption, Metadata};
use patterns::*;
use regex::{Regex, RegexSet};
use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    path::{Path, PathBuf},
};
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};

pub(crate) mod content;
pub(crate) mod git;
pub(crate) mod github;
pub(crate) mod license;
pub(crate) mod metadata;
pub(crate) mod path;
pub(crate) mod patterns;
pub(crate) mod scorecard;

/// Input used by checks to perform their operations.
#[derive(Debug, Clone)]
pub(crate) struct CheckInput<'a> {
    pub opts: &'a LintOptions,
    pub svc: &'a LintServices,
    pub cm_md: Option<&'a Metadata>,
    pub gh_md: &'a github::md::MdRepository,
    pub scorecard: &'a Scorecard,
}

/// Check output information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckOutput<T = ()> {
    pub passed: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    pub exempt: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption_reason: Option<String>,

    pub failed: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_reason: Option<String>,
}

impl<T> Default for CheckOutput<T> {
    fn default() -> Self {
        Self {
            passed: false,
            url: None,
            value: None,
            details: None,
            exempt: false,
            exemption_reason: None,
            failed: false,
            fail_reason: None,
        }
    }
}

impl<T> From<bool> for CheckOutput<T> {
    fn from(passed: bool) -> Self {
        Self {
            passed,
            ..Default::default()
        }
    }
}

impl<T> From<Option<T>> for CheckOutput<T> {
    fn from(value: Option<T>) -> Self {
        Self {
            passed: value.is_some(),
            value,
            ..Default::default()
        }
    }
}

impl<T> From<Exemption> for CheckOutput<T> {
    fn from(exemption: Exemption) -> Self {
        Self {
            exempt: true,
            exemption_reason: Some(exemption.reason),
            ..Default::default()
        }
    }
}

impl<T> From<&ScorecardCheck> for CheckOutput<T> {
    fn from(sc_check: &ScorecardCheck) -> Self {
        let mut output = CheckOutput::default();
        if sc_check.score >= 5.0 {
            output.passed = true;
        }
        output.details = Some(format!(
            r"# {} OpenSSF Scorecard check

**Score**: {} (check passes with score >= 5)

**Reason**: {}

**Details**: {}

**Please see the [check documentation]({}) in the ossf/scorecard repository for more details**",
            sc_check.name,
            sc_check.score,
            sc_check.reason,
            match &sc_check.details {
                Some(details) => format!("\n\n>{}", details.join("\n")),
                None => "-".to_string(),
            },
            sc_check.documentation.url,
        ));
        output
    }
}

impl<T> CheckOutput<T> {
    /// Create a new CheckOutput instance from the url provided.
    pub(crate) fn from_url(url: Option<String>) -> Self {
        Self {
            passed: url.is_some(),
            url,
            ..Default::default()
        }
    }

    /// Create a new CheckOutput instance from the Github url built using the
    /// path provided.
    pub(crate) fn from_path(path: Option<&PathBuf>, gh_md: &github::md::MdRepository) -> Self {
        match path {
            Some(path) => {
                let url = github::build_url(
                    path,
                    &gh_md.owner.login,
                    &gh_md.name,
                    &github::default_branch(gh_md.default_branch_ref.as_ref()),
                );
                CheckOutput::from_url(Some(url))
            }
            None => false.into(),
        }
    }
}

/// Wrapper function that takes care of running some common pre-check
/// operations and the synchronous check function provided.
pub(crate) fn run_check<T, F>(
    id: &'static str,
    check_fn: F,
    input: &CheckInput,
) -> Option<CheckOutput<T>>
where
    F: Fn(&CheckInput) -> Result<CheckOutput<T>>,
{
    if should_skip_check(id, &input.opts.check_sets) {
        return None;
    }

    // Check if an exemption has been declared for this check
    if let Some(exemption) = find_exemption(id, input.cm_md) {
        return Some(exemption.into());
    }

    // Call sync check function and wrap returned check output in an option
    match check_fn(input) {
        Ok(output) => Some(output),
        Err(err) => Some(CheckOutput {
            failed: true,
            fail_reason: Some(format!("{:#}", err)),
            ..Default::default()
        }),
    }
}

/// Wrapper function that takes care of running some common pre-check
/// operations and the asynchronous check function provided.
pub(crate) async fn run_async_check<'a, T, F, Fut>(
    id: &'static str,
    check_async_fn: F,
    input: &'a CheckInput<'a>,
) -> Option<CheckOutput<T>>
where
    F: Fn(&'a CheckInput<'a>) -> Fut,
    Fut: Future<Output = Result<CheckOutput<T>>>,
{
    if should_skip_check(id, &input.opts.check_sets) {
        return None;
    }

    // Check if an exemption has been declared for this check
    if let Some(exemption) = find_exemption(id, input.cm_md) {
        return Some(exemption.into());
    }

    // Call async check function and wrap returned check output in an option
    match check_async_fn(input).await {
        Ok(output) => Some(output),
        Err(err) => Some(CheckOutput {
            failed: true,
            fail_reason: Some(format!("{:#}", err)),
            ..Default::default()
        }),
    }
}

/// Adopters check.
pub(crate) fn adopters(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README file
    find_file_or_reference(input, &ADOPTERS_FILE, &*ADOPTERS_IN_README)
}

/// Analytics check.
pub(crate) async fn analytics(input: &CheckInput<'_>) -> Result<CheckOutput<Vec<String>>> {
    // Get website content
    let content = match &input.gh_md.homepage_url {
        Some(url) if !url.is_empty() => input.svc.http_client.get(url).send().await?.text().await?,
        _ => return Ok(false.into()),
    };

    let mut analytics_detected: Vec<String> = Vec::new();
    let mut details: String =
        "# Analytics providers detected in project's website \n\n".to_string();

    // Check Google Analytics 3 (Universal Analytics) tracking ID
    if GA3_IN_WEBSITE.is_match(&content) {
        analytics_detected.push("GA3".to_string());
        details.push_str("· Google Analytics 3 (Universal Analytics)\n")
    }

    // Check Google Analytics 4 measurement ID
    if GA4_IN_WEBSITE.is_match(&content) {
        analytics_detected.push("GA4".to_string());
        details.push_str("· Google Analytics 4\n")
    }

    // Check HubSpot tracking code
    if HUBSPOT_IN_WEBSITE.is_match(&content) {
        analytics_detected.push("HubSpot".to_string());
        details.push_str("· HubSpot\n")
    }

    // Return check output
    if !analytics_detected.is_empty() {
        return Ok(CheckOutput {
            passed: true,
            value: Some(analytics_detected),
            details: Some(details),
            ..Default::default()
        });
    }
    Ok(false.into())
}

/// Artifact Hub badge check.
pub(crate) fn artifacthub_badge(input: &CheckInput) -> Result<CheckOutput> {
    // Reference in README file
    Ok(CheckOutput::from_url(readme_capture(
        &input.opts.root,
        &[&*ARTIFACTHUB_URL],
    )?))
}

/// Binary artifacts check (from OpenSSF Scorecard).
pub(crate) fn binary_artifacts(input: &CheckInput) -> Result<CheckOutput> {
    Ok(match input.scorecard.get_check(BINARY_ARTIFACTS) {
        Some(sc_check) => sc_check.into(),
        None => false.into(),
    })
}

/// Branch protection check (from OpenSSF Scorecard).
pub(crate) fn branch_protection(input: &CheckInput) -> Result<CheckOutput> {
    Ok(match input.scorecard.get_check(BRANCH_PROTECTION) {
        Some(sc_check) => sc_check.into(),
        None => false.into(),
    })
}

/// Changelog check.
pub(crate) fn changelog(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README file
    let r = find_file_or_reference(input, &CHANGELOG_FILE, &*CHANGELOG_IN_README)?;
    if r.passed {
        return Ok(r);
    }

    // Reference in last release
    if github::latest_release_description_matches(input.gh_md, &*CHANGELOG_IN_GH_RELEASE) {
        return Ok(true.into());
    }

    Ok(false.into())
}

/// Contributor license agreement check.
pub(crate) fn cla(input: &CheckInput) -> Result<CheckOutput> {
    // CLA check in Github
    Ok(github::has_check(input.gh_md, &*CLA_IN_GH)?.into())
}

/// Code of conduct check.
pub(crate) fn code_of_conduct(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README file
    let r = find_file_or_reference(input, &CODE_OF_CONDUCT_FILE, &*CODE_OF_CONDUCT_IN_README)?;
    if r.passed {
        return Ok(r);
    }

    // File in Github (default community health file, for example)
    if let Some(coc) = &input.gh_md.code_of_conduct {
        return Ok(CheckOutput::from_url(coc.url.clone()));
    }

    Ok(false.into())
}

/// Code review check (from OpenSSF Scorecard).
pub(crate) fn code_review(input: &CheckInput) -> Result<CheckOutput> {
    Ok(match input.scorecard.get_check(CODE_REVIEW) {
        Some(sc_check) => sc_check.into(),
        None => false.into(),
    })
}

/// Community meeting check.
pub(crate) fn community_meeting(input: &CheckInput) -> Result<CheckOutput> {
    // Reference in README file
    Ok(readme_matches(&input.opts.root, &*COMMUNITY_MEETING_TEXT)?.into())
}

/// Contributing check.
pub(crate) async fn contributing(input: &CheckInput<'_>) -> Result<CheckOutput> {
    // File in repo or reference in README file
    let r = find_file_or_reference(input, &CONTRIBUTING_FILE, &*CONTRIBUTING_IN_README)?;
    if r.passed {
        return Ok(r);
    }

    // File in .github repo
    let url =
        github::has_community_health_file(&input.svc.http_client, "CONTRIBUTING.md", input.gh_md)
            .await?;
    Ok(CheckOutput::from_url(url))
}

/// Dangerous workflow check (from OpenSSF Scorecard).
pub(crate) fn dangerous_workflow(input: &CheckInput) -> Result<CheckOutput> {
    Ok(match input.scorecard.get_check(DANGEROUS_WORKFLOW) {
        Some(sc_check) => sc_check.into(),
        None => false.into(),
    })
}

/// Developer Certificate of Origin check.
pub(crate) fn dco(input: &CheckInput) -> Result<CheckOutput> {
    // DCO signature in commits
    if let Ok(passed) = git::commits_have_dco_signature(&input.opts.root) {
        if passed {
            return Ok(true.into());
        }
    }

    // DCO check in Github
    Ok(github::has_check(input.gh_md, &*DCO_IN_GH)?.into())
}

/// Dependency update tool check (from OpenSSF Scorecard).
pub(crate) fn dependency_update_tool(input: &CheckInput) -> Result<CheckOutput> {
    Ok(match input.scorecard.get_check(DEPENDENCY_UPDATE_TOOL) {
        Some(sc_check) => sc_check.into(),
        None => false.into(),
    })
}

/// GitHub discussions check.
pub(crate) fn github_discussions(input: &CheckInput) -> Result<CheckOutput> {
    if let Some(latest_discussion) = input
        .gh_md
        .discussions
        .nodes
        .as_ref()
        .and_then(|nodes| nodes.iter().flatten().next())
    {
        let created_at = OffsetDateTime::parse(&latest_discussion.created_at, &Rfc3339)?;
        let one_year_ago = (OffsetDateTime::now_utc() - Duration::days(365)).unix_timestamp();
        if created_at.unix_timestamp() > one_year_ago {
            return Ok(CheckOutput::from_url(Some(latest_discussion.url.clone())));
        }
    }
    Ok(false.into())
}

/// Governance check.
pub(crate) fn governance(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README file
    find_file_or_reference(input, &GOVERNANCE_FILE, &*GOVERNANCE_IN_README)
}

/// License check.
pub(crate) fn license(input: &CheckInput) -> Result<CheckOutput<String>> {
    // File in repo
    if let Some(spdx_id) = license::detect(&Globs {
        root: &input.opts.root,
        patterns: &LICENSE_FILE,
        case_sensitive: true,
    })? {
        return Ok(Some(spdx_id).into());
    }

    // License detected by Github
    if let Some(spdx_id) = input
        .gh_md
        .license_info
        .as_ref()
        .and_then(|l| l.spdx_id.as_ref())
    {
        if spdx_id != "NOASSERTION" {
            return Ok(Some(spdx_id.to_owned()).into());
        }
    }

    Ok(false.into())
}

/// Approved license check.
pub(crate) fn license_approved(
    spdx_id: Option<String>,
    input: &CheckInput,
) -> Option<CheckOutput<bool>> {
    if should_skip_check(LICENSE_APPROVED, &input.opts.check_sets) {
        return None;
    }

    // Check if an exemption has been declared for this check
    if let Some(exemption) = find_exemption(LICENSE_APPROVED, input.cm_md) {
        return Some(exemption.into());
    }

    // SPDX id in list of approved licenses
    let mut approved: Option<bool> = None;
    if let Some(spdx_id) = &spdx_id {
        approved = Some(license::is_approved(spdx_id))
    }

    Some(CheckOutput {
        passed: approved.unwrap_or(false),
        value: approved,
        ..Default::default()
    })
}

/// License scanning check.
pub(crate) fn license_scanning(input: &CheckInput) -> Result<CheckOutput> {
    // Scanning url in metadata file
    if let Some(url) = input
        .cm_md
        .as_ref()
        .and_then(|md| md.license_scanning.as_ref())
        .and_then(|ls| ls.url.as_ref())
    {
        return Ok(CheckOutput::from_url(Some(url.to_owned())));
    }

    // Reference in README file
    if let Some(url) = content::find(&readme_globs(&input.opts.root), &[&*FOSSA_URL, &*SNYK_URL])? {
        return Ok(CheckOutput::from_url(Some(url)));
    };

    Ok(false.into())
}

/// Maintained check (from OpenSSF Scorecard).
pub(crate) fn maintained(input: &CheckInput) -> Result<CheckOutput> {
    Ok(match input.scorecard.get_check(MAINTAINED) {
        Some(sc_check) => sc_check.into(),
        None => false.into(),
    })
}

/// Maintainers check.
pub(crate) fn maintainers(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README file
    find_file_or_reference(input, &MAINTAINERS_FILE, &*MAINTAINERS_IN_README)
}

/// OpenSSF badge check.
pub(crate) fn openssf_badge(input: &CheckInput) -> Result<CheckOutput> {
    // Reference in README file
    Ok(CheckOutput::from_url(readme_capture(
        &input.opts.root,
        &[&*OPENSSF_URL],
    )?))
}

/// Recent release check.
pub(crate) fn recent_release(input: &CheckInput) -> Result<CheckOutput> {
    if let Some(latest_release) = github::latest_release(input.gh_md) {
        let created_at = OffsetDateTime::parse(&latest_release.created_at, &Rfc3339)?;
        let one_year_ago = (OffsetDateTime::now_utc() - Duration::days(365)).unix_timestamp();
        if created_at.unix_timestamp() > one_year_ago {
            return Ok(CheckOutput::from_url(Some(latest_release.url.clone())));
        }
    }
    Ok(false.into())
}

/// Roadmap check.
pub(crate) fn roadmap(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README
    find_file_or_reference(input, &ROADMAP_FILE, &*ROADMAP_IN_README)
}

/// Readme check.
pub(crate) fn readme(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo
    if let Some(path) = path::find(&readme_globs(&input.opts.root))? {
        return Ok(CheckOutput::from_path(Some(&path), input.gh_md));
    }

    Ok(false.into())
}

/// Software bill of materials (SBOM).
pub(crate) fn sbom(input: &CheckInput) -> Result<CheckOutput> {
    // Asset in last release
    if let Some(true) = github::latest_release(input.gh_md)
        .and_then(|r| r.release_assets.nodes.as_ref())
        .map(|assets| {
            assets
                .iter()
                .flatten()
                .any(|asset| SBOM_IN_GH_RELEASE.is_match(&asset.name))
        })
    {
        return Ok(true.into());
    }

    // Reference in README file
    Ok(readme_matches(&input.opts.root, &*SBOM_IN_README)?.into())
}

/// Security policy check.
pub(crate) fn security_policy(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README file
    let r = find_file_or_reference(input, &SECURITY_POLICY_FILE, &*SECURITY_POLICY_IN_README)?;
    if r.passed {
        return Ok(r);
    }

    // File in Github (default community health file, for example)
    Ok(CheckOutput::from_url(
        input.gh_md.security_policy_url.clone(),
    ))
}

/// Signed releases check (from OpenSSF Scorecard).
pub(crate) fn signed_releases(input: &CheckInput) -> Result<CheckOutput> {
    Ok(match input.scorecard.get_check(SIGNED_RELEASES) {
        Some(sc_check) => sc_check.into(),
        None => false.into(),
    })
}

/// Slack presence check.
pub(crate) fn slack_presence(input: &CheckInput) -> Result<CheckOutput> {
    // Reference in README file
    Ok(readme_matches(&input.opts.root, &*SLACK_IN_README)?.into())
}

/// Token permissions check (from OpenSSF Scorecard).
pub(crate) fn token_permissions(input: &CheckInput) -> Result<CheckOutput> {
    Ok(match input.scorecard.get_check(TOKEN_PERMISSIONS) {
        Some(sc_check) => sc_check.into(),
        None => false.into(),
    })
}

/// Trademark disclaimer check.
pub(crate) async fn trademark_disclaimer(input: &CheckInput<'_>) -> Result<CheckOutput> {
    // Trademark disclaimer in website setup in Github
    if let Some(url) = &input.gh_md.homepage_url {
        if !url.is_empty() {
            return Ok(content::remote_matches(
                &input.svc.http_client,
                url,
                &*TRADEMARK_DISCLAIMER_IN_WEBSITE,
            )
            .await?
            .into());
        }
    }

    Ok(false.into())
}

/// Vulnerabilities check (from OpenSSF Scorecard).
pub(crate) fn vulnerabilities(input: &CheckInput) -> Result<CheckOutput> {
    Ok(match input.scorecard.get_check(VULNERABILITIES) {
        Some(sc_check) => sc_check.into(),
        None => false.into(),
    })
}

/// Website check.
pub(crate) fn website(input: &CheckInput) -> Result<CheckOutput> {
    // Website in Github
    if let Some(url) = &input.gh_md.homepage_url {
        if !url.is_empty() {
            return Ok(CheckOutput::from_url(Some(url.to_string())));
        }
    }

    Ok(false.into())
}

/// Check if the check provided should be skipped.
fn should_skip_check(check_id: &str, check_sets: &[CheckSet]) -> bool {
    // Skip if the check doesn't belong to any of the check sets provided
    if !check_sets.iter().any(|k| CHECKSET[k].contains(&check_id)) {
        return true;
    }

    false
}

/// Check if the repository is exempt from passing the provided check.
fn find_exemption(check_id: &str, cm_md: Option<&Metadata>) -> Option<Exemption> {
    if let Some(exemption) = cm_md
        .as_ref()
        .and_then(|md| md.exemptions.as_ref())
        .and_then(|exemptions| {
            exemptions
                .iter()
                .find(|exemption| exemption.check == check_id)
        })
    {
        if !exemption.reason.is_empty() {
            return Some(exemption.to_owned());
        }
    }

    None
}

/// Check if a file matching the patterns provided is found in the repo or if
/// any of the regular expressions provided matches the README file content.
fn find_file_or_reference(
    input: &CheckInput,
    patterns: &[&str],
    re: &RegexSet,
) -> Result<CheckOutput> {
    // File in repo
    if let Some(path) = path::find(&Globs {
        root: &input.opts.root,
        patterns,
        case_sensitive: false,
    })? {
        return Ok(CheckOutput::from_path(Some(&path), input.gh_md));
    }

    // Reference in README file
    if readme_matches(&input.opts.root, re)? {
        return Ok(true.into());
    }

    Ok(false.into())
}

/// Check if the README file content matches any of the regular expressions
/// provided.
fn readme_matches(root: &Path, re: &RegexSet) -> Result<bool> {
    content::matches(&readme_globs(root), re)
}

/// Check if the README file content matches any of the regular expressions
/// provided, returning the value from the first capture group.
fn readme_capture(root: &Path, regexps: &[&Regex]) -> Result<Option<String>> {
    content::find(&readme_globs(root), regexps)
}

// Returns a Globs instance used to locate the README file.
fn readme_globs(root: &Path) -> Globs {
    Globs {
        root,
        patterns: &README_FILE,
        case_sensitive: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linter::check::scorecard::ScorecardCheckDocs;
    use github::md::*;

    #[test]
    fn check_output_from_passed() {
        assert_eq!(
            CheckOutput::<()>::from(true),
            CheckOutput {
                passed: true,
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_output_from_value_some() {
        assert_eq!(
            CheckOutput::from(Some("value".to_string())),
            CheckOutput {
                passed: true,
                value: Some("value".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_output_from_value_none() {
        assert_eq!(
            CheckOutput::<()>::from(None),
            CheckOutput {
                passed: false,
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_output_from_exemption() {
        let exemption = Exemption {
            check: "test".to_string(),
            reason: "test".to_string(),
        };

        assert_eq!(
            CheckOutput::<()>::from(exemption),
            CheckOutput {
                exempt: true,
                exemption_reason: Some("test".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_output_from_scorecard_check_passed() {
        let sc_check = ScorecardCheck {
            name: "Code-Review".to_string(),
            reason: "reason".to_string(),
            details: Some(vec!["details".to_string()]),
            score: 8.0,
            documentation: ScorecardCheckDocs {
                url: "https://test.url".to_string(),
            },
        };

        assert_eq!(
            CheckOutput::<()>::from(&sc_check),
            CheckOutput {
                passed: true,
                details: Some("# Code-Review OpenSSF Scorecard check\n\n**Score**: 8 (check passes with score >= 5)\n\n**Reason**: reason\n\n**Details**: \n\n>details\n\n**Please see the [check documentation](https://test.url) in the ossf/scorecard repository for more details**".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_output_from_scorecard_check_not_passed() {
        let sc_check = ScorecardCheck {
            name: "Code-Review".to_string(),
            reason: "reason".to_string(),
            details: Some(vec!["details".to_string()]),
            score: 4.0,
            documentation: ScorecardCheckDocs {
                url: "https://test.url".to_string(),
            },
        };

        assert_eq!(
            CheckOutput::<()>::from(&sc_check),
            CheckOutput {
                passed: false,
                details: Some("# Code-Review OpenSSF Scorecard check\n\n**Score**: 4 (check passes with score >= 5)\n\n**Reason**: reason\n\n**Details**: \n\n>details\n\n**Please see the [check documentation](https://test.url) in the ossf/scorecard repository for more details**".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_output_from_url_some() {
        assert_eq!(
            CheckOutput::<()>::from_url(Some("url".to_string())),
            CheckOutput {
                passed: true,
                url: Some("url".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_output_from_url_none() {
        assert_eq!(
            CheckOutput::<()>::from_url(None),
            CheckOutput {
                passed: false,
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_output_from_path_some() {
        let gh_md = MdRepository {
            name: "repo".to_string(),
            owner: MdRepositoryOwner {
                login: "owner".to_string(),
                on: MdRepositoryOwnerOn::Organization,
            },
            ..MdRepository::default()
        };

        assert_eq!(
            CheckOutput::<()>::from_path(Some(&PathBuf::from("path")), &gh_md),
            CheckOutput {
                passed: true,
                url: Some("https://github.com/owner/repo/blob/master/path".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn check_output_from_path_none() {
        assert_eq!(
            CheckOutput::<()>::from_path(None, &MdRepository::default()),
            CheckOutput {
                passed: false,
                ..Default::default()
            }
        );
    }
}
