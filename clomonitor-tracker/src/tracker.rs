use crate::repository;
use anyhow::Result;
use config::Config;
use deadpool_postgres::Pool;
use futures::{
    future,
    stream::{FuturesUnordered, StreamExt},
};
use serde_json::Value;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{debug, error, info};

/// Maximum time that can take tracking a single repository.
const REPOSITORY_TRACK_TIMEOUT: u64 = 300;

/// Track all repositories registered in the database.
pub(crate) async fn run(cfg: Config, db_pool: Pool) -> Result<()> {
    info!("tracker started");

    // Initialize Github API client
    let mut builder = octocrab::Octocrab::builder();
    if let Ok(token) = cfg.get_str("creds.githubToken") {
        builder = builder.personal_token(token);
    }
    octocrab::initialise(builder)?;

    // Get repositories to process
    let repositories = repository::get_all(db_pool.get().await?).await?;
    if repositories.is_empty() {
        info!("no repositories found");
        info!("tracker finished");
        return Ok(());
    }

    // Track repositories
    info!("tracking repositories");
    let mut futs = FuturesUnordered::new();
    for repository in repositories {
        let db = db_pool.get().await?;
        futs.push(tokio::spawn(async move {
            if let Err(err) = timeout(
                Duration::from_secs(REPOSITORY_TRACK_TIMEOUT),
                repository.track(db),
            )
            .await
            {
                error!("error tracking repository {}: {err}", repository.id());
            }
        }));
        if futs.len() == cfg.get::<usize>("tracker.concurrency").unwrap() {
            futs.next().await;
        }
    }
    future::join_all(futs).await;

    // Check Github API rate limit status
    let response: Value = octocrab::instance().get("rate_limit", None::<&()>).await?;
    debug!("github rate limit info: {}", response["rate"]);

    info!("tracker finished");
    Ok(())
}
