use crate::{db::DynDB, handlers::*, middleware::metrics_collector, views::DynVT};
use anyhow::Result;
use axum::{
    extract::FromRef,
    http::{header::CACHE_CONTROL, HeaderValue},
    middleware,
    routing::{get, get_service, post},
    Router,
};
use config::Config;
use std::{path::Path, sync::Arc};
use tera::Tera;
use tower::ServiceBuilder;
use tower_http::{
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeader,
    trace::TraceLayer,
    validate_request::ValidateRequestHeaderLayer,
};

/// Static files cache duration.
pub const STATIC_CACHE_MAX_AGE: usize = 365 * 24 * 60 * 60;

/// Documentation files cache duration.
pub const DOCS_CACHE_MAX_AGE: usize = 300;

/// API server router's state.
#[derive(Clone, FromRef)]
struct RouterState {
    cfg: Arc<Config>,
    db: DynDB,
    vt: DynVT,
    tmpl: Arc<Tera>,
}

/// Setup API server router.
pub(crate) fn setup(cfg: &Arc<Config>, db: DynDB, vt: DynVT) -> Result<Router> {
    // Setup some paths
    let static_path = cfg.get_string("apiserver.staticPath")?;
    let index_path = Path::new(&static_path).join("index.html");
    let docs_path = Path::new(&static_path).join("docs");
    let scorecard_path = Path::new(&static_path).join("scorecard.html");

    // Setup templates
    let mut tmpl = Tera::default();
    tmpl.autoescape_on(vec![]);
    tmpl.add_template_file(index_path, Some("index.html"))?;
    let tmpl = Arc::new(tmpl);

    // Setup API routes
    let api_routes = Router::new()
        .route("/projects/search", get(search_projects))
        .route("/projects/views/:project_id", post(track_view))
        .route("/projects/:foundation/:project", get(project))
        .route("/projects/:foundation/:project/badge", get(badge))
        .route(
            "/projects/:foundation/:project/report-summary",
            get(report_summary_svg),
        )
        .route(
            "/projects/:foundation/:project/:repository/report.md",
            get(repository_report_md),
        )
        .route(
            "/projects/:foundation/:project/snapshots/:date",
            get(project_snapshot),
        )
        .route("/stats", get(stats))
        .route("/stats/snapshots/:date", get(stats_snapshot));

    // Setup router
    let mut router = Router::new()
        .route("/", get(index))
        .route("/projects/:foundation/:project", get(index_project))
        .route(
            "/projects/:foundation/:project/report-summary.png",
            get(report_summary_png),
        )
        .route("/data/repositories.csv", get(repositories_checks))
        .route("/scorecard", get_service(ServeFile::new(scorecard_path)))
        .nest("/api", api_routes)
        .nest_service(
            "/docs",
            get_service(SetResponseHeader::overriding(
                ServeDir::new(docs_path),
                CACHE_CONTROL,
                HeaderValue::try_from(format!("max-age={DOCS_CACHE_MAX_AGE}"))?,
            )),
        )
        .nest_service(
            "/static",
            get_service(SetResponseHeader::overriding(
                ServeDir::new(static_path),
                CACHE_CONTROL,
                HeaderValue::try_from(format!("max-age={STATIC_CACHE_MAX_AGE}"))?,
            )),
        )
        .fallback(index)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(middleware::from_fn(metrics_collector)),
        )
        .with_state(RouterState {
            cfg: cfg.clone(),
            db,
            vt,
            tmpl,
        });

    // Setup basic auth
    if cfg.get_bool("apiserver.basicAuth.enabled").unwrap_or(false) {
        let username = cfg.get_string("apiserver.basicAuth.username")?;
        let password = cfg.get_string("apiserver.basicAuth.password")?;
        router = router.layer(ValidateRequestHeaderLayer::basic(&username, &password));
    }

    Ok(router)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        db::{MockDB, SearchProjectsInput},
        views::MockViewsTracker,
    };
    use axum::{
        body::Body,
        http::{
            header::{CACHE_CONTROL, CONTENT_TYPE},
            Request, StatusCode,
        },
    };
    use clomonitor_core::{linter::*, score::Score};
    use mime::{APPLICATION_JSON, CSV, HTML};
    use mockall::predicate::*;
    use serde_json::json;
    use std::{fs, future, sync::Arc};
    use tera::Context;
    use time::Date;
    use tokio::sync::RwLock;
    use tower::ServiceExt;
    use uuid::Uuid;

    const TESTDATA_PATH: &str = "src/testdata";
    const FOUNDATION: &str = "cncf";
    const PROJECT: &str = "artifact-hub";
    const PROJECT_ID: &str = "00000000-0000-0000-0000-000000000001";
    const DATE: &str = "2022-10-28";
    const REPOSITORY: &str = "artifact-hub";

    #[tokio::test]
    async fn badge_found() {
        let mut db = MockDB::new();
        db.expect_project_rating()
            .with(eq(FOUNDATION), eq(PROJECT))
            .times(1)
            .returning(|_: &str, _: &str| Box::pin(future::ready(Ok(Some("a".to_string())))));

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/api/projects/{FOUNDATION}/{PROJECT}/badge"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers()[CACHE_CONTROL],
            format!("max-age={DEFAULT_API_MAX_AGE}")
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
            .with(eq(FOUNDATION), eq(PROJECT))
            .times(1)
            .returning(|_: &str, _: &str| Box::pin(future::ready(Ok(None))));

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/api/projects/{FOUNDATION}/{PROJECT}/badge"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn docs_files() {
        let response = setup_test_router(MockDB::new(), MockViewsTracker::new())
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
            format!("max-age={DOCS_CACHE_MAX_AGE}")
        );
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            fs::read_to_string(Path::new(TESTDATA_PATH).join("docs").join("topics.html")).unwrap()
        );
    }

    #[tokio::test]
    async fn index() {
        let response = setup_test_router(MockDB::new(), MockViewsTracker::new())
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
            format!("max-age={INDEX_CACHE_MAX_AGE}")
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
        let response = setup_test_router(MockDB::new(), MockViewsTracker::new())
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
            format!("max-age={INDEX_CACHE_MAX_AGE}")
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
        let response = setup_test_router(MockDB::new(), MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/projects/{FOUNDATION}/{PROJECT}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers()[CACHE_CONTROL],
            format!("max-age={INDEX_CACHE_MAX_AGE}")
        );
        assert_eq!(response.headers()[CONTENT_TYPE], HTML.as_ref());
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            render_index(
                PROJECT,
                INDEX_META_DESCRIPTION_PROJECT,
                "http://localhost:8000/projects/cncf/artifact-hub/report-summary.png"
            )
        );
    }

    #[tokio::test]
    async fn project_found() {
        let mut db = MockDB::new();
        db.expect_project_data()
            .with(eq(FOUNDATION), eq(PROJECT))
            .times(1)
            .returning(|_, _| {
                Box::pin(future::ready(Ok(Some(
                    r#"{"project": "info"}"#.to_string(),
                ))))
            });

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/api/projects/{FOUNDATION}/{PROJECT}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers()[CACHE_CONTROL],
            format!("max-age={DEFAULT_API_MAX_AGE}")
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
        db.expect_project_data()
            .with(eq(FOUNDATION), eq(PROJECT))
            .times(1)
            .returning(|_: &str, _: &str| Box::pin(future::ready(Ok(None))));

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/api/projects/{FOUNDATION}/{PROJECT}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn project_snapshot_invalid_date_format() {
        let db = MockDB::new();

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!(
                        "/api/projects/{FOUNDATION}/{PROJECT}/snapshots/20221028"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn project_snapshot_found() {
        let mut db = MockDB::new();
        db.expect_project_snapshot()
            .with(
                eq(FOUNDATION),
                eq(PROJECT),
                eq(Date::parse(DATE, &SNAPSHOT_DATE_FORMAT).unwrap()),
            )
            .times(1)
            .returning(|_, _, _| {
                Box::pin(future::ready(Ok(Some(
                    r#"{"snapshot": "data"}"#.to_string(),
                ))))
            });

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!(
                        "/api/projects/{FOUNDATION}/{PROJECT}/snapshots/{DATE}"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()[CACHE_CONTROL], "max-age=86400");
        assert_eq!(response.headers()[CONTENT_TYPE], APPLICATION_JSON.as_ref());
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            r#"{"snapshot": "data"}"#.to_string(),
        );
    }

    #[tokio::test]
    async fn project_snapshot_not_found() {
        let mut db = MockDB::new();
        db.expect_project_snapshot()
            .with(
                eq(FOUNDATION),
                eq(PROJECT),
                eq(Date::parse(DATE, &SNAPSHOT_DATE_FORMAT).unwrap()),
            )
            .times(1)
            .returning(|_, _, _| Box::pin(future::ready(Ok(None))));

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!(
                        "/api/projects/{FOUNDATION}/{PROJECT}/snapshots/{DATE}"
                    ))
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
            .with(eq(FOUNDATION), eq(PROJECT))
            .times(1)
            .returning(|_: &str, _: &str| Box::pin(future::ready(Ok(None))));

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!(
                        "/projects/{FOUNDATION}/{PROJECT}/report-summary.png"
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
            .with(eq(FOUNDATION), eq(PROJECT))
            .times(1)
            .returning(|_: &str, _: &str| {
                let score = Score {
                    global: 80.0,
                    documentation: Some(80.0),
                    license: Some(50.0),
                    ..Score::default()
                };
                Box::pin(future::ready(Ok(Some(score))))
            });

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!(
                        "/api/projects/{FOUNDATION}/{PROJECT}/report-summary"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers()[CACHE_CONTROL],
            format!("max-age={DEFAULT_API_MAX_AGE}")
        );
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let golden_path = "src/testdata/project-report-summary.golden.svg";
        // fs::write(golden_path, &body).unwrap(); // Uncomment to update golden file
        let golden = fs::read(golden_path).unwrap();
        assert_eq!(body, golden);
    }

    #[tokio::test]
    async fn report_summary_svg_not_found() {
        let mut db = MockDB::new();
        db.expect_project_score()
            .with(eq(FOUNDATION), eq(PROJECT))
            .times(1)
            .returning(|_: &str, _: &str| Box::pin(future::ready(Ok(None))));

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!(
                        "/api/projects/{FOUNDATION}/{PROJECT}/report-summary"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn repositories_checks() {
        let mut db = MockDB::new();
        db.expect_repositories_with_checks()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok("CSV data".to_string()))));

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/data/repositories.csv")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()[CACHE_CONTROL], "max-age=3600");
        assert_eq!(response.headers()[CONTENT_TYPE], CSV.as_ref());
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            "CSV data".to_string(),
        );
    }

    #[tokio::test]
    async fn repository_report_md_found() {
        let mut db = MockDB::new();
        db.expect_repository_report_md()
            .with(eq(FOUNDATION), eq(PROJECT), eq(REPOSITORY))
            .times(1)
            .returning(|_: &str, _: &str, _: &str| {
                let report_md = RepositoryReportMDTemplate {
                    name: "artifact-hub".to_string(),
                    url: "https://github.com/artifacthub/hub".to_string(),
                    check_sets: vec![CheckSet::Code],
                    score: Some(Score {
                        global: 99.999_999_999_999_99,
                        global_weight: 95,
                        documentation: Some(100.0),
                        documentation_weight: Some(30),
                        license: Some(100.0),
                        license_weight: Some(20),
                        best_practices: Some(100.0),
                        best_practices_weight: Some(20),
                        security: Some(100.0),
                        security_weight: Some(20),
                        legal: Some(100.0),
                        legal_weight: Some(5),
                    }),
                    report: Some(Report {
                        documentation: Documentation {
                            adopters: Some(CheckOutput::passed()),
                            annual_review: Some(CheckOutput::passed()),
                            code_of_conduct: Some(CheckOutput::passed()),
                            contributing: Some(CheckOutput::passed()),
                            changelog: Some(CheckOutput::passed()),
                            governance: Some(CheckOutput::passed()),
                            maintainers: Some(CheckOutput::passed()),
                            readme: Some(CheckOutput::passed()),
                            roadmap: Some(CheckOutput::passed()),
                            summary_table: Some(CheckOutput::passed()),
                            website: Some(CheckOutput::passed()),
                        },
                        license: License {
                            license_approved: Some(CheckOutput::passed()),
                            license_scanning: Some(
                                CheckOutput::passed()
                                    .url(Some("https://license-scanning.url".to_string())),
                            ),
                            license_spdx_id: Some(
                                CheckOutput::passed().value(Some("Apache-2.0".to_string())),
                            ),
                        },
                        best_practices: BestPractices {
                            analytics: Some(CheckOutput::passed()),
                            artifacthub_badge: Some(CheckOutput::exempt()),
                            cla: Some(CheckOutput::passed()),
                            community_meeting: Some(CheckOutput::passed()),
                            dco: Some(CheckOutput::passed()),
                            github_discussions: Some(CheckOutput::passed()),
                            openssf_badge: Some(CheckOutput::passed()),
                            openssf_scorecard_badge: Some(CheckOutput::passed()),
                            recent_release: Some(CheckOutput::passed()),
                            slack_presence: Some(CheckOutput::passed()),
                        },
                        security: Security {
                            binary_artifacts: Some(CheckOutput::passed()),
                            code_review: Some(CheckOutput::passed()),
                            dangerous_workflow: Some(CheckOutput::passed()),
                            dependency_update_tool: Some(CheckOutput::passed()),
                            maintained: Some(CheckOutput::passed()),
                            sbom: Some(CheckOutput::passed()),
                            security_policy: Some(CheckOutput::passed()),
                            signed_releases: Some(CheckOutput::passed()),
                            token_permissions: Some(CheckOutput::passed()),
                        },
                        legal: Legal {
                            trademark_disclaimer: Some(CheckOutput::passed()),
                        },
                    }),
                };
                Box::pin(future::ready(Ok(Some(report_md))))
            });

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!(
                        "/api/projects/{FOUNDATION}/{PROJECT}/{REPOSITORY}/report.md"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers()[CACHE_CONTROL],
            format!("max-age={DEFAULT_API_MAX_AGE}")
        );
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let golden_path = "src/testdata/repository-report.golden.md";
        // fs::write(golden_path, &body).unwrap(); // Uncomment to update golden file
        let golden = fs::read(golden_path).unwrap();
        assert_eq!(body, golden);
    }

    #[tokio::test]
    async fn repository_report_md_not_found() {
        let mut db = MockDB::new();
        db.expect_repository_report_md()
            .with(eq(FOUNDATION), eq(PROJECT), eq(REPOSITORY))
            .times(1)
            .returning(|_: &str, _: &str, _: &str| Box::pin(future::ready(Ok(None))));

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!(
                        "/api/projects/{FOUNDATION}/{PROJECT}/{REPOSITORY}/report.md"
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

        let response = setup_test_router(db, MockViewsTracker::new())
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
            format!("max-age={DEFAULT_API_MAX_AGE}")
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
        let response = setup_test_router(MockDB::new(), MockViewsTracker::new())
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
            format!("max-age={STATIC_CACHE_MAX_AGE}")
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
            .withf(|v| v.as_deref() == Some(FOUNDATION))
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(r#"{"some": "stats"}"#.to_string()))));

        let response = setup_test_router(db, MockViewsTracker::new())
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

    #[tokio::test]
    async fn stats_snapshot_invalid_date_format() {
        let db = MockDB::new();

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!(
                        "/api/stats/snapshots/20230105?foundation={FOUNDATION}"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn stats_snapshot_found() {
        let mut db = MockDB::new();
        db.expect_stats_snapshot()
            .withf(|foundation, date| {
                foundation.as_deref() == Some(FOUNDATION)
                    && *date == Date::parse(DATE, &SNAPSHOT_DATE_FORMAT).unwrap()
            })
            .times(1)
            .returning(|_, _| {
                Box::pin(future::ready(Ok(Some(r#"{"some": "stats"}"#.to_string()))))
            });

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!(
                        "/api/stats/snapshots/{DATE}?foundation={FOUNDATION}"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()[CACHE_CONTROL], "max-age=86400");
        assert_eq!(response.headers()[CONTENT_TYPE], APPLICATION_JSON.as_ref());
        assert_eq!(
            hyper::body::to_bytes(response.into_body()).await.unwrap(),
            r#"{"some": "stats"}"#.to_string(),
        );
    }

    #[tokio::test]
    async fn stats_snapshot_not_found() {
        let mut db = MockDB::new();
        db.expect_stats_snapshot()
            .withf(|foundation, date| {
                foundation.as_deref().is_none()
                    && *date == Date::parse(DATE, &SNAPSHOT_DATE_FORMAT).unwrap()
            })
            .times(1)
            .returning(|_, _| Box::pin(future::ready(Ok(None))));

        let response = setup_test_router(db, MockViewsTracker::new())
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/api/stats/snapshots/{DATE}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn track_view() {
        let mut vt = MockViewsTracker::new();
        vt.expect_track_view()
            .withf(|project_id| *project_id == Uuid::parse_str(PROJECT_ID).unwrap())
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(()))));

        let response = setup_test_router(MockDB::new(), vt)
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/projects/views/{PROJECT_ID}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    fn setup_test_router(db: MockDB, vt: MockViewsTracker) -> Router {
        let cfg = setup_test_config();
        setup(&Arc::new(cfg), Arc::new(db), Arc::new(RwLock::new(vt))).unwrap()
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
