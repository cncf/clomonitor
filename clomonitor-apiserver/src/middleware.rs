use axum::{extract::MatchedPath, http::Request, middleware::Next, response::IntoResponse};
use lazy_static::lazy_static;
use regex::RegexSet;
use std::time::Instant;

/// Middleware that collects some metrics about requests processed.
pub(crate) async fn metrics_collector<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    // Define the endpoints we'd like to monitor
    lazy_static! {
        static ref ENDPOINTS_TO_MONITOR: RegexSet = RegexSet::new(vec![
            r"^/api/.*$",
            r"^/projects/:foundation/:org/:project/report-summary.png$",
        ])
        .expect("exprs in ENDPOINTS_TO_MONITOR to be valid");
    }

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
        metrics::histogram!(
            "clomonitor_apiserver_http_request_duration",
            duration,
            &labels
        );
    }

    response
}
