use crate::{db::DynDB, handlers::*, middleware::metrics_collector};
use anyhow::Result;
use axum::{
    extract::Extension,
    http::{header::CACHE_CONTROL, HeaderValue, StatusCode},
    middleware,
    routing::{get, get_service},
    Router,
};
use config::Config;
use std::{path::Path, sync::Arc};
use tera::Tera;
use tower::ServiceBuilder;
use tower_http::{
    auth::RequireAuthorizationLayer, services::ServeDir, set_header::SetResponseHeader,
    trace::TraceLayer,
};

/// Static files cache duration.
pub const STATIC_CACHE_MAX_AGE: usize = 365 * 24 * 60 * 60;

/// Documentation files cache duration.
pub const DOCS_CACHE_MAX_AGE: usize = 300;

/// Setup API server router.
pub(crate) fn setup(cfg: Arc<Config>, db: DynDB) -> Result<Router> {
    // Setup error handler
    let error_handler = |err: std::io::Error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("internal error: {}", err),
        )
    };

    // Setup some paths
    let static_path = cfg.get_string("apiserver.staticPath")?;
    let index_path = Path::new(&static_path).join("index.html");
    let docs_path = Path::new(&static_path).join("docs");

    // Setup templates
    let mut tmpl = Tera::default();
    tmpl.autoescape_on(vec![]);
    tmpl.add_template_file(index_path, Some("index.html"))?;
    let tmpl = Arc::new(tmpl);

    // Setup API routes
    let api_routes = Router::new()
        .route("/projects/search", get(search_projects))
        .route("/projects/:foundation/:org/:project", get(project))
        .route("/projects/:foundation/:org/:project/badge", get(badge))
        .route(
            "/projects/:foundation/:org/:project/report-summary",
            get(report_summary_svg),
        )
        .route("/stats", get(stats));

    // Setup router
    let mut router = Router::new()
        .route("/", get(index))
        .route("/projects/:foundation/:org/:project", get(index_project))
        .route(
            "/projects/:foundation/:org/:project/report-summary.png",
            get(report_summary_png),
        )
        .route("/data/repositories.csv", get(repositories_checks))
        .nest("/api", api_routes)
        .nest(
            "/docs",
            get_service(SetResponseHeader::overriding(
                ServeDir::new(docs_path),
                CACHE_CONTROL,
                HeaderValue::try_from(format!("max-age={}", DOCS_CACHE_MAX_AGE))?,
            ))
            .handle_error(error_handler),
        )
        .nest(
            "/static",
            get_service(SetResponseHeader::overriding(
                ServeDir::new(static_path),
                CACHE_CONTROL,
                HeaderValue::try_from(format!("max-age={}", STATIC_CACHE_MAX_AGE))?,
            ))
            .handle_error(error_handler),
        )
        .fallback(get(index))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(middleware::from_fn(metrics_collector))
                .layer(Extension(cfg.clone()))
                .layer(Extension(db))
                .layer(Extension(tmpl)),
        );

    // Setup basic auth
    if cfg.get_bool("apiserver.basicAuth.enabled").unwrap_or(false) {
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
    use mime::{APPLICATION_JSON, HTML};
    use mockall::predicate::*;
    use serde_json::json;
    use std::{fs, future, sync::Arc};
    use tera::Context;
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
        assert_eq!(
            response.headers()[CACHE_CONTROL],
            format!("max-age={}", DEFAULT_API_MAX_AGE)
        );
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
    async fn docs_files() {
        let response = setup_test_router(MockDB::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/docs/topics.html")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers()[CACHE_CONTROL],
            format!("max-age={}", DOCS_CACHE_MAX_AGE)
        );
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            fs::read_to_string(Path::new(TESTDATA_PATH).join("docs").join("topics.html")).unwrap()
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
            response.headers()[CACHE_CONTROL],
            format!("max-age={}", INDEX_CACHE_MAX_AGE)
        );
        assert_eq!(response.headers()[CONTENT_TYPE], HTML.as_ref());
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            render_index(
                INDEX_META_TITLE,
                INDEX_META_DESCRIPTION,
                "http://localhost:8000/static/media/clomonitor.png"
            )
        );
    }

    #[tokio::test]
    async fn index_fallback() {
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
            response.headers()[CACHE_CONTROL],
            format!("max-age={}", INDEX_CACHE_MAX_AGE)
        );
        assert_eq!(response.headers()[CONTENT_TYPE], HTML.as_ref());
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            render_index(
                INDEX_META_TITLE,
                INDEX_META_DESCRIPTION,
                "http://localhost:8000/static/media/clomonitor.png"
            )
        );
    }

    #[tokio::test]
    async fn index_project() {
        let response = setup_test_router(MockDB::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/projects/{FOUNDATION}/{ORG}/{PROJECT}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers()[CACHE_CONTROL],
            format!("max-age={}", INDEX_CACHE_MAX_AGE)
        );
        assert_eq!(response.headers()[CONTENT_TYPE], HTML.as_ref());
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            render_index(
                PROJECT,
                INDEX_META_DESCRIPTION_PROJECT,
                "http://localhost:8000/projects/cncf/artifact-hub/artifact-hub/report-summary.png"
            )
        );
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
        assert_eq!(
            response.headers()[CACHE_CONTROL],
            format!("max-age={}", DEFAULT_API_MAX_AGE)
        );
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
    async fn report_summary_png_not_found() {
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
                        "/projects/{FOUNDATION}/{ORG}/{PROJECT}/report-summary.png"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn report_summary_svg_found() {
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
        assert_eq!(
            response.headers()[CACHE_CONTROL],
            format!("max-age={}", DEFAULT_API_MAX_AGE)
        );
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let golden_path = "src/testdata/report-summary.golden.svg";
        // fs::write(golden_path, &body).unwrap(); // Uncomment to update golden file
        let golden = fs::read(golden_path).unwrap();
        assert_eq!(body, golden);
    }

    #[tokio::test]
    async fn report_summary_svg_not_found() {
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
                sort_by: Some("name".to_string()),
                sort_direction: Some("asc".to_string()),
                text: Some("hub".to_string()),
                foundation: Some(vec!["cncf".to_string()]),
                maturity: Some(vec!["graduated".to_string(), "incubating".to_string()]),
                rating: Some(vec!['a', 'b']),
                accepted_from: Some("20200101".to_string()),
                accepted_to: Some("20210101".to_string()),
                passing_check: Some(vec!["dco".to_string(), "readme".to_string()]),
                not_passing_check: Some(vec!["website".to_string()]),
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
                    .method("GET")
                    .uri(
                        "\
                        /api/projects/search?\
                            limit=10&\
                            offset=1&\
                            sort_by=name&\
                            sort_direction=asc&\
                            text=hub&\
                            foundation[0]=cncf&\
                            maturity[0]=graduated&\
                            maturity[1]=incubating&\
                            rating[0]=a&\
                            rating[1]=b&\
                            accepted_from=20200101&\
                            accepted_to=20210101&\
                            passing_check[0]=dco&\
                            passing_check[1]=readme&\
                            not_passing_check[0]=website\
                        ",
                    )
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers()[CACHE_CONTROL],
            format!("max-age={}", DEFAULT_API_MAX_AGE)
        );
        assert_eq!(response.headers()[CONTENT_TYPE], APPLICATION_JSON.as_ref());
        assert_eq!(response.headers()[PAGINATION_TOTAL_COUNT], "1");
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            r#"[{"project": "info"}]"#.to_string(),
        );
    }

    #[tokio::test]
    async fn static_files() {
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
            response.headers()[CACHE_CONTROL],
            format!("max-age={}", STATIC_CACHE_MAX_AGE)
        );
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            fs::read_to_string(Path::new(TESTDATA_PATH).join("lib.js")).unwrap()
        );
    }

    #[tokio::test]
    async fn stats() {
        let mut db = MockDB::new();
        db.expect_stats()
            .withf(|v| v.as_deref() == Some(&FOUNDATION.to_string()))
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
        assert_eq!(response.headers()[CACHE_CONTROL], "max-age=3600");
        assert_eq!(response.headers()[CONTENT_TYPE], APPLICATION_JSON.as_ref());
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            r#"{"some": "stats"}"#.to_string(),
        );
    }

    fn setup_test_router(db: MockDB) -> Router {
        let cfg = setup_test_config();
        setup(Arc::new(cfg), Arc::new(db)).unwrap()
    }

    fn setup_test_config() -> Config {
        Config::builder()
            .set_default("apiserver.baseURL", "http://localhost:8000")
            .unwrap()
            .set_default("apiserver.staticPath", TESTDATA_PATH)
            .unwrap()
            .set_default("apiserver.basicAuth.enabled", false)
            .unwrap()
            .build()
            .unwrap()
    }

    fn render_index(title: &str, description: &str, image: &str) -> String {
        let mut ctx = Context::new();
        ctx.insert("title", title);
        ctx.insert("description", description);
        ctx.insert("image", image);
        let input = fs::read_to_string(Path::new(TESTDATA_PATH).join("index.html")).unwrap();
        Tera::one_off(&input, &ctx, false).unwrap()
    }
}
