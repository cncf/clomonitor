use super::{
    content, github,
    path::{self, Globs},
};
use crate::linter::{
    check::{CheckInput, CheckOutput},
    checks::readme,
    metadata::{Exemption, Metadata},
    CheckSet, CHECKS,
};
use anyhow::Result;
use regex::{Regex, RegexSet};
use std::path::Path;

/// Check if a file matching the patterns provided is found in the repo or if
/// any of the regular expressions provided matches the README file content.
pub(crate) fn find_file_or_readme_ref(
    input: &CheckInput,
    patterns: &[&str],
    re: &RegexSet,
) -> Result<CheckOutput> {
    // File in repo
    if let Some(path) = path::find(&Globs {
        root: &input.li.root,
        patterns,
        case_sensitive: false,
    })? {
        let url = github::build_url(
            &path,
            &input.gh_md.owner.login,
            &input.gh_md.name,
            &github::default_branch(input.gh_md.default_branch_ref.as_ref()),
        );
        return Ok(CheckOutput::passed().url(Some(url)));
    }

    // Reference in README file
    if readme_matches(&input.li.root, re)? {
        return Ok(CheckOutput::passed());
    }

    Ok(CheckOutput::not_passed())
}

/// Check if the README file content matches any of the regular expressions
/// provided.
pub(crate) fn readme_matches(root: &Path, re: &RegexSet) -> Result<bool> {
    content::matches(&readme_globs(root), re)
}

/// Check if the README file content matches any of the regular expressions
/// provided, returning the value from the first capture group.
pub(crate) fn readme_capture(root: &Path, regexps: &[&Regex]) -> Result<Option<String>> {
    content::find(&readme_globs(root), regexps)
}

// Returns a Globs instance used to locate the README file.
pub(crate) fn readme_globs(root: &Path) -> Globs {
    Globs {
        root,
        patterns: &readme::FILE_PATTERNS,
        case_sensitive: true,
    }
}

/// Check if the repository is exempt from passing the provided check.
pub(crate) fn find_exemption(check_id: &str, cm_md: Option<&Metadata>) -> Option<Exemption> {
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

/// Check if the check provided should be skipped.
pub(crate) fn should_skip_check(check_id: &str, check_sets: &[CheckSet]) -> bool {
    // Skip if the check doesn't belong to any of the check sets provided
    if !CHECKS[check_id]
        .check_sets
        .iter()
        .any(|check_set| check_sets.contains(check_set))
    {
        return true;
    }

    false
}
