use std::sync::Arc;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

use crate::{
    handlers::{health, run},
    state::State,
};

/// Maximum size of a run request body.
const MAX_REQUEST_BODY_BYTES: usize = 64 * 1024;

/// Setup router.
pub(crate) fn setup(state: Arc<State>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/run/{tool}", post(run))
        .layer(DefaultBodyLimit::max(MAX_REQUEST_BODY_BYTES))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

#[cfg(test)]
#[cfg(unix)]
mod tests {
    use std::{
        fs,
        os::unix::fs::PermissionsExt,
        path::{Path, PathBuf},
        time::{Duration, Instant},
    };

    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode, header::AUTHORIZATION},
        response::Response,
    };
    use clomonitor_core::tools::{LocalTool, Tool};
    use serde_json::{Value, json};
    use tower::ServiceExt;

    use crate::state::{RunnerConfig, ToolEntry};

    use super::*;

    /// AFDocs report fixture emitted by the successful afdocs stub.
    const AFDOCS_FIXTURE: &str = "../clomonitor-core/src/testdata/afdocs/report.json";

    #[tokio::test]
    async fn health_lists_enabled_tools() {
        // Setup router with two tools enabled (unsorted)
        let dir = tempfile::tempdir().unwrap();
        let (router, _) = setup_router(
            dir.path(),
            &[
                (Tool::Scorecard, "echo '{}'", ms(5000)),
                (Tool::Afdocs, "echo '{}'", ms(5000)),
            ],
            default_config(),
        );

        // Run the request
        let response = router
            .oneshot(Request::get("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Check the response lists the tools sorted by id with their versions
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            json_body(response).await,
            json!({"status": "ok", "tools": {"afdocs": "0.20.0", "scorecard": "4.13.0"}})
        );
    }

    #[tokio::test]
    async fn run_afdocs_success() {
        // Setup router with an afdocs stub emitting the report fixture
        let dir = tempfile::tempdir().unwrap();
        let (router, _) = setup_router(
            dir.path(),
            &[(Tool::Afdocs, &afdocs_ok_body(), ms(10_000))],
            default_config(),
        );

        // Run the request
        let response = router
            .oneshot(request("afdocs", &afdocs_body(), None))
            .await
            .unwrap();

        // Check the response
        assert_eq!(response.status(), StatusCode::OK);
        let body = json_body(response).await;
        assert_eq!(body["tool"], "afdocs");
        assert_eq!(body["tool_version"], "0.20.0");
        assert!(body["duration_ms"].is_u64());
        assert_eq!(body["output"]["results"].as_array().unwrap().len(), 23);

        // Check the stub received the expected arguments and no token
        let args = stub_record(dir.path(), ".args");
        assert!(args.starts_with("check https://docs.example.org/ --format json --score --sampling deterministic --max-links 20"), "{args}");
        assert!(!stub_record(dir.path(), ".env").contains("GITHUB_TOKEN"));
    }

    #[tokio::test]
    async fn run_credentials_validation() {
        // Setup router with both tools enabled
        let dir = tempfile::tempdir().unwrap();
        let (router, _) = setup_router(
            dir.path(),
            &[
                (Tool::Scorecard, "echo '{}'", ms(5000)),
                (Tool::Afdocs, "echo '{}'", ms(5000)),
            ],
            default_config(),
        );

        // Check scorecard without token is rejected
        let response = router
            .clone()
            .oneshot(request("scorecard", &scorecard_body(), None))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert!(
            json_body(response).await["error"]
                .as_str()
                .unwrap()
                .contains("requires a GitHub token")
        );

        // Check afdocs with token is rejected
        let response = router
            .clone()
            .oneshot(request("afdocs", &afdocs_body(), Some("secret")))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert!(
            json_body(response).await["error"]
                .as_str()
                .unwrap()
                .contains("does not accept credentials")
        );

        // Check a malformed authorization header is rejected
        let response = router
            .oneshot(
                Request::post("/run/scorecard")
                    .header(AUTHORIZATION, "Basic abc")
                    .body(Body::from(scorecard_body().to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert!(
            json_body(response).await["error"]
                .as_str()
                .unwrap()
                .contains("invalid authorization header")
        );
    }

    #[tokio::test]
    async fn run_deadline_kills_tool() {
        // Setup router with a stub outliving its 400ms deadline
        let dir = tempfile::tempdir().unwrap();
        let (router, _) = setup_router(
            dir.path(),
            &[(Tool::Afdocs, "echo $$ > \"$LOG/pid\"; sleep 30", ms(400))],
            RunnerConfig {
                min_budget: ms(10),
                ..default_config()
            },
        );

        // Run the request
        let start = Instant::now();
        let response = router
            .oneshot(request("afdocs", &afdocs_body(), None))
            .await
            .unwrap();

        // Check the request timed out around the deadline
        assert!(start.elapsed() < Duration::from_secs(10));
        assert_eq!(response.status(), StatusCode::GATEWAY_TIMEOUT);

        // Check the tool process is gone
        tokio::time::sleep(ms(100)).await;
        assert_stub_gone(dir.path());
    }

    #[tokio::test]
    async fn run_dropped_request_kills_tool() {
        // Setup router with a long-running stub
        let dir = tempfile::tempdir().unwrap();
        let (router, _) = setup_router(
            dir.path(),
            &[(Tool::Afdocs, "echo $$ > \"$LOG/pid\"; sleep 30", ms(30_000))],
            default_config(),
        );

        // Run the request and wait for the stub to start
        let handle = tokio::spawn(router.oneshot(request("afdocs", &afdocs_body(), None)));
        for _ in 0..50 {
            if dir.path().join("pid").exists() {
                break;
            }
            tokio::time::sleep(ms(50)).await;
        }

        // Simulate the client going away
        handle.abort();
        let _ = handle.await;

        // Check the tool process is gone
        tokio::time::sleep(ms(200)).await;
        assert_stub_gone(dir.path());
    }

    #[tokio::test]
    async fn run_invalid_requests() {
        // Setup router with afdocs enabled
        let dir = tempfile::tempdir().unwrap();
        let (router, _) = setup_router(
            dir.path(),
            &[(Tool::Afdocs, "echo '{}'", ms(5000))],
            default_config(),
        );

        // Check malformed, invalid and mismatched bodies are rejected
        for body in [
            Body::from("not json"),
            Body::from(json!({"url": "ftp://x/"}).to_string()),
            Body::from(json!({"url": "http://user:pw@x/"}).to_string()),
            Body::from(json!({"url": "https://docs.example.org/", "max_links": 500}).to_string()),
            Body::from(json!({"repo_url": "https://github.com/o/r"}).to_string()),
        ] {
            let response = router
                .clone()
                .oneshot(
                    Request::post("/run/afdocs")
                        .header("content-type", "application/json")
                        .body(body)
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }
    }

    #[tokio::test]
    async fn run_near_exhausted_budget_rejected() {
        // Setup a single slot held for 600ms so the queued request gets
        // admitted with ~400ms left, below the 500ms minimum budget
        let dir = tempfile::tempdir().unwrap();
        let (router, _) = setup_router(
            dir.path(),
            &[(Tool::Afdocs, "sleep 0.6; echo '{}'", ms(1000))],
            RunnerConfig {
                max_concurrent_runs: 1,
                max_queue: 2,
                min_budget: ms(500),
            },
        );

        // Run the first request and queue a second one behind it
        let first = tokio::spawn(
            router
                .clone()
                .oneshot(request("afdocs", &afdocs_body(), None)),
        );
        tokio::time::sleep(ms(100)).await;
        let second = router
            .oneshot(request("afdocs", &afdocs_body(), None))
            .await
            .unwrap();

        // Check the first run completes and the second is asked to retry later
        assert_eq!(first.await.unwrap().unwrap().status(), StatusCode::OK);
        assert_eq!(second.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(second.headers()["retry-after"], "30");
        assert!(
            json_body(second).await["error"]
                .as_str()
                .unwrap()
                .contains("not enough time left")
        );
    }

    #[tokio::test]
    async fn run_queued_request_gets_reduced_budget() {
        // Setup a single slot with a stub sleeping 1s: the first run completes,
        // the queued one is admitted with ~1s left of its 1.5s deadline, which
        // is not enough for the tool to finish, so it is killed on its deadline
        let dir = tempfile::tempdir().unwrap();
        let (router, _) = setup_router(
            dir.path(),
            &[(Tool::Afdocs, "sleep 1; echo '{}'", ms(1500))],
            RunnerConfig {
                max_concurrent_runs: 1,
                max_queue: 2,
                min_budget: ms(10),
            },
        );

        // Run the first request and queue a second one behind it
        let start = Instant::now();
        let first = tokio::spawn(
            router
                .clone()
                .oneshot(request("afdocs", &afdocs_body(), None)),
        );
        tokio::time::sleep(ms(100)).await;
        let second = router
            .oneshot(request("afdocs", &afdocs_body(), None))
            .await
            .unwrap();

        // Check the first run completes and the second times out around its
        // deadline, with queue time counted against it
        assert_eq!(first.await.unwrap().unwrap().status(), StatusCode::OK);
        assert_eq!(second.status(), StatusCode::GATEWAY_TIMEOUT);
        assert!(start.elapsed() < ms(2500), "{:?}", start.elapsed());
    }

    #[tokio::test]
    async fn run_saturation_returns_service_unavailable() {
        // Setup a single slot and a single queue position
        let dir = tempfile::tempdir().unwrap();
        let (router, state) = setup_router(
            dir.path(),
            &[(Tool::Afdocs, "sleep 2; echo '{}'", ms(10_000))],
            RunnerConfig {
                max_concurrent_runs: 1,
                max_queue: 1,
                min_budget: ms(10),
            },
        );

        // Run one request and queue another one behind it
        let running = tokio::spawn(
            router
                .clone()
                .oneshot(request("afdocs", &afdocs_body(), None)),
        );
        tokio::time::sleep(ms(200)).await;
        let queued = tokio::spawn(
            router
                .clone()
                .oneshot(request("afdocs", &afdocs_body(), None)),
        );
        tokio::time::sleep(ms(200)).await;
        assert_eq!(state.waiting(), 1);

        // Check a third request is rejected asking the client to retry later
        let response = router
            .oneshot(request("afdocs", &afdocs_body(), None))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(response.headers()["retry-after"], "30");
        assert!(
            json_body(response).await["error"]
                .as_str()
                .unwrap()
                .contains("queue is full")
        );

        // Check the admitted requests complete and the queue is released
        assert_eq!(running.await.unwrap().unwrap().status(), StatusCode::OK);
        assert_eq!(queued.await.unwrap().unwrap().status(), StatusCode::OK);
        assert_eq!(state.waiting(), 0);
    }

    #[tokio::test]
    async fn run_scorecard_injects_token() {
        // Setup router with a scorecard stub echoing the token it received
        let dir = tempfile::tempdir().unwrap();
        let (router, _) = setup_router(
            dir.path(),
            &[(
                Tool::Scorecard,
                "echo \"{\\\"token\\\": \\\"$GITHUB_TOKEN\\\"}\"; exit 0",
                ms(10_000),
            )],
            default_config(),
        );

        // Run the request providing a token
        let response = router
            .oneshot(request("scorecard", &scorecard_body(), Some("secret")))
            .await
            .unwrap();

        // Check the token reached the tool through its environment
        assert_eq!(response.status(), StatusCode::OK);
        let body = json_body(response).await;
        assert_eq!(body["tool"], "scorecard");
        assert_eq!(body["tool_version"], "4.13.0");
        assert_eq!(body["output"], json!({"token": "secret"}));
    }

    #[tokio::test]
    async fn run_tool_failure_is_bad_gateway() {
        // Setup router with a stub failing with an unexpected exit code
        let dir = tempfile::tempdir().unwrap();
        let (router, _) = setup_router(
            dir.path(),
            &[(Tool::Afdocs, "echo boom >&2; exit 2", ms(5000))],
            default_config(),
        );

        // Run the request
        let response = router
            .oneshot(request("afdocs", &afdocs_body(), None))
            .await
            .unwrap();

        // Check the failure is reported with the exit code and stderr
        assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
        let error = json_body(response).await["error"]
            .as_str()
            .unwrap()
            .to_string();
        assert!(
            error.contains("exited with code 2") && error.contains("boom"),
            "{error}"
        );
    }

    #[tokio::test]
    async fn run_unknown_or_disabled_tool() {
        // Setup router with only afdocs enabled
        let dir = tempfile::tempdir().unwrap();
        let (router, _) = setup_router(
            dir.path(),
            &[(Tool::Afdocs, "echo '{}'", ms(5000))],
            default_config(),
        );

        // Check an unknown tool is not found
        let response = router
            .clone()
            .oneshot(request("unknown", &json!({}), None))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert!(
            json_body(response).await["error"]
                .as_str()
                .unwrap()
                .contains("unknown tool")
        );

        // Check a known but disabled tool is not found
        let response = router
            .oneshot(request("scorecard", &scorecard_body(), Some("t")))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert!(
            json_body(response).await["error"]
                .as_str()
                .unwrap()
                .contains("not enabled")
        );
    }

    #[tokio::test]
    async fn run_waiting_for_slot_past_deadline_times_out() {
        // Setup a single slot with a short deadline
        let dir = tempfile::tempdir().unwrap();
        let (router, state) = setup_router(
            dir.path(),
            &[(Tool::Afdocs, "echo '{}'", ms(300))],
            RunnerConfig {
                max_concurrent_runs: 1,
                max_queue: 2,
                min_budget: ms(10),
            },
        );

        // Hold the only slot so the request never gets admitted
        let permit = state.acquire_slot().await;

        // Run the request
        let start = Instant::now();
        let response = router
            .oneshot(request("afdocs", &afdocs_body(), None))
            .await
            .unwrap();

        // Check the request timed out while queued and left the queue
        assert_eq!(response.status(), StatusCode::GATEWAY_TIMEOUT);
        assert!(start.elapsed() < ms(1500));
        assert!(
            json_body(response).await["error"]
                .as_str()
                .unwrap()
                .contains("waiting for a run slot")
        );
        assert_eq!(state.waiting(), 0);
        drop(permit);
    }

    // Helpers.

    /// Request body for an afdocs run.
    fn afdocs_body() -> Value {
        json!({"url": "https://docs.example.org/", "max_links": 20})
    }

    /// Stub body emitting the afdocs report fixture and exiting like afdocs
    /// does when the docs checked do not pass.
    fn afdocs_ok_body() -> String {
        format!(
            "cat \"{}\"; exit 1",
            fs::canonicalize(AFDOCS_FIXTURE).unwrap().display()
        )
    }

    /// Check the stub process that recorded its pid in the directory provided
    /// is no longer alive.
    fn assert_stub_gone(dir: &Path) {
        let pid = fs::read_to_string(dir.join("pid"))
            .unwrap()
            .trim()
            .to_string();
        let alive = fs::read_to_string(format!("/proc/{pid}/status"))
            .is_ok_and(|s| !s.contains("zombie") && !s.contains("dead"));
        assert!(!alive, "process {pid} still alive");
    }

    /// Runner configuration used by tests not exercising admission control.
    fn default_config() -> RunnerConfig {
        RunnerConfig {
            max_concurrent_runs: 2,
            max_queue: 2,
            min_budget: ms(50),
        }
    }

    /// Parse the JSON body of the response provided.
    async fn json_body(response: Response) -> Value {
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    /// Duration in milliseconds.
    fn ms(millis: u64) -> Duration {
        Duration::from_millis(millis)
    }

    /// Build a run request for the tool provided, with an optional bearer
    /// token.
    fn request(tool: &str, body: &Value, token: Option<&str>) -> Request<Body> {
        let mut builder = Request::builder()
            .method("POST")
            .uri(format!("/run/{tool}"))
            .header("content-type", "application/json");
        if let Some(token) = token {
            builder = builder.header(AUTHORIZATION, format!("Bearer {token}"));
        }
        builder.body(Body::from(body.to_string())).unwrap()
    }

    /// Request body for a scorecard run.
    fn scorecard_body() -> Value {
        json!({"repo_url": "https://github.com/org/repo", "checks": ["Code-Review"]})
    }

    /// Setup a router whose tools are stubs created in the directory provided
    /// running the bodies given, returning the shared state as well.
    fn setup_router(
        dir: &Path,
        tools: &[(Tool, &str, Duration)],
        config: RunnerConfig,
    ) -> (Router, Arc<State>) {
        let entries = tools
            .iter()
            .map(|(tool, body, deadline)| ToolEntry {
                deadline: *deadline,
                local: LocalTool {
                    bin: stub(dir, *tool, body),
                    tool: *tool,
                    version: version(*tool).to_string(),
                },
            })
            .collect();
        let state = Arc::new(State::new(entries, config).unwrap());
        (setup(state.clone()), state)
    }

    /// Create a stub tool binary that records how it was invoked and runs
    /// the command body provided.
    fn stub(dir: &Path, tool: Tool, run_body: &str) -> PathBuf {
        let path = dir.join(tool.binary());
        fs::write(
            &path,
            format!(
                r#"#!/bin/sh
LOG="{log}"
env > "$LOG/$$.env"
echo "$@" > "$LOG/$$.args"
{run_body}
"#,
                log = dir.display()
            ),
        )
        .unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    /// Read the record with the extension provided written by the stub in
    /// the directory given.
    fn stub_record(dir: &Path, extension: &str) -> String {
        let entry = fs::read_dir(dir)
            .unwrap()
            .filter_map(Result::ok)
            .find(|e| e.file_name().to_string_lossy().ends_with(extension))
            .unwrap();
        fs::read_to_string(entry.path()).unwrap()
    }

    /// Version the probed stub of the tool provided reports.
    fn version(tool: Tool) -> &'static str {
        match tool {
            Tool::Afdocs => "0.20.0",
            Tool::Scorecard => "4.13.0",
        }
    }
}
