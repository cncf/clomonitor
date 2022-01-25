mod handlers;
mod router;

use anyhow::Error;
use config::{Config, File};
use deadpool_postgres::{Config as DbConfig, Runtime};
use std::net::SocketAddr;
use tokio::signal;
use tokio_postgres::NoTls;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Setup logging
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "remonitor_apiserver=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();
    info!("apiserver started");

    // Setup configuration
    let cfg_dir = dirs::config_dir()
        .expect("config dir not found")
        .join("remonitor");
    let mut cfg = Config::new();
    cfg.set_default("db.dbname", "remonitor")?;
    cfg.merge(File::from(cfg_dir.join("apiserver")))?;

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
            .expect("failed to install ctrl+c handler");
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
