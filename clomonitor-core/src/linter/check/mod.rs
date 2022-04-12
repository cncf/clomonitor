use self::path::Globs;
use super::{LintOptions, LintServices};
use crate::{config::*, linter::CheckSet};
use anyhow::Result;
use metadata::{Exemption, Metadata};
use octocrab::models::Repository;
use patterns::*;
use regex::{Regex, RegexSet};
use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    path::{Path, PathBuf},
};

pub(crate) mod content;
pub(crate) mod github;
pub(crate) mod license;
pub(crate) mod metadata;
pub(crate) mod path;
pub(crate) mod patterns;

/// Input used by checks to perform their operations.
#[derive(Debug)]
#[non_exhaustive]
pub(crate) struct CheckInput<'a> {
    pub opts: &'a LintOptions,
    pub svc: &'a LintServices,
    pub cm_md: Option<Metadata>,
    pub gh_md: Repository,
}

/// Check output information.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckOutput<T = ()> {
    pub passed: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,

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
    pub(crate) fn from_path(path: Option<PathBuf>, gh_md: &Repository) -> Self {
        match path {
            Some(path) => {
                let url = github::build_url(
                    &path,
                    &gh_md.owner.as_ref().unwrap().login,
                    &gh_md.name,
                    gh_md.default_branch.as_ref().unwrap(),
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
    if let Some(exemption) = find_exemption(id, &input.cm_md) {
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
    if let Some(exemption) = find_exemption(id, &input.cm_md) {
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

/// Artifact Hub badge check.
pub(crate) fn artifacthub_badge(input: &CheckInput) -> Result<CheckOutput> {
    // Reference in README file
    Ok(CheckOutput::from_url(readme_capture(
        &input.opts.root,
        vec![&*ARTIFACTHUB_URL],
    )?))
}

/// Changelog check.
pub(crate) async fn changelog(input: &CheckInput<'_>) -> Result<CheckOutput> {
    // File in repo or reference in README file
    let r = find_file_or_reference(input, &CHANGELOG_FILE, &*CHANGELOG_IN_README)?;
    if r.passed {
        return Ok(r);
    }

    // Reference in last release
    if github::last_release_body_matches(
        &input.svc.github_client,
        &input.opts.url,
        &*CHANGELOG_IN_GH_RELEASE,
    )
    .await?
    {
        return Ok(true.into());
    }

    Ok(false.into())
}

/// Contributor license agreement check.
pub(crate) async fn cla(input: &CheckInput<'_>) -> Result<CheckOutput> {
    // CLA check in Github
    Ok(
        github::has_check(&input.svc.github_client, &input.opts.url, &*CLA_IN_GH)
            .await?
            .into(),
    )
}

/// Code of conduct check.
pub(crate) async fn code_of_conduct(input: &CheckInput<'_>) -> Result<CheckOutput> {
    // File in repo or reference in README file
    let r = find_file_or_reference(input, &CODE_OF_CONDUCT_FILE, &*CODE_OF_CONDUCT_IN_README)?;
    if r.passed {
        return Ok(r);
    }

    // File in .github repo
    let url = github::has_community_health_file(
        &input.svc.github_client,
        &input.svc.http_client,
        "CODE_OF_CONDUCT.md",
        &input.gh_md,
    )
    .await?;
    Ok(CheckOutput::from_url(url))
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
    let url = github::has_community_health_file(
        &input.svc.github_client,
        &input.svc.http_client,
        "CONTRIBUTING.md",
        &input.gh_md,
    )
    .await?;
    Ok(CheckOutput::from_url(url))
}

/// Developer Certificate of Origin check.
pub(crate) async fn dco(input: &CheckInput<'_>) -> Result<CheckOutput> {
    // DCO check in Github
    Ok(
        github::has_check(&input.svc.github_client, &input.opts.url, &*DCO_IN_GH)
            .await?
            .into(),
    )
}

/// Governance check.
pub(crate) fn governance(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README file
    find_file_or_reference(input, &GOVERNANCE_FILE, &*GOVERNANCE_IN_README)
}

/// License check.
pub(crate) fn license(input: &CheckInput) -> Result<CheckOutput<String>> {
    // File in repo
    if let Some(spdx_id) = license::detect(Globs {
        root: &input.opts.root,
        patterns: &LICENSE_FILE,
        case_sensitive: true,
    })? {
        return Ok(Some(spdx_id).into());
    }

    // License detected by Github
    if let Some(license) = &input.gh_md.license {
        if license.spdx_id != "NOASSERTION" {
            return Ok(Some(license.spdx_id.to_owned()).into());
        }
    }

    Ok(false.into())
}

/// Approved license check.
pub(crate) fn license_approved(
    spdx_id: &Option<String>,
    input: &CheckInput,
) -> Option<CheckOutput<bool>> {
    if should_skip_check(LICENSE_APPROVED, &input.opts.check_sets) {
        return None;
    }

    // Check if an exemption has been declared for this check
    if let Some(exemption) = find_exemption(LICENSE_APPROVED, &input.cm_md) {
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
    if let Some(md) = &input.cm_md {
        if let Some(license_scanning) = &md.license_scanning {
            if let Some(url) = &license_scanning.url {
                return Ok(CheckOutput::from_url(Some(url.to_owned())));
            }
        }
    }

    // Reference in README file
    if let Some(url) = content::find(
        readme_globs(&input.opts.root),
        vec![&*FOSSA_URL, &*SNYK_URL],
    )? {
        return Ok(CheckOutput::from_url(Some(url)));
    };

    Ok(false.into())
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
        vec![&*OPENSSF_URL],
    )?))
}

/// Recent release check.
pub(crate) async fn recent_release(input: &CheckInput<'_>) -> Result<CheckOutput> {
    Ok(CheckOutput::from_url(
        github::has_recent_release(&input.svc.github_client, &input.opts.url).await?,
    ))
}

/// Roadmap check.
pub(crate) fn roadmap(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README
    find_file_or_reference(input, &ROADMAP_FILE, &*ROADMAP_IN_README)
}

/// Readme check.
pub(crate) fn readme(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo
    if let Some(path) = path::find(readme_globs(&input.opts.root))? {
        return Ok(CheckOutput::from_path(Some(path), &input.gh_md));
    }

    Ok(false.into())
}

/// Software bill of materials (SBOM).
pub(crate) async fn sbom(input: &CheckInput<'_>) -> Result<CheckOutput> {
    // Asset in last release
    if let Some(last_release) =
        github::last_release(&input.svc.github_client, &input.opts.url).await?
    {
        if last_release
            .assets
            .iter()
            .any(|asset| SBOM_IN_GH_RELEASE.is_match(&asset.name))
        {
            return Ok(true.into());
        }
    }

    // Reference in README file
    Ok(readme_matches(&input.opts.root, &*SBOM_IN_README)?.into())
}

/// Security policy check.
pub(crate) async fn security_policy(input: &CheckInput<'_>) -> Result<CheckOutput> {
    // File in repo or reference in README file
    let r = find_file_or_reference(input, &SECURITY_POLICY_FILE, &*SECURITY_POLICY_IN_README)?;
    if r.passed {
        return Ok(r);
    }

    // File in .github repo
    let url = github::has_community_health_file(
        &input.svc.github_client,
        &input.svc.http_client,
        "SECURITY.md",
        &input.gh_md,
    )
    .await?;
    Ok(CheckOutput::from_url(url))
}

/// Slack presence check.
pub(crate) fn slack_presence(input: &CheckInput) -> Result<CheckOutput> {
    // Reference in README file
    Ok(readme_matches(&input.opts.root, &*SLACK_IN_README)?.into())
}

/// Trademark disclaimer check.
pub(crate) async fn trademark_disclaimer(input: &CheckInput<'_>) -> Result<CheckOutput> {
    // Trademark disclaimer in website setup in Github
    if let Some(url) = &input.gh_md.homepage {
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

/// Website check.
pub(crate) fn website(input: &CheckInput) -> Result<CheckOutput> {
    // Website in Github
    if let Some(url) = &input.gh_md.homepage {
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
fn find_exemption(check_id: &str, cm_md: &Option<Metadata>) -> Option<Exemption> {
    if let Some(md) = cm_md {
        if let Some(exemptions) = &md.exemptions {
            if let Some(exemption) = exemptions.iter().find(|e| e.check == check_id) {
                if !exemption.reason.is_empty() && exemption.reason != "~" {
                    return Some(exemption.clone());
                }
            }
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
    if let Some(path) = path::find(Globs {
        root: &input.opts.root,
        patterns,
        case_sensitive: false,
    })? {
        return Ok(CheckOutput::from_path(Some(path), &input.gh_md));
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
    content::matches(readme_globs(root), re)
}

/// Check if the README file content matches any of the regular expressions
/// provided, returning the value from the first capture group.
fn readme_capture(root: &Path, regexps: Vec<&Regex>) -> Result<Option<String>> {
    content::find(readme_globs(root), regexps)
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
}
