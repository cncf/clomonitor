use crate::{db::DynDB, handlers::*};
use anyhow::Result;
use axum::{
    extract::Extension,
    http::StatusCode,
    routing::{get, get_service, post},
    Router,
};
use config::Config;
use std::path::Path;
use tower::ServiceBuilder;
use tower_http::{
    auth::RequireAuthorizationLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

/// Setup API server router.
pub(crate) fn setup(cfg: &Config, db: DynDB) -> Result<Router> {
    // Setup some paths
    let static_path = cfg.get_string("apiserver.staticPath")?;
    let index_path = Path::new(&static_path).join("index.html");
    let docs_path = Path::new(&static_path).join("docs");

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
        .route("/api/projects/:foundation/:org/:project", get(project))
        .route("/api/projects/:foundation/:org/:project/badge", get(badge))
        .route(
            "/api/projects/:foundation/:org/:project/report-summary",
            get(report_summary_svg),
        )
        .route("/api/stats", get(stats))
        .route(
            "/",
            get_service(ServeFile::new(&index_path)).handle_error(error_handler),
        )
        .nest(
            "/docs",
            get_service(ServeDir::new(docs_path)).handle_error(error_handler),
        )
        .nest(
            "/static",
            get_service(ServeDir::new(static_path)).handle_error(error_handler),
        )
        .fallback(get_service(ServeFile::new(&index_path)).handle_error(error_handler))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(db)),
        );

    // Setup basic auth
    if cfg.get_bool("apiserver.basicAuth.enabled")? {
        let username = cfg.get_string("apiserver.basicAuth.username")?;
        let password = cfg.get_string("apiserver.basicAuth.password")?;
        router = router.layer(RequireAuthorizationLayer::basic(&username, &password));
    }

    Ok(router)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{MockDB, SearchProjectsInput};
    use axum::{
        body::Body,
        http::{
            header::{CACHE_CONTROL, CONTENT_TYPE},
            Request,
        },
    };
    use clomonitor_core::score::Score;
    use mime::APPLICATION_JSON;
    use mockall::predicate::*;
    use serde_json::json;
    use std::{fs, future, sync::Arc};
    use tower::ServiceExt;

    const TESTDATA_PATH: &str = "src/testdata";
    const FOUNDATION: &str = "cncf";
    const ORG: &str = "artifact-hub";
    const PROJECT: &str = "artifact-hub";

    #[tokio::test]
    async fn badge_found() {
        let mut db = MockDB::new();
        db.expect_project_rating()
            .with(eq(FOUNDATION), eq(ORG), eq(PROJECT))
            .times(1)
            .returning(|_: &str, _: &str, _: &str| {
                Box::pin(future::ready(Ok(Some("a".to_string()))))
            });

        let response = setup_test_router(db)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/api/projects/{FOUNDATION}/{ORG}/{PROJECT}/badge"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()[CONTENT_TYPE], APPLICATION_JSON.as_ref());
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            json!({
                "labelColor": "3F1D63",
                "namedLogo": "cncf",
                "logoColor": "BEB5C8",
                "logoWidth": 10,
                "label": "CLOMonitor Report",
                "message": "A",
                "color": "green",
                "schemaVersion": 1,
                "style": "flat"
            })
            .to_string()
        );
    }

    #[tokio::test]
    async fn badge_not_found() {
        let mut db = MockDB::new();
        db.expect_project_rating()
            .with(eq(FOUNDATION), eq(ORG), eq(PROJECT))
            .times(1)
            .returning(|_: &str, _: &str, _: &str| Box::pin(future::ready(Ok(None))));

        let response = setup_test_router(db)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/api/projects/{FOUNDATION}/{ORG}/{PROJECT}/badge"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn project_found() {
        let mut db = MockDB::new();
        db.expect_project()
            .with(eq(FOUNDATION), eq(ORG), eq(PROJECT))
            .times(1)
            .returning(|_, _, _| {
                Box::pin(future::ready(Ok(Some(
                    r#"{"project": "info"}"#.to_string(),
                ))))
            });

        let response = setup_test_router(db)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/api/projects/{FOUNDATION}/{ORG}/{PROJECT}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()[CONTENT_TYPE], APPLICATION_JSON.as_ref());
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            r#"{"project": "info"}"#.to_string(),
        );
    }

    #[tokio::test]
    async fn project_not_found() {
        let mut db = MockDB::new();
        db.expect_project()
            .with(eq(FOUNDATION), eq(ORG), eq(PROJECT))
            .times(1)
            .returning(|_: &str, _: &str, _: &str| Box::pin(future::ready(Ok(None))));

        let response = setup_test_router(db)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/api/projects/{FOUNDATION}/{ORG}/{PROJECT}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn report_summary_found() {
        let mut db = MockDB::new();
        db.expect_project_score()
            .with(eq(FOUNDATION), eq(ORG), eq(PROJECT))
            .times(1)
            .returning(|_: &str, _: &str, _: &str| {
                let score = Score {
                    global: 80.0,
                    documentation: Some(80.0),
                    license: Some(50.0),
                    ..Score::default()
                };
                Box::pin(future::ready(Ok(Some(score))))
            });

        let response = setup_test_router(db)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!(
                        "/api/projects/{FOUNDATION}/{ORG}/{PROJECT}/report-summary"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()[CACHE_CONTROL], "max-age=3600");
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let golden_path = "src/testdata/report-summary.golden";
        // fs::write(golden_path, &body).unwrap(); // Uncomment to update golden file
        let golden = fs::read_to_string(golden_path).unwrap();
        assert_eq!(body, golden);
    }

    #[tokio::test]
    async fn report_summary_not_found() {
        let mut db = MockDB::new();
        db.expect_project_score()
            .with(eq(FOUNDATION), eq(ORG), eq(PROJECT))
            .times(1)
            .returning(|_: &str, _: &str, _: &str| Box::pin(future::ready(Ok(None))));

        let response = setup_test_router(db)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!(
                        "/api/projects/{FOUNDATION}/{ORG}/{PROJECT}/report-summary"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn search_projects() {
        let mut db = MockDB::new();
        db.expect_search_projects()
            .with(eq(SearchProjectsInput {
                limit: Some(10),
                offset: Some(1),
                text: Some("hub".to_string()),
                ..SearchProjectsInput::default()
            }))
            .times(1)
            .returning(|_| {
                Box::pin(future::ready(Ok((
                    1,
                    r#"[{"project": "info"}]"#.to_string(),
                ))))
            });

        let response = setup_test_router(db)
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/projects/search")
                    .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        json!({"limit": 10, "offset": 1, "text": "hub"}).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()[CONTENT_TYPE], APPLICATION_JSON.as_ref());
        assert_eq!(response.headers()[PAGINATION_TOTAL_COUNT], "1");
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            r#"[{"project": "info"}]"#.to_string(),
        );
    }

    #[tokio::test]
    async fn stats() {
        let mut db = MockDB::new();
        db.expect_stats()
            .with(eq(Some(FOUNDATION.to_string())))
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(r#"{"some": "stats"}"#.to_string()))));

        let response = setup_test_router(db)
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/api/stats?foundation={FOUNDATION}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()[CONTENT_TYPE], APPLICATION_JSON.as_ref());
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            r#"{"some": "stats"}"#.to_string(),
        );
    }

    #[tokio::test]
    async fn index() {
        let response = setup_test_router(MockDB::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            fs::read_to_string(Path::new(TESTDATA_PATH).join("index.html")).unwrap()
        );
    }

    #[tokio::test]
    async fn fallback_to_index() {
        let response = setup_test_router(MockDB::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/not-found")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            fs::read_to_string(Path::new(TESTDATA_PATH).join("index.html")).unwrap()
        );
    }

    #[tokio::test]
    async fn static_content() {
        let response = setup_test_router(MockDB::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/static/lib.js")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            fs::read_to_string(Path::new(TESTDATA_PATH).join("lib.js")).unwrap()
        );
    }

    fn setup_test_router(db: MockDB) -> Router {
        let cfg = setup_test_config();
        setup(&cfg, Arc::new(db)).unwrap()
    }

    fn setup_test_config() -> Config {
        Config::builder()
            .set_default("apiserver.staticPath", TESTDATA_PATH)
            .unwrap()
            .set_default("apiserver.basicAuth.enabled", false)
            .unwrap()
            .build()
            .unwrap()
    }
}
