use crate::repository;
use anyhow::Error;
use config::Config;
use deadpool_postgres::Pool;
use futures::{
    future,
    stream::{FuturesUnordered, StreamExt},
};
use tracing::{error, info};

/// Track all repositories registered in the database.
pub(crate) async fn run(cfg: Config, db_pool: Pool) -> Result<(), Error> {
    info!("tracker started");

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
        let github_token = cfg.get_str("creds.githubToken").ok();
        futs.push(tokio::spawn(async move {
            if let Err(err) = repository.track(db, github_token.as_deref()).await {
                error!("error tracking repository {}: {err}", repository.id());
            }
        }));
        if futs.len() == cfg.get::<usize>("tracker.concurrency").unwrap() {
            futs.next().await;
        }
    }
    future::join_all(futs).await;

    info!("tracker finished");
    Ok(())
}
