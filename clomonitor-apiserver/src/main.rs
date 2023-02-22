use crate::{db::PgDB, views::ViewsTrackerDB};
use anyhow::{Context, Result};
use clap::Parser;
use config::{Config, File};
use deadpool_postgres::{Config as DbConfig, Runtime};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::{signal, sync::RwLock};
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

mod db;
mod filters;
mod handlers;
mod middleware;
mod router;
mod views;

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
        .set_default("apiserver.addr", "127.0.0.1:8000")?
        .set_default("apiserver.baseURL", "http://127.0.0.1:8000")?
        .add_source(File::from(args.config))
        .build()
        .context("error setting up configuration")?;
    let cfg = Arc::new(cfg);

    // Setup logging
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "clomonitor_apiserver=debug,tower_http=debug")
    }
    let s = tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env());
    match cfg.get_string("log.format").as_deref() {
        Ok("json") => s.json().init(),
        _ => s.init(),
    };

    // Setup database
    debug!("setting up database");
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());
    let db_cfg: DbConfig = cfg.get("db")?;
    let pool = db_cfg.create_pool(Some(Runtime::Tokio1), connector)?;
    let db = Arc::new(PgDB::new(pool));

    // Setup views tracker
    let vt = Arc::new(RwLock::new(ViewsTrackerDB::new(db.clone())));

    // Setup and launch Prometheus exporter
    debug!("setting up prometheus exporter");
    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("clomonitor_apiserver_http_request_duration".to_string()),
            &[
                0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
            ],
        )?
        .install()?;

    // Setup and launch API HTTP server
    debug!("setting up apiserver");
    let router = router::setup(cfg.clone(), db, vt.clone())?;
    let addr: SocketAddr = cfg.get_string("apiserver.addr")?.parse()?;
    info!("apiserver started");
    info!(%addr, "listening");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    // Ask views tracker to stop and wait for it to finish
    vt.write().await.stop().await;

    info!("apiserver stopped");
    Ok(())
}

async fn shutdown_signal() {
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
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    info!("apiserver stopping");
}
