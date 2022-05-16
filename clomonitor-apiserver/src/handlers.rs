use super::filters;
use crate::db::{DynDB, SearchProjectsInput};
use anyhow::Error;
use askama_axum::Template;
use axum::{
    body::Full,
    extract,
    extract::{Extension, Query, RawQuery},
    http::{
        header::{CACHE_CONTROL, CONTENT_TYPE},
        Response, StatusCode,
    },
    response::{self, IntoResponse},
};
use clomonitor_core::score::Score;
use config::Config;
use mime::{APPLICATION_JSON, HTML, PNG};
use serde_json::json;
use std::{collections::HashMap, fmt::Display, sync::Arc};
use tera::{Context, Tera};
use tracing::error;

/// Index HTML document cache duration.
pub const INDEX_CACHE_MAX_AGE: usize = 300;

/// Default cache duration for some API endpoints.
pub const DEFAULT_API_MAX_AGE: usize = 300;

/// Header that indicates the number of items available for pagination purposes.
pub const PAGINATION_TOTAL_COUNT: &str = "pagination-total-count";

/// Metadata used when rendering the index HTML document.
pub const INDEX_META_TITLE: &str = "CLOMonitor";
pub const INDEX_META_DESCRIPTION: &str = "CLOMonitor is a tool that periodically checks open source projects repositories to verify they meet certain project health best practices.";
pub const INDEX_META_DESCRIPTION_PROJECT: &str = "CLOMonitor report summary";

/// Report summary image dimensions.
pub const REPORT_SUMMARY_WIDTH: u32 = 900;
pub const REPORT_SUMMARY_HEIGHT: u32 = 470;

/// Handler that returns the information needed to render the project's badge.
pub(crate) async fn badge(
    Extension(db): Extension<DynDB>,
    extract::Path((foundation, org, project)): extract::Path<(String, String, String)>,
) -> impl IntoResponse {
    // Get project rating from database
    let rating = db
        .project_rating(&foundation, &org, &project)
        .await
        .map_err(internal_error)?;
    if rating.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Prepare badge configuration
    let message: String;
    let color: &str;
    match rating {
        Some(rating) => {
            message = rating.to_uppercase();
            color = match rating.as_ref() {
                "a" => "green",
                "b" => "yellow",
                "c" => "orange",
                "d" => "red",
                _ => "grey",
            };
        }
        None => {
            message = "not processed yet".to_owned();
            color = "grey";
        }
    }

    // Return badge configuration as json
    let headers = [(CACHE_CONTROL, format!("max-age={}", DEFAULT_API_MAX_AGE))];
    Ok((
        headers,
        response::Json(json!({
            "labelColor": "3F1D63",
            "namedLogo": "cncf",
            "logoColor": "BEB5C8",
            "logoWidth": 10,
            "label": "CLOMonitor Report",
            "message": message,
            "color": color,
            "schemaVersion": 1,
            "style": "flat"
        })),
    ))
}

/// Handler that returns the index HTML document with some metadata embedded.
pub(crate) async fn index(
    Extension(cfg): Extension<Arc<Config>>,
    Extension(tmpl): Extension<Arc<Tera>>,
) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("title", INDEX_META_TITLE);
    ctx.insert("description", INDEX_META_DESCRIPTION);
    ctx.insert(
        "image",
        &format!(
            "{}/static/media/clomonitor.png",
            cfg.get_string("apiserver.baseURL")
                .expect("baseURL not found"),
        ),
    );

    let headers = [
        (CACHE_CONTROL, format!("max-age={}", INDEX_CACHE_MAX_AGE)),
        (CONTENT_TYPE, HTML.to_string()),
    ];
    (
        headers,
        tmpl.render("index.html", &ctx).map_err(internal_error),
    )
}

/// Handler that returns the index HTML document with some project specific
/// metadata embedded.
pub(crate) async fn index_project(
    Extension(cfg): Extension<Arc<Config>>,
    Extension(tmpl): Extension<Arc<Tera>>,
    extract::Path((foundation, org, project)): extract::Path<(String, String, String)>,
) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("title", &project);
    ctx.insert("description", INDEX_META_DESCRIPTION_PROJECT);
    ctx.insert(
        "image",
        &format!(
            "{}/projects/{}/{}/{}/report-summary.png",
            cfg.get_string("apiserver.baseURL")
                .expect("baseURL not found"),
            &foundation,
            &org,
            &project
        ),
    );

    let headers = [
        (CACHE_CONTROL, format!("max-age={}", INDEX_CACHE_MAX_AGE)),
        (CONTENT_TYPE, HTML.to_string()),
    ];
    (
        headers,
        tmpl.render("index.html", &ctx).map_err(internal_error),
    )
}

/// Handler that returns the requested project.
pub(crate) async fn project(
    Extension(db): Extension<DynDB>,
    extract::Path((foundation, org, project)): extract::Path<(String, String, String)>,
) -> impl IntoResponse {
    // Get project from database
    let project = db
        .project(&foundation, &org, &project)
        .await
        .map_err(internal_error)?;

    // Return project information as json if found
    match project {
        Some(project) => {
            let headers = [
                (CACHE_CONTROL, format!("max-age={}", DEFAULT_API_MAX_AGE)),
                (CONTENT_TYPE, APPLICATION_JSON.to_string()),
            ];
            Ok((headers, project))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Template for the report summary SVG image.
#[derive(Debug, Clone, Template)]
#[template(path = "report-summary.svg")]
pub(crate) struct ReportSummaryTemplate {
    pub score: Score,
    pub theme: String,
}

/// Handler that returns a PNG image with the project's report summary.
pub(crate) async fn report_summary_png(
    Extension(db): Extension<DynDB>,
    extract::Path((foundation, org, project)): extract::Path<(String, String, String)>,
) -> impl IntoResponse {
    // Get project score from database
    let score = db
        .project_score(&foundation, &org, &project)
        .await
        .map_err(internal_error)?;
    if score.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Render report summary SVG
    let svg = ReportSummaryTemplate {
        score: score.unwrap(),
        theme: "light".to_string(),
    }
    .render()
    .map_err(internal_error)?;

    // Convert report summary SVG to PNG
    let mut opt = usvg::Options::default();
    opt.fontdb.load_system_fonts();
    opt.font_family = "Open Sans Semibold".to_string();
    let rtree = usvg::Tree::from_data(svg.as_bytes(), &opt.to_ref()).map_err(internal_error)?;
    let mut pixmap = tiny_skia::Pixmap::new(REPORT_SUMMARY_WIDTH, REPORT_SUMMARY_HEIGHT).unwrap();
    resvg::render(
        &rtree,
        usvg::FitTo::Size(REPORT_SUMMARY_WIDTH, REPORT_SUMMARY_HEIGHT),
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .unwrap();
    let png = pixmap.encode_png().map_err(internal_error)?;

    let headers = [
        (CACHE_CONTROL, format!("max-age={}", DEFAULT_API_MAX_AGE)),
        (CONTENT_TYPE, PNG.to_string()),
    ];
    Ok((headers, png))
}

/// Handler that returns an SVG image with the project's report summary.
pub(crate) async fn report_summary_svg(
    Extension(db): Extension<DynDB>,
    extract::Path((foundation, org, project)): extract::Path<(String, String, String)>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // Get project score from database
    let score = db
        .project_score(&foundation, &org, &project)
        .await
        .map_err(internal_error)?;

    // Render report summary SVG and return it if the score was found
    match score {
        Some(score) => {
            let theme = match params.get("theme") {
                Some(v) => v.to_owned(),
                _ => "light".to_string(),
            };
            let headers = [(CACHE_CONTROL, format!("max-age={}", DEFAULT_API_MAX_AGE))];
            Ok((headers, ReportSummaryTemplate { score, theme }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Handler that allows searching for projects.
pub(crate) async fn search_projects(
    Extension(db): Extension<DynDB>,
    RawQuery(query): RawQuery,
) -> impl IntoResponse {
    // Search projects in database
    let query = query.unwrap_or_default();
    let input: SearchProjectsInput =
        serde_qs::from_str(&query).map_err(|_| StatusCode::BAD_REQUEST)?;
    let (count, projects) = db.search_projects(&input).await.map_err(internal_error)?;

    // Return search results as json
    Response::builder()
        .header(CACHE_CONTROL, format!("max-age={}", DEFAULT_API_MAX_AGE))
        .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
        .header(PAGINATION_TOTAL_COUNT, count.to_string())
        .body(Full::from(projects))
        .map_err(internal_error)
}

/// Handler that returns some general stats.
pub(crate) async fn stats(
    Extension(db): Extension<DynDB>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // Get stats from database
    let stats = db
        .stats(params.get("foundation"))
        .await
        .map_err(internal_error)?;

    // Return stats as json
    Response::builder()
        .header(CACHE_CONTROL, "max-age=3600")
        .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
        .body(Full::from(stats))
        .map_err(internal_error)
}

/// Helper for mapping any error into a `500 Internal Server Error` response.
fn internal_error<E>(err: E) -> StatusCode
where
    E: Into<Error> + Display,
{
    error!("{err}");
    StatusCode::INTERNAL_SERVER_ERROR
}
