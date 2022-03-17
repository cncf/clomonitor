use super::{content, patterns::*};
use anyhow::{format_err, Error};
use chrono::{Duration, Utc};
use lazy_static::lazy_static;
use octocrab::{
    models::{repos::Release, Repository},
    params::State,
};
use regex::{Regex, RegexSet};
use std::path::Path;

/// Build a url from the path and metadata provided.
pub(crate) fn build_url(path: &Path, owner: &str, repo: &str, branch: &str) -> String {
    format!(
        "https://github.com/{}/{}/blob/{}/{}",
        owner,
        repo,
        branch,
        path.to_string_lossy(),
    )
}

/// Get repository's metadata from the Github API.
pub(crate) async fn get_repo_metadata(repo_url: &str) -> Result<Repository, Error> {
    let (owner, repo) = get_owner_and_repo(repo_url)?;
    let github = octocrab::instance();
    match github.repos(&owner, &repo).get().await {
        Ok(repo) => Ok(repo),
        Err(err) => Err(err.into()),
    }
}

/// Check if the given default community health file is available in the
/// .github repository, returning the url to the file when found.
pub(crate) async fn has_community_health_file(
    file: &str,
    gh_md: &Repository,
) -> Result<Option<String>, Error> {
    lazy_static! {
        static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
    }
    let raw_url = format!(
        "https://raw.githubusercontent.com/{}/.github/{}/{}",
        gh_md.owner.as_ref().unwrap().login,
        gh_md.default_branch.as_ref().unwrap(),
        file
    );
    match HTTP_CLIENT.head(raw_url).send().await?.status() {
        http::StatusCode::OK => {
            let url = build_url(
                Path::new(file),
                &gh_md.owner.as_ref().unwrap().login,
                ".github",
                gh_md.default_branch.as_ref().unwrap(),
            );
            Ok(Some(url))
        }
        _ => Ok(None),
    }
}

/// Check if the repository has released a new version in the last year.
pub(crate) async fn has_recent_release(repo_url: &str) -> Result<Option<String>, Error> {
    if let Some(last_release) = last_release(repo_url).await? {
        if let Some(created_at) = last_release.created_at {
            if created_at > Utc::now() - Duration::days(365) {
                return Ok(Some(last_release.html_url.into()));
            }
        }
    }
    Ok(None)
}

/// Check if the last PR in the repository has the DCO check.
pub(crate) async fn last_pr_has_dco_check(repo_url: &str) -> Result<bool, Error> {
    let (owner, repo) = get_owner_and_repo(repo_url)?;
    let github = octocrab::instance();
    let mut page = github
        .pulls(&owner, &repo)
        .list()
        .state(State::Closed)
        .per_page(1)
        .send()
        .await?;
    Ok(match page.take_items().first() {
        Some(pr) => {
            let checks_url = format!(
                "https://github.com/{}/{}/pull/{}/checks",
                &owner, &repo, pr.number
            );
            content::remote_matches(&checks_url, &*DCO).await?
        }
        None => false,
    })
}

/// Check if the last release body matches any of the regular expressions
/// provided.
pub(crate) async fn last_release_body_matches(
    repo_url: &str,
    re: &RegexSet,
) -> Result<bool, Error> {
    if let Some(last_release) = last_release(repo_url).await? {
        if let Some(body) = last_release.body {
            return Ok(re.is_match(&body));
        }
    }
    Ok(false)
}

/// Extract the owner and repository from the repository url provided.
fn get_owner_and_repo(repo_url: &str) -> Result<(String, String), Error> {
    lazy_static! {
        static ref GITHUB_REPO_URL_RE: Regex =
            Regex::new("^https://github.com/(?P<org>[^/]+)/(?P<repo>[^/]+)/?$").unwrap();
    }
    let c = GITHUB_REPO_URL_RE
        .captures(repo_url)
        .ok_or(format_err!("invalid repository url"))?;
    Ok((c["org"].to_string(), c["repo"].to_string()))
}

/// Return the last release of the provided repository when available.
async fn last_release(repo_url: &str) -> Result<Option<Release>, Error> {
    let (owner, repo) = get_owner_and_repo(repo_url)?;
    let github = octocrab::instance();
    let mut page = github
        .repos(&owner, &repo)
        .releases()
        .list()
        .per_page(1)
        .send()
        .await?;
    Ok(page.take_items().first().cloned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_url_works() {
        assert_eq!(
            build_url(Path::new("path/test.md"), "owner", "repo", "main"),
            "https://github.com/owner/repo/blob/main/path/test.md".to_string()
        );
    }

    #[test]
    fn get_owner_and_repo_valid_url() {
        assert_eq!(
            get_owner_and_repo("https://github.com/org/repo").unwrap(),
            ("org".to_string(), "repo".to_string())
        );
    }

    #[test]
    fn get_owner_and_repo_valid_url_trailing_slash() {
        assert_eq!(
            get_owner_and_repo("https://github.com/org/repo/").unwrap(),
            ("org".to_string(), "repo".to_string())
        );
    }

    #[test]
    fn get_owner_and_repo_invalid_url() {
        assert!(matches!(
            get_owner_and_repo("https://github.com/org"),
            Err(_)
        ));
    }
}
