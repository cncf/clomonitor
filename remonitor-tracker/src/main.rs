mod score;
mod tracker;

use crate::tracker::*;
use anyhow::Error;
use config::{Config, File};
use deadpool_postgres::{Config as DbConfig, Runtime};
use futures::future;
use futures::stream::{FuturesUnordered, StreamExt};
use tokio_postgres::NoTls;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Check if required external tools are available
    // TODO

    // Setup logging
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "remonitor_tracker=debug")
    }
    tracing_subscriber::fmt::init();
    info!("tracker started");

    // Setup configuration
    let cfg_dir = dirs::config_dir()
        .expect("config dir not found")
        .join("remonitor");
    let mut cfg = Config::new();
    cfg.set_default("db.dbname", "remonitor")?;
    cfg.set_default("tracker.concurrency", 10)?;
    cfg.merge(File::from(cfg_dir.join("tracker")))?;

    // Setup database
    let db_cfg: DbConfig = cfg.get("db").unwrap();
    let db_pool = db_cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;

    // Get repositories to process
    let repositories = get_repositories(db_pool.get().await?).await?;
    if repositories.is_empty() {
        info!("no repositories found");
        info!("tracker finished");
        return Ok(());
    }

    // Process repositories
    info!("processing repositories");
    let mut futs = FuturesUnordered::new();
    for repo in repositories {
        let db = db_pool.get().await?;
        futs.push(tokio::spawn(async move {
            let repository_id = repo.id();
            if let Err(err) = process_repository(db, repo).await {
                error!("error processing repository {repository_id}: {err}");
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
