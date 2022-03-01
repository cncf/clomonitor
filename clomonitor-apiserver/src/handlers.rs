use super::filters;
use askama_axum::Template;
use axum::{
    extract,
    extract::{Extension, Query},
    http::{
        header::{HeaderMap, HeaderName, HeaderValue},
        StatusCode,
    },
    response,
};
use clomonitor_core::score::{primary, Score};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use tokio_postgres::types::Json;
use tracing::error;

/// Header that indicates the number of items available for pagination purposes.
const PAGINATION_TOTAL_COUNT: &str = "pagination-total-count";

/// Query input used when searching for projects.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SearchProjectsInput {
    limit: Option<usize>,
    offset: Option<usize>,
    sort_by: Option<String>,
    sort_direction: Option<String>,
    text: Option<String>,
    category: Option<Vec<usize>>,
    maturity: Option<Vec<usize>>,
    rating: Option<Vec<char>>,
}

/// Handler that returns the information needed to render the project's badge.
pub(crate) async fn badge(
    Extension(db_pool): Extension<Pool>,
    extract::Path((org, project)): extract::Path<(String, String)>,
) -> Result<response::Json<Value>, StatusCode> {
    // Get project rating from database
    let db = db_pool.get().await.map_err(internal_error)?;
    let rows = db
        .query(
            "
            select rating
            from project p
            join organization o using (organization_id)
            where o.name = $1::text
            and p.name = $2::text
            ",
            &[&org, &project],
        )
        .await
        .map_err(internal_error)?;
    if rows.len() != 1 {
        return Err(StatusCode::NOT_FOUND);
    }
    let rating: Option<String> = rows.first().unwrap().get("rating");

    // Prepare badge configuration and return it
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
    Extension(db_pool): Extension<Pool>,
    extract::Path((org, project)): extract::Path<(String, String)>,
) -> Result<response::Json<Value>, StatusCode> {
    // Get project from database
    let db = db_pool.get().await.map_err(internal_error)?;
    let row = db
        .query_one("select get_project($1::text, $2::text)", &[&org, &project])
        .await
        .map_err(internal_error)?;
    let project: Option<Json<Value>> = row.get(0);

    match project {
        Some(Json(project)) => Ok(response::Json(project)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Template for the report summary SVG image.
#[derive(Template)]
#[template(path = "report-summary.svg")]
pub struct ReportSummaryTemplate {
    pub score: primary::Score,
    pub theme: String,
}

/// Handler that returns an SVG image with the project's report summary.
pub(crate) async fn report_summary_svg(
    Extension(db_pool): Extension<Pool>,
    extract::Path((org, project)): extract::Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<ReportSummaryTemplate, StatusCode> {
    // Get project score from database
    let db = db_pool.get().await.map_err(internal_error)?;
    let rows = db
        .query(
            "
            select score
            from project p
            join organization o using (organization_id)
            where o.name = $1::text
            and p.name = $2::text
            ",
            &[&org, &project],
        )
        .await
        .map_err(internal_error)?;
    if rows.len() != 1 {
        return Err(StatusCode::NOT_FOUND);
    }
    let score: Option<Json<Score>> = rows.first().unwrap().get("score");

    // Render report summary SVG and return it
    match score {
        Some(Json(Score::Primary(score))) => {
            let theme = match params.get("theme") {
                Some(v) => v.to_owned(),
                _ => "light".to_string(),
            };
            Ok(ReportSummaryTemplate { score, theme })
        }
        _ => Err(StatusCode::NOT_FOUND),
    }
}

/// Handler that allows searching for projects.
pub(crate) async fn search_projects(
    Extension(db_pool): Extension<Pool>,
    extract::Json(input): extract::Json<SearchProjectsInput>,
) -> Result<(HeaderMap, response::Json<Value>), StatusCode> {
    // Search projects in database
    let db = db_pool.get().await.map_err(internal_error)?;
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
fn internal_error<E: std::error::Error>(err: E) -> StatusCode {
    error!("{err}");
    StatusCode::INTERNAL_SERVER_ERROR
}
