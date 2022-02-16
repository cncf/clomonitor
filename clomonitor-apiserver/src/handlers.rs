use axum::{
    extract,
    extract::Extension,
    http::{
        header::{HeaderMap, HeaderName, HeaderValue},
        StatusCode,
    },
    response,
};
use clomonitor_core::score::Score;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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

    // Prepare badge configuration and return it
    let message: String;
    let color: &str;
    match score {
        Some(Json(score)) => {
            message = score.global().to_string();
            color = match score.rating() {
                'a' => "green",
                'b' => "yellow",
                'c' => "orange",
                'd' => "red",
                _ => "grey",
            };
        }
        None => {
            message = "not processed yet".to_owned();
            color = "grey";
        }
    }
    Ok(response::Json(json!({
        "labelColor": "250948",
        "namedLogo": "cncf",
        "logoColor": "BEB5C8",
        "logoWidth": 10,
        "label": "CloMonitor Score",
        "message": message,
        "color": color,
        "schemaVersion": 1,
        "style": "flat"
    })))
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

/// Handler that returns the requested project.
pub(crate) async fn get_project(
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

/// Helper for mapping any error into a `500 Internal Server Error` response.
fn internal_error<E: std::error::Error>(err: E) -> StatusCode {
    error!("{err}");
    StatusCode::INTERNAL_SERVER_ERROR
}
