#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::doc_markdown)]

use std::{net::SocketAddr, path::PathBuf, sync::Arc, time::Duration};

use anyhow::{Context, Result, format_err};
use clap::Parser;
use clomonitor_core::tools::{LocalTool, Tool};
use config::{Config, File};
use tokio::{net::TcpListener, signal};
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

use crate::state::{RunnerConfig, State, ToolEntry};

mod handlers;
mod router;
mod state;

/// Command line arguments.
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
        .set_default("runner.addr", "0.0.0.0:8080")?
        .set_default("runner.maxConcurrentRuns", 10)?
        .set_default("runner.maxQueue", 20)?
        .set_default("runner.minBudgetMs", 60_000)?
        .add_source(File::from(args.config))
        .build()
        .context("error setting up configuration")?;

    // Setup logging
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "clomonitor_runner=debug,tower_http=debug");
        }
    }
    let s = tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env());
    match cfg.get_string("log.format").as_deref() {
        Ok("json") => s.json().init(),
        _ => s.init(),
    }

    // Setup enabled tools (binaries must be available and supported)
    debug!("setting up tools");
    let tool_ids: Vec<String> = cfg
        .get("runner.tools")
        .context("error setting up configuration (runner.tools)")?;
    if tool_ids.is_empty() {
        return Err(format_err!("no tools enabled (runner.tools)"));
    }
    let mut entries = Vec::new();
    for id in &tool_ids {
        let tool =
            Tool::from_id(id).ok_or_else(|| format_err!("unknown tool {id:?} in runner.tools"))?;
        let local = LocalTool::locate(tool)
            .await
            .with_context(|| format!("error setting up {tool}"))?;
        info!(%tool, version = %local.version, bin = %local.bin.display(), "tool ready");
        entries.push(ToolEntry {
            deadline: tool.deadline(),
            local,
        });
    }

    // Setup and launch HTTP server
    debug!("setting up runner");
    let state = Arc::new(State::new(
        entries,
        RunnerConfig {
            max_concurrent_runs: cfg.get::<usize>("runner.maxConcurrentRuns")?,
            max_queue: cfg.get::<usize>("runner.maxQueue")?,
            min_budget: Duration::from_millis(cfg.get::<u64>("runner.minBudgetMs")?),
        },
    )?);
    let router = router::setup(state.clone());
    let addr: SocketAddr = cfg.get_string("runner.addr")?.parse()?;
    let listener = TcpListener::bind(addr).await?;
    info!("runner started");
    info!(%addr, tools = ?tool_ids, "listening");
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal(state))
        .await?;

    info!("runner stopped");
    Ok(())
}

/// Wait for a shutdown signal (ctrl+c or, on unix, SIGTERM) and log the work
/// left to drain once it arrives.
async fn shutdown_signal(state: Arc<State>) {
    // Setup signal handlers
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("ctrl+c signal handler to be installed");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("terminate signal handler to be installed")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // Wait for any of the signals
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }

    // Make the drain observable: the server stops accepting connections and
    // waits for the requests below to complete
    info!(
        running = state.running(),
        queued = state.waiting(),
        "shutdown signal received, draining in-flight runs"
    );
}
