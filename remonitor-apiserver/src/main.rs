use anyhow::Error;
use axum::{
    extract,
    extract::Extension,
    http::{
        header::{HeaderMap, HeaderName, HeaderValue},
        StatusCode,
    },
    response,
    routing::{get_service, post},
    AddExtensionLayer, Router,
};
use config::{Config, File};
use deadpool_postgres::{Config as DbConfig, Pool, Runtime};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::SocketAddr;
use std::path::Path;
use tokio_postgres::types::Json;
use tokio_postgres::NoTls;
use tower::ServiceBuilder;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::info;

/// Header that indicates the number of items available for pagination purposes.
const PAGINATION_TOTAL_COUNT: &str = "pagination-total-count";

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
    let router = setup_router(&cfg, db_pool)?;
    let addr: SocketAddr = cfg.get_str("apiserver.addr")?.parse()?;
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

/// Setup API server router.
fn setup_router(cfg: &Config, db_pool: Pool) -> Result<Router, Error> {
    // Setup some paths
    let static_path = cfg.get_str("apiserver.staticPath")?;
    let index_path = Path::new(&static_path).join("index.html");

    // Setup error handler
    let error_handler = |err: std::io::Error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("internal error: {}", err),
        )
    };

    // Setup router
    let router = Router::new()
        .route("/api/projects/search", post(search_projects))
        .route(
            "/",
            get_service(ServeFile::new(index_path)).handle_error(error_handler),
        )
        .nest(
            "/static",
            get_service(ServeDir::new(static_path)).handle_error(error_handler),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(AddExtensionLayer::new(db_pool)),
        );

    Ok(router)
}

/// Handler that allows searching for projects.
async fn search_projects(
    Extension(db_pool): Extension<Pool>,
    input: extract::Json<SearchProjectInput>,
) -> Result<(HeaderMap, response::Json<Value>), (StatusCode, String)> {
    let extract::Json(input) = input;

    // Get connection from db pool
    let db = db_pool.get().await.map_err(internal_error)?;

    // Search projects in database
    let row = db
        .query_one("select * from search_projects($1::jsonb)", &[&Json(input)])
        .await
        .map_err(internal_error)?;
    let Json(projects): Json<Value> = row.get("projects");
    let total_count: i64 = row.get("total_count");

    // Prepare response headers
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static(PAGINATION_TOTAL_COUNT),
        HeaderValue::from_str(&total_count.to_string()).unwrap(),
    );

    Ok((headers, response::Json(projects)))
}

/// Helper for mapping any error into a `500 Internal Server Error` response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

/// Query input used when searching for projects.
#[derive(Debug, Serialize, Deserialize)]
struct SearchProjectInput {
    limit: Option<usize>,
    offset: Option<usize>,
    text: Option<String>,
    category: Option<Vec<usize>>,
    maturity: Option<Vec<usize>>,
    rating: Option<Vec<char>>,
}
