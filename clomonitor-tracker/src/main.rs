#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::doc_markdown)]

use std::{path::PathBuf, sync::Arc};

use anyhow::{Context, Result};
use clap::Parser;
use clomonitor_core::linter::{CoreLinter, ToolMode, ToolsConfig};
use config::{Config, File};
use deadpool_postgres::{Config as DbConfig, Runtime};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use tracing::{debug, warn};
use tracing_subscriber::EnvFilter;

use crate::{db::PgDB, git::GitCLI};

mod db;
mod git;
mod tracker;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Config file path
    #[clap(short, long)]
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
        unsafe {
            std::env::set_var("RUST_LOG", "clomonitor_tracker=debug");
        }
    }
    let s = tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env());
    match cfg.get_string("log.format").as_deref() {
        Ok("json") => s.json().init(),
        _ => s.init(),
    }

    // Setup external tools execution modes (the tracker does not run the
    // tools' binaries itself, it delegates to a runner service when one is
    // configured)
    let tools = ToolsConfig {
        afdocs: tool_mode(&cfg, "runner.afdocsUrl", ToolMode::Disabled)?,
        scorecard: tool_mode(&cfg, "runner.scorecardUrl", ToolMode::Disabled)?,
    };
    if tools.afdocs == ToolMode::Disabled {
        warn!("runner.afdocsUrl not set: agent readiness checks will be reported as failed");
    }
    if tools.scorecard == ToolMode::Disabled {
        warn!("runner.scorecardUrl not set: scorecard backed checks will be reported as failed");
    }

    // Setup database
    debug!("setting up database");
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());
    let db_cfg: DbConfig = cfg.get("db")?;
    let pool = db_cfg.create_pool(Some(Runtime::Tokio1), connector)?;
    let db = Arc::new(PgDB::new(pool));

    // Run tracker
    let git = Arc::new(GitCLI::new()?);
    let linter = Arc::new(CoreLinter::new());
    tracker::run(&cfg, db, git, linter, tools).await
}

/// Get the execution mode of an external tool from the runner url set in the
/// configuration key provided, using the default mode when it is not set.
fn tool_mode(cfg: &Config, key: &str, default: ToolMode) -> Result<ToolMode> {
    match cfg.get_string(key) {
        Ok(url) => ToolMode::runner(&url).context("error setting up configuration"),
        Err(config::ConfigError::NotFound(_)) => Ok(default),
        Err(err) => Err(err).context("error setting up configuration"),
    }
}
