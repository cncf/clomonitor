use anyhow::Error;
use askalono::*;
use chrono::{Duration, Utc};
use glob::{glob_with, MatchOptions, PatternError};
use lazy_static::lazy_static;
use regex::{Regex, RegexSet};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::error;

/// SPDX licenses data. Used to detect license used by repositories.
const LICENSES_DATA: &[u8] = include_bytes!("data/licenses.bin.zstd");

/// CNCF approved licenses.
/// https://github.com/cncf/foundation/blob/master/allowed-third-party-license-policy.md
static APPROVED_LICENSES: [&str; 10] = [
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-2-Clause-FreeBSD",
    "BSD-3-Clause",
    "ISC",
    "MIT",
    "PostgreSQL",
    "Python-2.0",
    "X11",
    "Zlib",
];

/// Glob matching configuration.
#[derive(Debug)]
pub(crate) struct Globs<'a, P>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
{
    pub root: &'a Path,
    pub patterns: P,
    pub case_sensitive: bool,
}

/// Check if the content of any of the files that match the globs provided
/// matches any of the regular expressions given.
pub(crate) fn content_matches<P, R>(globs: Globs<P>, regexps: R) -> Result<bool, Error>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
    R: IntoIterator,
    R::Item: AsRef<str>,
{
    let re = RegexSet::new(regexps)?;
    Ok(matching_paths(globs)?.iter().any(|path| {
        if let Ok(content) = fs::read_to_string(path) {
            return re.is_match(&content);
        }
        false
    }))
}

/// Check if the repository has released a new version in the last year.
pub(crate) async fn has_recent_release(repo_url: &str) -> Option<bool> {
    let (owner, repo) = get_owner_and_repo(repo_url)?;
    let github = octocrab::instance();
    let mut page = github
        .repos(&owner, &repo)
        .releases()
        .list()
        .per_page(1)
        .send()
        .await
        .ok()?;
    let releases = page.take_items();
    let last_release = releases.first()?;
    Some(last_release.created_at? > Utc::now() - Duration::days(365))
}

/// Check if the project has added a website to the Github repository.
pub(crate) async fn has_website(repo_url: &str) -> bool {
    get_website(repo_url).await.is_some()
}

/// Check if the license provided is an approved one.
pub(crate) fn is_approved_license(spdx_id: &str) -> bool {
    APPROVED_LICENSES.contains(&spdx_id)
}

/// Check repository's license and return its SPDX id if possible.
pub(crate) fn license<P>(globs: Globs<P>) -> Result<Option<String>, Error>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
{
    lazy_static! {
        static ref LICENSES: Store = Store::from_cache(LICENSES_DATA).unwrap();
    }
    let mut spdx_id: Option<String> = None;
    matching_paths(globs)?.iter().any(|path| {
        if let Ok(content) = fs::read_to_string(path) {
            let m = LICENSES.analyze(&TextData::from(content));
            if m.score > 0.9 {
                spdx_id = Some(m.name.to_string());
                return true;
            }
        }
        false
    });
    Ok(spdx_id)
}

/// Check if exists at least a path that matches the globs provided.
pub(crate) fn path_exists<P>(globs: Globs<P>) -> Result<bool, PatternError>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
{
    Ok(!matching_paths(globs)?.is_empty())
}

/// Return all paths that match any of the globs provided.
fn matching_paths<P>(globs: Globs<P>) -> Result<Vec<PathBuf>, PatternError>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
{
    let options = MatchOptions {
        case_sensitive: globs.case_sensitive,
        ..Default::default()
    };
    globs
        .patterns
        .into_iter()
        .map(|pattern| globs.root.join(pattern.as_ref()))
        .map(|pattern| pattern.to_string_lossy().into_owned())
        .try_fold(Vec::new(), |mut paths, pattern| {
            match glob_with(&pattern, options) {
                Ok(pattern_paths) => {
                    paths.extend(pattern_paths.filter_map(Result::ok));
                    Ok(paths)
                }
                Err(err) => Err(err),
            }
        })
}

/// Extract the owner and repository from the repository url provided.
fn get_owner_and_repo(url: &str) -> Option<(String, String)> {
    lazy_static! {
        static ref GITHUB_RE: Regex =
            Regex::new("^https://github.com/(?P<org>.+)/(?P<repo>.+)$").unwrap();
    }
    let c = GITHUB_RE.captures(url)?;
    Some((c["org"].to_string(), c["repo"].to_string()))
}

/// Get project's website from Github repository.
async fn get_website(url: &str) -> Option<String> {
    let (owner, repo) = get_owner_and_repo(url)?;
    let github = octocrab::instance();
    match github.repos(&owner, &repo).get().await {
        Ok(repo) => none_if_empty(repo.homepage),
        Err(err) => {
            error!("error getting repository {owner}/{repo}: {err}");
            None
        }
    }
}

/// Helper function that returns None if the provided Option<String> has some
/// value on it but it's empty.
fn none_if_empty(o: Option<String>) -> Option<String> {
    let value = o?;
    if value.is_empty() {
        return None;
    }
    Some(value)
}
