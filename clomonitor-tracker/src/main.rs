mod tracker;

use crate::tracker::*;
use anyhow::{format_err, Error};
use clap::Parser;
use config::{Config, File};
use deadpool_postgres::{Config as DbConfig, Runtime};
use futures::future;
use futures::stream::{FuturesUnordered, StreamExt};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use std::path::PathBuf;
use tracing::{error, info};
use which::which;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Config file path
    #[clap(short, long, parse(from_os_str))]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    // Setup logging
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "clomonitor_tracker=debug")
    }
    tracing_subscriber::fmt::init();

    // Check if required external tools are available
    if which("git").is_err() {
        return Err(format_err!("git not found in PATH"));
    }

    info!("tracker started");

    // Setup configuration
    let mut cfg = Config::new();
    cfg.set_default("db.dbname", "clomonitor")?;
    cfg.set_default("tracker.concurrency", 10)?;
    cfg.merge(File::from(args.config))?;

    // Setup database
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());
    let db_cfg: DbConfig = cfg.get("db").unwrap();
    let db_pool = db_cfg.create_pool(Some(Runtime::Tokio1), connector)?;

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
