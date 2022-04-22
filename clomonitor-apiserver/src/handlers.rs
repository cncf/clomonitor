use super::filters;
use crate::db::{DynDB, SearchProjectsInput};
use anyhow::Error;
use askama_axum::Template;
use axum::{
    body::Full,
    extract,
    extract::{Extension, Query},
    http::{
        header::{CACHE_CONTROL, CONTENT_TYPE},
        Response, StatusCode,
    },
    response::{self, IntoResponse},
};
use clomonitor_core::score::Score;
use mime::APPLICATION_JSON;
use serde_json::json;
use std::{collections::HashMap, fmt::Display};
use tracing::error;

/// Header that indicates the number of items available for pagination purposes.
pub const PAGINATION_TOTAL_COUNT: &str = "pagination-total-count";

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
    Ok(response::Json(json!({
        "labelColor": "3F1D63",
        "namedLogo": "cncf",
        "logoColor": "BEB5C8",
        "logoWidth": 10,
        "label": "CLOMonitor Report",
        "message": message,
        "color": color,
        "schemaVersion": 1,
        "style": "flat"
    })))
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
            let headers = [(CONTENT_TYPE, APPLICATION_JSON.as_ref())];
            Ok((headers, project))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Template for the report summary SVG image.
#[derive(Template)]
#[template(path = "report-summary.svg")]
pub(crate) struct ReportSummaryTemplate {
    pub score: Score,
    pub theme: String,
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
    if score.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Render report summary SVG and return it if the score was found
    match score {
        Some(score) => {
            let theme = match params.get("theme") {
                Some(v) => v.to_owned(),
                _ => "light".to_string(),
            };
            let headers = [(CACHE_CONTROL, "max-age=3600")];
            Ok((headers, ReportSummaryTemplate { score, theme }))
        }
        _ => Err(StatusCode::NOT_FOUND),
    }
}

/// Handler that allows searching for projects.
pub(crate) async fn search_projects(
    Extension(db): Extension<DynDB>,
    extract::Json(input): extract::Json<SearchProjectsInput>,
) -> impl IntoResponse {
    // Search projects in database
    let (count, projects) = db.search_projects(&input).await.map_err(internal_error)?;

    // Return search results as json
    Response::builder()
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
        .stats(params.get("foundation").map(|p| p.to_owned()))
        .await
        .map_err(internal_error)?;

    // Return stats as json
    Response::builder()
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
