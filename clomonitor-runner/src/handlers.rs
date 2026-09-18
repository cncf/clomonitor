use std::{sync::Arc, time::Instant};

use axum::{
    Json,
    body::Bytes,
    extract::{Path, State as AxumState},
    http::{
        HeaderMap, HeaderValue, StatusCode,
        header::{AUTHORIZATION, RETRY_AFTER},
    },
    response::{IntoResponse, Response},
};
use clomonitor_core::tools::{self, Tool, ToolRequest, process::is_deadline_exceeded};
use serde_json::json;
use tokio::time::timeout;
use tracing::{debug, info, warn};

use crate::state::{Admission, State};

/// Value of the Retry-After header returned when the runner is saturated.
const RETRY_AFTER_SECS: u64 = 30;

/// Handler that returns the runner health status and the tools enabled.
pub(crate) async fn health(AxumState(state): AxumState<Arc<State>>) -> impl IntoResponse {
    let tools: serde_json::Map<String, serde_json::Value> = state
        .tools()
        .iter()
        .filter_map(|t| state.tool(*t))
        .map(|e| (e.local.tool.id().to_string(), json!(e.local.version)))
        .collect();
    Json(json!({ "status": "ok", "tools": tools }))
}

/// Handler that runs the tool requested.
pub(crate) async fn run(
    AxumState(state): AxumState<Arc<State>>,
    Path(tool_id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let start = Instant::now();

    // Resolve tool
    let Some(tool) = Tool::from_id(&tool_id) else {
        return error(StatusCode::NOT_FOUND, format!("unknown tool {tool_id:?}"));
    };
    let Some(entry) = state.tool(tool) else {
        return error(StatusCode::NOT_FOUND, format!("tool {tool} not enabled"));
    };

    // Credentials (only accepted for tools that require them)
    let token = match bearer_token(&headers) {
        Ok(token) => token,
        Err(msg) => return error(StatusCode::BAD_REQUEST, msg),
    };
    match (tool.requires_github_token(), token.is_some()) {
        (true, false) => {
            return error(
                StatusCode::BAD_REQUEST,
                format!("{tool} requires a GitHub token (Authorization: Bearer)"),
            );
        }
        (false, true) => {
            return error(
                StatusCode::BAD_REQUEST,
                format!("{tool} does not accept credentials"),
            );
        }
        _ => {}
    }

    // Parse and validate request
    let value: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(value) => value,
        Err(err) => return error(StatusCode::BAD_REQUEST, format!("invalid JSON body: {err}")),
    };
    let request = match ToolRequest::from_json(tool, value) {
        Ok(request) => request,
        Err(err) => return error(StatusCode::BAD_REQUEST, format!("invalid request: {err:#}")),
    };

    // Admission control: wait for a run slot within the request-wide deadline
    let deadline = entry.deadline;
    let queue_guard = match state.enqueue() {
        Admission::Queued(guard) => guard,
        Admission::QueueFull => {
            warn!(%tool, "queue full");
            return unavailable("runner queue is full");
        }
    };
    let Ok(permit) = timeout(
        deadline.saturating_sub(start.elapsed()),
        state.acquire_slot(),
    )
    .await
    else {
        warn!(%tool, "deadline exceeded while waiting for a run slot");
        return error(
            StatusCode::GATEWAY_TIMEOUT,
            "deadline exceeded while waiting for a run slot",
        );
    };
    drop(queue_guard);

    // Do not start a run that cannot complete within the remaining budget
    let remaining = deadline.saturating_sub(start.elapsed());
    if remaining < state.config().min_budget {
        warn!(%tool, ?remaining, "not enough budget left to start the run");
        drop(permit);
        return unavailable("not enough time left to run the tool");
    }

    // Run tool (killed and reaped on deadline or when the request is dropped)
    debug!(%tool, ?remaining, "run started");
    let result = tools::run_local(&request, &entry.local, token.as_deref(), remaining).await;
    drop(permit);

    // Prepare response
    match result {
        Ok(output) => {
            info!(%tool, duration_ms = output.duration_ms, "run completed");
            (StatusCode::OK, Json(output)).into_response()
        }
        Err(err) if is_deadline_exceeded(&err) => {
            warn!(%tool, "run deadline exceeded");
            error(StatusCode::GATEWAY_TIMEOUT, format!("{err:#}"))
        }
        Err(err) => {
            warn!(%tool, error = %format!("{err:#}"), "run failed");
            error(StatusCode::BAD_GATEWAY, format!("{err:#}"))
        }
    }
}

/// Extract the bearer token from the authorization header, if any.
fn bearer_token(headers: &HeaderMap) -> Result<Option<String>, String> {
    let Some(value) = headers.get(AUTHORIZATION) else {
        return Ok(None);
    };
    let value = value
        .to_str()
        .map_err(|_| "invalid authorization header".to_string())?;
    match value.strip_prefix("Bearer ") {
        Some(token) if !token.trim().is_empty() => Ok(Some(token.trim().to_string())),
        _ => Err("invalid authorization header (expected: Bearer <token>)".to_string()),
    }
}

/// Build an error response with the status code and message provided.
fn error(status: StatusCode, msg: impl Into<String>) -> Response {
    (status, Json(json!({ "error": msg.into() }))).into_response()
}

/// Build a service unavailable response asking the client to retry later.
fn unavailable(msg: &str) -> Response {
    let mut response = error(StatusCode::SERVICE_UNAVAILABLE, msg);
    response
        .headers_mut()
        .insert(RETRY_AFTER, HeaderValue::from(RETRY_AFTER_SECS));
    response
}
