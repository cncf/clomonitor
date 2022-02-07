use crate::handlers::*;
use anyhow::Error;
use axum::{
    http::StatusCode,
    routing::{get, get_service, post},
    AddExtensionLayer, Router,
};
use config::Config;
use deadpool_postgres::Pool;
use std::path::Path;
use tower::ServiceBuilder;
use tower_http::{
    auth::RequireAuthorizationLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

/// Setup API server router.
pub(crate) fn setup(cfg: &Config, db_pool: Pool) -> Result<Router, Error> {
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
    let mut router = Router::new()
        .route("/api/projects/search", post(search_projects))
        .route("/api/projects/:project_id", get(get_project))
        .route(
            "/",
            get_service(ServeFile::new(&index_path)).handle_error(error_handler),
        )
        .nest(
            "/static",
            get_service(ServeDir::new(static_path)).handle_error(error_handler),
        )
        .fallback(get_service(ServeFile::new(&index_path)).handle_error(error_handler))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(AddExtensionLayer::new(db_pool)),
        );

    // Setup basic auth
    if cfg.get_bool("apiserver.basicAuth.enabled")? {
        let username = cfg.get_str("apiserver.basicAuth.username")?;
        let password = cfg.get_str("apiserver.basicAuth.password")?;
        router = router.layer(RequireAuthorizationLayer::basic(&username, &password));
    }

    Ok(router)
}
