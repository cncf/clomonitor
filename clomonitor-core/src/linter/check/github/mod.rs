use super::patterns::GITHUB_REPO_URL;
use anyhow::{format_err, Context, Result};
use graphql_client::{GraphQLQuery, Response};
use regex::RegexSet;
use std::path::Path;

/// Github GraphQL API URL.
const GITHUB_GRAPHQL_API: &str = "https://api.github.com/graphql";

/// Type alias for GraphQL URI scalar type.
#[allow(clippy::upper_case_acronyms)]
type URI = String;

/// Type alias for GraphQL DateTime scalar type.
type DateTime = String;

/// Represents the GraphQL Github API metadata query.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/linter/check/github/github_schema.graphql",
    query_path = "src/linter/check/github/md.graphql",
    response_derives = "Debug"
)]
pub struct Md;

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

/// Returns the default branch to use from the default branch reference
/// provided.
pub(crate) fn default_branch(r: &Option<md::MdRepositoryDefaultBranchRef>) -> String {
    match r {
        Some(r) => r.name.clone(),
        None => "master".to_string(),
    }
}

/// Get repository's metadata from the Github GraphQL API.
pub(crate) async fn get_metadata(
    http_client: &reqwest::Client,
    repo_url: &str,
) -> Result<md::MdRepository> {
    let (owner, repo) = get_owner_and_repo(repo_url)?;

    // Do request to GraphQL API
    let vars = md::Variables { owner, repo };
    let req_body = &Md::build_query(vars);
    let resp = http_client
        .post(GITHUB_GRAPHQL_API)
        .json(req_body)
        .send()
        .await
        .context("error requesting repository medatata from github graphql api")?;

    // Parse response body and extract repository metadata
    let resp_body: Response<md::ResponseData> = resp
        .json()
        .await
        .context("error deserializing repository medatata response from github graphql api")?;
    let repo = resp_body
        .data
        .ok_or_else(|| format_err!("data not found"))?
        .repository
        .ok_or_else(|| format_err!("repository not found"))?;

    Ok(repo)
}

/// Check if the repo has a check in the latest PR that matches any of the
/// regular expressions provided.
pub(crate) fn has_check(gh_md: &md::MdRepository, re: &RegexSet) -> Result<bool> {
    // Get latest PR head commit from metadata
    let latest_pr_head_commit = gh_md
        .pull_requests
        .nodes
        .as_ref()
        .and_then(|prs| prs.iter().next())
        .and_then(|pr_opt| pr_opt.as_ref())
        .and_then(|pr| pr.commits.nodes.as_ref())
        .and_then(|commits| commits.iter().next())
        .and_then(|commit_opt| commit_opt.as_ref())
        .map(|commit| &commit.commit);

    // Get check suites from commit obtained above
    let check_suites = latest_pr_head_commit
        .and_then(|commit| commit.check_suites.as_ref())
        .and_then(|check_suites| check_suites.nodes.as_ref());

    // Search in check suites apps name
    if let Some(true) = check_suites.map(|check_suites| {
        check_suites.iter().flatten().any(|s| {
            if let Some(app) = s.app.as_ref() {
                return re.is_match(&app.name);
            }
            false
        })
    }) {
        return Ok(true);
    }

    // Search in check runs name
    if let Some(true) = check_suites.map(|check_suites| {
        check_suites.iter().flatten().any(|check_suite| {
            if let Some(match_found) = check_suite
                .check_runs
                .as_ref()
                .and_then(|check_runs| check_runs.nodes.as_ref())
                .map(|check_runs| {
                    check_runs
                        .iter()
                        .flatten()
                        .any(|check_run| re.is_match(&check_run.name))
                })
            {
                return match_found;
            }
            false
        })
    }) {
        return Ok(true);
    }

    // Search in commit statuses context
    if let Some(true) = latest_pr_head_commit
        .and_then(|commit| commit.status.as_ref())
        .map(|status| status.contexts.iter().any(|c| re.is_match(&c.context)))
    {
        return Ok(true);
    }

    Ok(false)
}

/// Check if the given default community health file is available in the
/// .github repository, returning the url to the file when found.
pub(crate) async fn has_community_health_file(
    http_client: &reqwest::Client,
    file: &str,
    gh_md: &md::MdRepository,
) -> Result<Option<String>> {
    // Check if the file is in the repo
    let file_raw_url = format!(
        "https://raw.githubusercontent.com/{}/.github/HEAD/{}",
        &gh_md.owner.login, file
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
            let url = build_url(Path::new(file), &gh_md.owner.login, ".github", "HEAD");
            Ok(Some(url))
        }
        _ => Ok(None),
    }
}

/// Check if the latest release description matches any of the regular
/// expressions provided.
pub(crate) fn latest_release_description_matches(
    gh_md: &md::MdRepository,
    re: &RegexSet,
) -> Result<bool> {
    if let Some(description) = gh_md
        .latest_release
        .as_ref()
        .and_then(|r| r.description.as_ref())
    {
        return Ok(re.is_match(description));
    }
    Ok(false)
}

/// Extract the owner and repository from the repository url provided.
fn get_owner_and_repo(repo_url: &str) -> Result<(String, String)> {
    let c = GITHUB_REPO_URL
        .captures(repo_url)
        .ok_or_else(|| format_err!("invalid repository url"))?;
    Ok((c["org"].to_string(), c["repo"].to_string()))
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
