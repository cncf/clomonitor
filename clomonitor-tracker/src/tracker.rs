use crate::repository;
use anyhow::Result;
use clomonitor_core::linter::{setup_github_http_client, GithubOptions};
use config::Config;
use deadpool::unmanaged;
use deadpool_postgres::Pool;
use futures::{
    future,
    stream::{FuturesUnordered, StreamExt},
};
use serde_json::Value;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{debug, info, warn};

/// Maximum time that can take tracking a single repository.
const REPOSITORY_TRACK_TIMEOUT: u64 = 300;

/// Track all repositories registered in the database.
pub(crate) async fn run(cfg: &Config, db_pool: &Pool) -> Result<()> {
    info!("tracker started");

    // Setup GitHub tokens pool
    let gh_tokens = cfg.get::<Vec<String>>("creds.githubTokens")?;
    let gh_tokens_pool = unmanaged::Pool::from(gh_tokens.clone());

    // Get repositories to process
    let repositories = repository::get_all(&db_pool.get().await?).await?;
    if repositories.is_empty() {
        info!("no repositories found");
        info!("tracker finished");
        return Ok(());
    }

    // Track repositories
    info!("tracking repositories");
    let mut futs = FuturesUnordered::new();
    for repository in repositories {
        // Get db connection and GitHub token from the corresponding pool
        let mut db = db_pool.get().await?;
        let github_token = gh_tokens_pool.get().await?;

        // Track next repository
        futs.push(tokio::spawn(async move {
            if timeout(
                Duration::from_secs(REPOSITORY_TRACK_TIMEOUT),
                repository.track(&mut db, github_token.as_ref()),
            )
            .await
            .is_err()
            {
                warn!("timeout tracking repository {}", repository.repository_id);
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
        let client = setup_github_http_client(&GithubOptions {
            token,
            ..GithubOptions::default()
        })?;
        let response: Value = client
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
