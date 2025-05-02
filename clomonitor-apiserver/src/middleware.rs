use std::{sync::LazyLock, time::Instant};

use axum::{
    extract::{MatchedPath, Request},
    middleware::Next,
    response::IntoResponse,
};
use regex::RegexSet;

/// Middleware that collects some metrics about requests processed.
pub(crate) async fn metrics_collector(req: Request, next: Next) -> impl IntoResponse {
    // Define the endpoints we'd like to monitor
    static ENDPOINTS_TO_MONITOR: LazyLock<RegexSet> = LazyLock::new(|| {
        RegexSet::new([
            r"^/api/.*$",
            r"^/projects/:foundation/:org/:project/report-summary.png$",
        ])
        .expect("exprs in ENDPOINTS_TO_MONITOR to be valid")
    });

    let start = Instant::now();

    // Collect some info from request
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    // Execute next handler
    let response = next.run(req).await;

    // Collect some info from response and track metric if the path matches
    // any of the endpoints we'd like to monitor
    if ENDPOINTS_TO_MONITOR.is_match(&path) {
        let duration = start.elapsed().as_secs_f64();
        let labels = [
            ("status", response.status().as_u16().to_string()),
            ("method", method.to_string()),
            ("path", path),
        ];
        metrics::histogram!("clomonitor_apiserver_http_request_duration", &labels).record(duration);
    }

    response
}
