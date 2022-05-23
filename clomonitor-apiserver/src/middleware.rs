use axum::{extract::MatchedPath, http::Request, middleware::Next, response::IntoResponse};
use std::time::Instant;

/// Middleware that collects some metrics about requests processed.
pub(crate) async fn metrics_collector<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();

    // Collect some info from request
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    // Execute next handler in chain
    let response = next.run(req).await;

    // Collect some info from response and track metric
    let duration = start.elapsed().as_secs_f64();
    let labels = [
        ("status", response.status().as_u16().to_string()),
        ("method", method.to_string()),
        ("path", path),
    ];
    metrics::histogram!("http_request_duration", duration, &labels);

    response
}
