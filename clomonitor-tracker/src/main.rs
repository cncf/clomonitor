use anyhow::{format_err, Error};
use clap::Parser;
use config::{Config, File};
use deadpool_postgres::{Config as DbConfig, Runtime};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use std::path::PathBuf;
use which::which;

mod repository;
mod tracker;

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

    // Run tracker
    tracker::run(cfg, db_pool).await?;

    Ok(())
}
