use anyhow::{format_err, Context, Result};
use clap::Parser;
use config::{Config, File};
use deadpool_postgres::{Config as DbConfig, Runtime};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use std::path::PathBuf;
use tracing::debug;
use tracing_subscriber::EnvFilter;
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
async fn main() -> Result<()> {
    let args = Args::parse();

    // Setup configuration
    let cfg = Config::builder()
        .set_default("tracker.concurrency", 10)?
        .add_source(File::from(args.config))
        .build()
        .context("error setting up configuration")?;

    // Setup logging
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "clomonitor_tracker=debug")
    }
    let s = tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env());
    match cfg.get_string("log.format").as_deref() {
        Ok("json") => s.json().init(),
        _ => s.init(),
    };

    // Check if required external tools are available
    debug!("checking required external tools");
    if which("git").is_err() {
        return Err(format_err!("git not found in PATH"));
    }

    // Setup database
    debug!("setting up database");
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());
    let db_cfg: DbConfig = cfg.get("db")?;
    let db_pool = db_cfg.create_pool(Some(Runtime::Tokio1), connector)?;

    // Run tracker
    tracker::run(&cfg, &db_pool).await?;

    Ok(())
}
