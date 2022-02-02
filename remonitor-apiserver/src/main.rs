mod handlers;
mod router;

use anyhow::Error;
use clap::Parser;
use config::{Config, File};
use deadpool_postgres::{Config as DbConfig, Runtime};
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::signal;
use tokio_postgres::NoTls;
use tracing::info;

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
        std::env::set_var("RUST_LOG", "remonitor_apiserver=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();
    info!("apiserver started");

    // Setup configuration
    let mut cfg = Config::new();
    cfg.set_default("db.dbname", "remonitor")?;
    cfg.set_default("apiserver.addr", "127.0.0.1:8000")?;
    cfg.set_default("apiserver.basicAuth.enabled", false)?;
    cfg.merge(File::from(args.config))?;

    // Setup database
    let db_cfg: DbConfig = cfg.get("db").unwrap();
    let db_pool = db_cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;

    // Setup and launch HTTP server
    let router = router::setup(&cfg, db_pool)?;
    let addr: SocketAddr = cfg.get_str("apiserver.addr")?.parse()?;
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    info!("apiserver stopped");
    Ok(())
}

async fn shutdown_signal() {
    // Setup signal handlers
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install ctrl+c signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install terminate signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // Wait for any of the signals
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    info!("apiserver stopping...");
}
