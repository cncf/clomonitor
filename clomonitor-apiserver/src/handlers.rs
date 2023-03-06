use super::filters;
use crate::{
    db::{DynDB, SearchProjectsInput},
    views::DynVT,
};
use anyhow::Error;
use askama_axum::Template;
use axum::{
    body::Full,
    extract::{Path, Query, RawQuery, State},
    http::{
        header::{CACHE_CONTROL, CONTENT_TYPE},
        Response, StatusCode,
    },
    response::{self, IntoResponse},
};
use clomonitor_core::{
    linter::{CheckSet, Report},
    score::Score,
};
use config::Config;
use lazy_static::lazy_static;
use mime::{APPLICATION_JSON, CSV, HTML, PNG};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, fmt::Display, sync::Arc};
use tera::{Context, Tera};
use time::{
    format_description::{self, FormatItem},
    Date,
};
use tracing::error;
use uuid::Uuid;

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

lazy_static! {
    /// Format used in snapshots dates.
    pub static ref SNAPSHOT_DATE_FORMAT: Vec<FormatItem<'static>> =
        format_description::parse("[year]-[month]-[day]")
        .expect("format to be valid");
}

/// Handler that returns the information needed to render the project's badge.
pub(crate) async fn badge(
    State(db): State<DynDB>,
    Path((foundation, project)): Path<(String, String)>,
) -> impl IntoResponse {
    // Get project rating from database
    let rating = db
        .project_rating(&foundation, &project)
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
    let headers = [(CACHE_CONTROL, format!("max-age={DEFAULT_API_MAX_AGE}"))];
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
    State(cfg): State<Arc<Config>>,
    State(tmpl): State<Arc<Tera>>,
) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("title", INDEX_META_TITLE);
    ctx.insert("description", INDEX_META_DESCRIPTION);
    ctx.insert(
        "image",
        &format!(
            "{}/static/media/clomonitor.png",
            cfg.get_string("apiserver.baseURL")
                .expect("base url to be set"),
        ),
    );

    let headers = [
        (CACHE_CONTROL, format!("max-age={INDEX_CACHE_MAX_AGE}")),
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
    State(cfg): State<Arc<Config>>,
    State(tmpl): State<Arc<Tera>>,
    Path((foundation, project)): Path<(String, String)>,
) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("title", &project);
    ctx.insert("description", INDEX_META_DESCRIPTION_PROJECT);
    ctx.insert(
        "image",
        &format!(
            "{}/projects/{}/{}/report-summary.png",
            cfg.get_string("apiserver.baseURL")
                .expect("base url to be set"),
            &foundation,
            &project
        ),
    );

    let headers = [
        (CACHE_CONTROL, format!("max-age={INDEX_CACHE_MAX_AGE}")),
        (CONTENT_TYPE, HTML.to_string()),
    ];
    (
        headers,
        tmpl.render("index.html", &ctx).map_err(internal_error),
    )
}

/// Handler that returns some information about the requested project.
pub(crate) async fn project(
    State(db): State<DynDB>,
    Path((foundation, project)): Path<(String, String)>,
) -> impl IntoResponse {
    // Get project from database
    let project = db
        .project_data(&foundation, &project)
        .await
        .map_err(internal_error)?;

    // Return project information as json if found
    match project {
        Some(project) => {
            let headers = [
                (CACHE_CONTROL, format!("max-age={DEFAULT_API_MAX_AGE}")),
                (CONTENT_TYPE, APPLICATION_JSON.to_string()),
            ];
            Ok((headers, project))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Handler that returns the requested project snapshot.
pub(crate) async fn project_snapshot(
    State(db): State<DynDB>,
    Path((foundation, project, date)): Path<(String, String, String)>,
) -> impl IntoResponse {
    // Parse date
    let date: Date =
        Date::parse(&date, &SNAPSHOT_DATE_FORMAT).map_err(|_| StatusCode::BAD_REQUEST)?;

    // Get project snapshot from database
    let project = db
        .project_snapshot(&foundation, &project, &date)
        .await
        .map_err(internal_error)?;

    // Return project snapshot data if found
    match project {
        Some(project) => {
            let headers = [
                (CACHE_CONTROL, format!("max-age={}", 24 * 60 * 60)),
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

impl ReportSummaryTemplate {
    fn new(score: Score, theme: Option<String>) -> Self {
        let theme = theme.unwrap_or_else(|| "light".to_string());
        Self { score, theme }
    }
}

/// Handler that returns a PNG image with the project's report summary.
pub(crate) async fn report_summary_png(
    State(db): State<DynDB>,
    Path((foundation, project)): Path<(String, String)>,
) -> impl IntoResponse {
    // Get project score from database
    let score = db
        .project_score(&foundation, &project)
        .await
        .map_err(internal_error)?;
    if score.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Render report summary SVG
    let svg = ReportSummaryTemplate::new(score.expect("checked if is some above"), None)
        .render()
        .map_err(internal_error)?;

    // Convert report summary SVG to PNG
    let mut opt = usvg::Options {
        font_family: "Open Sans SemiBold".to_string(),
        ..Default::default()
    };
    opt.fontdb.load_system_fonts();
    let tree = usvg::Tree::from_data(svg.as_bytes(), &opt.to_ref()).map_err(internal_error)?;
    let mut pixmap = tiny_skia::Pixmap::new(REPORT_SUMMARY_WIDTH, REPORT_SUMMARY_HEIGHT)
        .expect("width or height defined in consts are not zero");
    resvg::render(
        &tree,
        usvg::FitTo::Size(REPORT_SUMMARY_WIDTH, REPORT_SUMMARY_HEIGHT),
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .expect("width or height defined in consts are not zero");
    let png = pixmap.encode_png().map_err(internal_error)?;

    let headers = [
        (CACHE_CONTROL, format!("max-age={DEFAULT_API_MAX_AGE}")),
        (CONTENT_TYPE, PNG.to_string()),
    ];
    Ok((headers, png))
}

/// Handler that returns an SVG image with the project's report summary.
pub(crate) async fn report_summary_svg(
    State(db): State<DynDB>,
    Path((foundation, project)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // Get project score from database
    let score = db
        .project_score(&foundation, &project)
        .await
        .map_err(internal_error)?;

    // Render report summary SVG and return it if the score was found
    match score {
        Some(score) => {
            let headers = [(CACHE_CONTROL, format!("max-age={DEFAULT_API_MAX_AGE}"))];
            let theme = params.get("theme").cloned();
            Ok((headers, ReportSummaryTemplate::new(score, theme)))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Handler that returns all repositories with checks details in CSV format.
pub(crate) async fn repositories_checks(State(db): State<DynDB>) -> impl IntoResponse {
    // Get all repositories from database
    let repos = db
        .repositories_with_checks()
        .await
        .map_err(internal_error)?;

    Response::builder()
        .header(CACHE_CONTROL, "max-age=3600")
        .header(CONTENT_TYPE, CSV.as_ref())
        .body(Full::from(repos))
        .map_err(internal_error)
}

/// Template for the repository report in markdown format.
#[derive(Debug, Clone, Template, Serialize, Deserialize)]
#[template(path = "repository-report.md")]
pub(crate) struct RepositoryReportMDTemplate {
    pub name: String,
    pub url: String,
    pub check_sets: Vec<CheckSet>,
    pub score: Option<Score>,
    pub report: Option<Report>,
}

/// Handler that returns the repository's report in markdown format.
pub(crate) async fn repository_report_md(
    State(db): State<DynDB>,
    Path((foundation, project, repository)): Path<(String, String, String)>,
) -> impl IntoResponse {
    // Get repository report info from database
    let report_md = db
        .repository_report_md(&foundation, &project, &repository)
        .await
        .map_err(internal_error)?;

    // Render repository report in markdown format and return it
    match report_md {
        Some(report_md) => {
            let headers = [(CACHE_CONTROL, format!("max-age={DEFAULT_API_MAX_AGE}"))];
            Ok((headers, report_md))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Handler that allows searching for projects.
pub(crate) async fn search_projects(
    State(db): State<DynDB>,
    RawQuery(query): RawQuery,
) -> impl IntoResponse {
    // Search projects in database
    let query = query.unwrap_or_default();
    let input: SearchProjectsInput =
        serde_qs::from_str(&query).map_err(|_| StatusCode::BAD_REQUEST)?;
    let (count, projects) = db.search_projects(&input).await.map_err(internal_error)?;

    // Return search results as json
    Response::builder()
        .header(CACHE_CONTROL, format!("max-age={DEFAULT_API_MAX_AGE}"))
        .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
        .header(PAGINATION_TOTAL_COUNT, count.to_string())
        .body(Full::from(projects))
        .map_err(internal_error)
}

/// Handler that returns some general stats.
pub(crate) async fn stats(
    State(db): State<DynDB>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // Get stats from database
    let stats = db
        .stats(params.get("foundation").map(|p| p.as_str()))
        .await
        .map_err(internal_error)?;

    // Return stats as json
    Response::builder()
        .header(CACHE_CONTROL, "max-age=3600")
        .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
        .body(Full::from(stats))
        .map_err(internal_error)
}

/// Handler that returns the requested stats snapshot.
pub(crate) async fn stats_snapshot(
    State(db): State<DynDB>,
    Path(date): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // Get stats snapshot from database
    let foundation = params.get("foundation").map(|f| f.as_str());
    let date: Date =
        Date::parse(&date, &SNAPSHOT_DATE_FORMAT).map_err(|_| StatusCode::BAD_REQUEST)?;
    let stats = db
        .stats_snapshot(foundation, &date)
        .await
        .map_err(internal_error)?;

    // Return snapshot data if found
    match stats {
        Some(stats) => {
            let headers = [
                (CACHE_CONTROL, format!("max-age={}", 24 * 60 * 60)),
                (CONTENT_TYPE, APPLICATION_JSON.to_string()),
            ];
            Ok((headers, stats))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Handler used to track a project view.
pub(crate) async fn track_view(
    State(vt): State<DynVT>,
    Path(project_id): Path<Uuid>,
) -> impl IntoResponse {
    match vt.read().await.track_view(project_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => internal_error(err),
    }
}

/// Helper for mapping any error into a `500 Internal Server Error` response.
fn internal_error<E>(err: E) -> StatusCode
where
    E: Into<Error> + Display,
{
    error!(%err);
    StatusCode::INTERNAL_SERVER_ERROR
}
