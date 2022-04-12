use super::patterns::GITHUB_REPO_URL;
use anyhow::{format_err, Context, Result};
use cached::proc_macro::cached;
use chrono::{Duration, Utc};
use octocrab::{
    models::{pulls::PullRequest, repos::Release, Repository, Status},
    params::State,
    Octocrab,
};
use regex::RegexSet;
use serde::Deserialize;
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
#[cached(
    result = true,
    sync_writes = true,
    key = "String",
    convert = r#"{ format!("{}", repo_url) }"#
)]
pub(crate) async fn get_repo_metadata(
    github_client: &Octocrab,
    repo_url: &str,
) -> Result<Repository> {
    let (owner, repo) = get_owner_and_repo(repo_url)?;
    github_client
        .repos(&owner, &repo)
        .get()
        .await
        .context("error getting repository metadata")
}

/// Check if the repo has a check that matches any of the regular expressions
/// provided.
pub(crate) async fn has_check(
    github_client: &Octocrab,
    repo_url: &str,
    re: &RegexSet,
) -> Result<bool> {
    let (owner, repo) = get_owner_and_repo(repo_url)?;

    // Get last closed PR head commit sha
    let sha = match get_last_closed_pr(github_client, &owner, &repo).await? {
        Some(pr) => pr.head.sha,
        None => return Ok(false),
    };

    // Search in check suites
    if get_commit_check_suites(github_client, &owner, &repo, &sha)
        .await?
        .iter()
        .any(|s| re.is_match(&s.app.name))
    {
        return Ok(true);
    }

    // Search in commit statuses
    if get_commit_statuses(github_client, &owner, &repo, &sha)
        .await?
        .iter()
        .filter(|s| s.context.is_some())
        .any(|s| re.is_match(s.context.as_ref().unwrap()))
    {
        return Ok(true);
    }

    // Search in check runs
    if get_commit_check_runs(github_client, &owner, &repo, &sha)
        .await?
        .iter()
        .any(|r| re.is_match(&r.name))
    {
        return Ok(true);
    }

    Ok(false)
}

/// Check if the given default community health file is available in the
/// .github repository, returning the url to the file when found.
pub(crate) async fn has_community_health_file(
    github_client: &Octocrab,
    http_client: &reqwest::Client,
    file: &str,
    gh_md: &Repository,
) -> Result<Option<String>> {
    // Get community health files repository metadata
    let community_repo_url = format!(
        "https://github.com/{}/.github",
        gh_md.owner.as_ref().unwrap().login
    );
    let community_repo = match get_repo_metadata(github_client, &community_repo_url).await {
        Ok(repo) => repo,
        Err(_) => return Ok(None),
    };

    // Check if the file is in the repo
    let file_raw_url = format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}",
        &community_repo.owner.as_ref().unwrap().login,
        &community_repo.name,
        community_repo.default_branch.as_ref().unwrap(),
        file
    );
    match http_client
        .head(&file_raw_url)
        .send()
        .await
        .context(format!(
            "error checking community health file {}",
            &file_raw_url
        ))?
        .status()
    {
        http::StatusCode::OK => {
            let url = build_url(
                Path::new(file),
                &community_repo.owner.as_ref().unwrap().login,
                &community_repo.name,
                community_repo.default_branch.as_ref().unwrap(),
            );
            Ok(Some(url))
        }
        _ => Ok(None),
    }
}

/// Check if the repository has released a new version in the last year.
pub(crate) async fn has_recent_release(
    github_client: &Octocrab,
    repo_url: &str,
) -> Result<Option<String>> {
    if let Some(last_release) = last_release(github_client, repo_url).await? {
        if let Some(created_at) = last_release.created_at {
            if created_at > Utc::now() - Duration::days(365) {
                return Ok(Some(last_release.html_url.into()));
            }
        }
    }
    Ok(None)
}

/// Return the last release of the provided repository when available.
#[cached(
    result = true,
    sync_writes = true,
    key = "String",
    convert = r#"{ format!("{}", repo_url) }"#
)]
pub(crate) async fn last_release(
    github_client: &Octocrab,
    repo_url: &str,
) -> Result<Option<Release>> {
    let (owner, repo) = get_owner_and_repo(repo_url)?;
    let mut page = github_client
        .repos(&owner, &repo)
        .releases()
        .list()
        .per_page(1)
        .send()
        .await
        .context("error getting last release")?;
    Ok(page.take_items().first().cloned())
}

/// Check if the last release body matches any of the regular expressions
/// provided.
pub(crate) async fn last_release_body_matches(
    github_client: &Octocrab,
    repo_url: &str,
    re: &RegexSet,
) -> Result<bool> {
    if let Some(last_release) = last_release(github_client, repo_url).await? {
        if let Some(body) = last_release.body {
            return Ok(re.is_match(&body));
        }
    }
    Ok(false)
}

/// Get commit check runs.
#[cached(
    result = true,
    sync_writes = true,
    key = "String",
    convert = r#"{ format!("{}{}{}", owner, repo, sha) }"#
)]
async fn get_commit_check_runs(
    github_client: &Octocrab,
    owner: &str,
    repo: &str,
    sha: &str,
) -> Result<Vec<GHCheckRun>> {
    let url = format!("repos/{}/{}/commits/{}/check-runs", &owner, &repo, &sha);
    let response: GHCheckRunsResponse = github_client
        .get(url, None::<&()>)
        .await
        .context("error getting check runs")?;
    Ok(response.check_runs)
}

/// Get commit check suites.
#[cached(
    result = true,
    sync_writes = true,
    key = "String",
    convert = r#"{ format!("{}{}{}", owner, repo, sha) }"#
)]
async fn get_commit_check_suites(
    github_client: &Octocrab,
    owner: &str,
    repo: &str,
    sha: &str,
) -> Result<Vec<GHCheckSuite>> {
    let url = format!("repos/{}/{}/commits/{}/check-suites", &owner, &repo, &sha);
    let response: GHCheckSuitesResponse = github_client
        .get(url, None::<&()>)
        .await
        .context("error getting check suites")?;
    Ok(response.check_suites)
}

/// Get commit statuses.
#[cached(
    result = true,
    sync_writes = true,
    key = "String",
    convert = r#"{ format!("{}{}{}", owner, repo, sha) }"#
)]
async fn get_commit_statuses(
    github_client: &Octocrab,
    owner: &str,
    repo: &str,
    sha: &str,
) -> Result<Vec<Status>> {
    let page = github_client
        .repos(owner, repo)
        .list_statuses(sha.to_string())
        .send()
        .await?;
    github_client
        .all_pages::<Status>(page)
        .await
        .context("error getting commit statuses")
}

/// Get last closed PR in repository.
#[cached(
    result = true,
    sync_writes = true,
    key = "String",
    convert = r#"{ format!("{}{}", owner, repo) }"#
)]
async fn get_last_closed_pr(
    github_client: &Octocrab,
    owner: &str,
    repo: &str,
) -> Result<Option<PullRequest>> {
    let mut page = github_client
        .pulls(owner, repo)
        .list()
        .state(State::Closed)
        .per_page(1)
        .send()
        .await
        .context("error getting last closed PR")?;
    Ok(page.take_items().first().map(|pr| pr.to_owned()))
}

/// Extract the owner and repository from the repository url provided.
fn get_owner_and_repo(repo_url: &str) -> Result<(String, String)> {
    let c = GITHUB_REPO_URL
        .captures(repo_url)
        .ok_or_else(|| format_err!("invalid repository url"))?;
    Ok((c["org"].to_string(), c["repo"].to_string()))
}

#[derive(Deserialize)]
pub struct GHCheckSuitesResponse {
    pub check_suites: Vec<GHCheckSuite>,
}

#[derive(Clone, Deserialize)]
pub struct GHCheckSuite {
    pub app: GHApp,
}

#[derive(Clone, Deserialize)]
pub struct GHApp {
    pub name: String,
}

#[derive(Deserialize)]
pub struct GHCheckRunsResponse {
    pub check_runs: Vec<GHCheckRun>,
}

#[derive(Clone, Deserialize)]
pub struct GHCheckRun {
    pub name: String,
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
