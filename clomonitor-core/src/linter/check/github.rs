use anyhow::{format_err, Error};
use chrono::{Duration, Utc};
use lazy_static::lazy_static;
use octocrab::models::{repos::Release, Repository};
use regex::{Regex, RegexSet};

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
    if let Some(last_release) = last_release(repo_url).await? {
        if let Some(created_at) = last_release.created_at {
            return Ok(created_at > Utc::now() - Duration::days(365));
        }
    }
    Ok(false)
}

/// Check if the last release body matches any of the regular expressions
/// provided.
pub(crate) async fn last_release_body_matches<R>(repo_url: &str, regexps: R) -> Result<bool, Error>
where
    R: IntoIterator,
    R::Item: AsRef<str>,
{
    if let Some(last_release) = last_release(repo_url).await? {
        if let Some(body) = last_release.body {
            let re = RegexSet::new(regexps)?;
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
