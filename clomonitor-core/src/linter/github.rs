use anyhow::{format_err, Error};
use chrono::{Duration, Utc};
use lazy_static::lazy_static;
use octocrab::models::Repository;
use regex::Regex;

/// Get repository's metadata from the Github API.
pub(crate) async fn get_metadata(repo_url: &str) -> Result<Repository, Error> {
    let (owner, repo) = get_owner_and_repo(repo_url)?;
    let github = octocrab::instance();
    match github.repos(&owner, &repo).get().await {
        Ok(repo) => Ok(repo),
        Err(err) => Err(err.into()),
    }
}

/// Check if the repository has released a new version in the last year.
pub(crate) async fn has_recent_release(repo_url: &str) -> Result<bool, Error> {
    let (owner, repo) = get_owner_and_repo(repo_url)?;
    let github = octocrab::instance();
    let mut page = github
        .repos(&owner, &repo)
        .releases()
        .list()
        .per_page(1)
        .send()
        .await?;
    let releases = page.take_items();
    if let Some(last_release) = releases.first() {
        if let Some(created_at) = last_release.created_at {
            return Ok(created_at > Utc::now() - Duration::days(365));
        }
    }
    Ok(false)
}

/// Extract the owner and repository from the repository url provided.
fn get_owner_and_repo(repo_url: &str) -> Result<(String, String), Error> {
    lazy_static! {
        static ref GITHUB_RE: Regex =
            Regex::new("^https://github.com/(?P<org>[^/]+)/(?P<repo>[^/]+)/?$").unwrap();
    }
    let c = GITHUB_RE
        .captures(repo_url)
        .ok_or(format_err!("invalid repository url"))?;
    Ok((c["org"].to_string(), c["repo"].to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

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
