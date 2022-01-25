use axum::{
    extract,
    extract::Extension,
    http::{
        header::{HeaderMap, HeaderName, HeaderValue},
        StatusCode,
    },
    response,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::types::Json;
use uuid::Uuid;

/// Header that indicates the number of items available for pagination purposes.
const PAGINATION_TOTAL_COUNT: &str = "pagination-total-count";

/// Query input used when searching for projects.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SearchProjectsInput {
    limit: Option<usize>,
    offset: Option<usize>,
    text: Option<String>,
    category: Option<Vec<usize>>,
    maturity: Option<Vec<usize>>,
    rating: Option<Vec<char>>,
}

/// Handler that allows searching for projects.
pub(crate) async fn search_projects(
    Extension(db_pool): Extension<Pool>,
    extract::Json(input): extract::Json<SearchProjectsInput>,
) -> Result<(HeaderMap, response::Json<Value>), StatusCode> {
    let db = db_pool.get().await.map_err(internal_error)?;
    let row = db
        .query_one("select * from search_projects($1::jsonb)", &[&Json(input)])
        .await
        .map_err(internal_error)?;
    let Json(projects): Json<Value> = row.get("projects");
    let total_count: i64 = row.get("total_count");

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
    extract::Path(project_id): extract::Path<Uuid>,
) -> Result<response::Json<Value>, StatusCode> {
    let db = db_pool.get().await.map_err(internal_error)?;
    let row = db
        .query_one("select get_project($1::uuid)", &[&project_id])
        .await
        .map_err(internal_error)?;

    let project: Option<Json<Value>> = row.get(0);
    match project {
        Some(Json(project)) => Ok(response::Json(project)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Helper for mapping any error into a `500 Internal Server Error` response.
fn internal_error<E: std::error::Error>(_: E) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}
