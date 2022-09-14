use crate::{db::DynDB, git::DynGit};
use anyhow::{format_err, Result};
use clomonitor_core::linter::{
    lint, setup_github_http_client, CheckSet, GithubOptions, LintOptions, LintServices,
};
use config::Config;
use deadpool::unmanaged::{Object, Pool};
use futures::{
    future,
    stream::{FuturesUnordered, StreamExt},
};
use serde_json::Value;
use std::time::{Duration, Instant};
use tempfile::Builder;
use time::{self, OffsetDateTime};
use tokio::time::timeout;
use tracing::{debug, info, instrument, warn};
use uuid::Uuid;

/// Maximum time that can take tracking a single repository.
const REPOSITORY_TRACK_TIMEOUT: u64 = 600;

/// A project's repository.
#[derive(Debug, Clone)]
pub(crate) struct Repository {
    pub repository_id: Uuid,
    pub url: String,
    pub check_sets: Vec<CheckSet>,
    pub digest: Option<String>,
    pub updated_at: OffsetDateTime,
}

/// Track all repositories registered in the database.
pub(crate) async fn run(cfg: &Config, db: DynDB, git: DynGit) -> Result<()> {
    info!("tracker started");

    // Setup GitHub tokens pool
    let gh_tokens = cfg.get::<Vec<String>>("creds.githubTokens")?;
    if gh_tokens.is_empty() {
        return Err(format_err!(
            "GitHub tokens not found in config file (creds.githubTokens)"
        ));
    }
    let gh_tokens_pool = Pool::from(gh_tokens.clone());

    // Get repositories to process
    debug!("getting repositories");
    let repositories = db.repositories().await?;
    if repositories.is_empty() {
        info!("no repositories found");
        info!("tracker finished");
        return Ok(());
    }

    // Track repositories
    info!("tracking repositories");
    let mut futs = FuturesUnordered::new();
    for repository in repositories {
        // Track next repository
        let db = db.clone();
        let git = git.clone();
        let github_token = gh_tokens_pool.get().await?;
        let repository_id = repository.repository_id;
        futs.push(tokio::spawn(async move {
            if timeout(
                Duration::from_secs(REPOSITORY_TRACK_TIMEOUT),
                track_repository(db, git, github_token, repository),
            )
            .await
            .is_err()
            {
                warn!("timeout tracking repository {}", repository_id);
            }
        }));

        // Wait if needed to honor the concurrency limits
        if futs.len() == cfg.get::<usize>("tracker.concurrency")? {
            futs.next().await;
        }
    }
    future::join_all(futs).await;

    // Check Github API rate limit status for each token
    for (i, token) in gh_tokens.into_iter().enumerate() {
        let gh_client = setup_github_http_client(&GithubOptions {
            token,
            ..GithubOptions::default()
        })?;
        let response: Value = gh_client
            .get("https://api.github.com/rate_limit")
            .send()
            .await?
            .json()
            .await?;
        debug!(
            "token [{}] github rate limit info: [rate: {}] [graphql: {}]",
            i, response["rate"], response["resources"]["graphql"]
        );
    }

    info!("tracker finished");
    Ok(())
}

/// Track repository if it has changed since the last time it was tracked.
/// This involves cloning the repository, linting it and storing the results.
#[instrument(fields(repository_id = %repository.repository_id), skip_all, err)]
pub(crate) async fn track_repository(
    db: DynDB,
    git: DynGit,
    github_token: Object<String>,
    repository: Repository,
) -> Result<()> {
    let start = Instant::now();

    // Process only if the repository has changed since the last time it
    // was tracked or if it hasn't been tracked in more than 1 day
    let remote_digest = git.remote_digest(&repository.url).await?;
    if let Some(digest) = &repository.digest {
        let one_day_ago = OffsetDateTime::now_utc() - time::Duration::days(1);
        if &remote_digest == digest && repository.updated_at > one_day_ago {
            return Ok(());
        }
    }

    debug!("started");

    // Clone repository
    let tmp_dir = Builder::new().prefix("clomonitor").tempdir()?;
    git.clone_repository(&repository.url, tmp_dir.path())
        .await?;

    // Lint repository
    let mut errors: Option<String> = None;
    let opts = LintOptions {
        root: tmp_dir.into_path(),
        url: repository.url.clone(),
        check_sets: repository.check_sets.clone(),
        github_token: github_token.to_owned(),
    };
    let svc = LintServices::new(&GithubOptions {
        token: github_token.to_owned(),
        ..GithubOptions::default()
    })?;
    let report = match lint(&opts, &svc).await {
        Ok(report) => Some(report),
        Err(err) => {
            warn!("error linting repository: {:#}", err);
            errors = Some(format!("error linting repository: {:#}", err));
            None
        }
    };

    // Store tracking results in database
    db.store_results(
        &repository.repository_id,
        report.as_ref(),
        errors.as_ref(),
        &remote_digest,
    )
    .await?;

    debug!("completed in {}s", start.elapsed().as_secs());
    Ok(())
}
