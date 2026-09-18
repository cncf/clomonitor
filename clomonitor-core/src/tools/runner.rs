//! Generic client for `clomonitor-runner`, the service that runs external
//! tools on request. The client is tool agnostic: callers provide a
//! [`ToolRequest`] (and credentials when the tool requires them) and get back
//! the raw [`RunOutput`] to interpret.
//!
//! Runner API: `POST {base_url}/run/{tool}` with the request JSON as body.
//! Credentials travel in the `Authorization: Bearer` header and are only sent
//! for tools that require them. Responses: `200` with a [`RunOutput`] JSON,
//! `429`/`503` (retryable, may include `Retry-After` in seconds) or any other
//! status with a JSON `{"error": "..."}` body (fatal for the run).

use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail, format_err};
use reqwest::{
    StatusCode,
    header::{AUTHORIZATION, RETRY_AFTER},
};
use serde::Deserialize;
use tokio::time::{sleep, timeout};

use super::{MAX_OUTPUT_BYTES, RunOutput, ToolRequest};

/// Maximum number of bytes of an error response body included in errors.
const MAX_ERROR_BODY_BYTES: usize = 512;

/// Maximum number of retries (connect errors, 429 and 503 only).
const MAX_RETRIES: u32 = 2;

/// Base backoff between retries (multiplied by the attempt number).
const RETRY_BACKOFF_BASE: Duration = Duration::from_secs(5);

/// Maximum backoff between retries.
const RETRY_BACKOFF_MAX: Duration = Duration::from_secs(15);

/// Margin added to the tool deadline when deciding if a retry fits in the
/// remaining budget (also used as per-request timeout slack).
const RETRY_BUDGET_MARGIN: Duration = Duration::from_secs(10);

/// Runner client.
#[derive(Debug, Clone)]
pub struct RunnerClient {
    /// Retry backoff settings.
    backoff: Backoff,
    /// Runner base url (without trailing slash).
    base_url: String,
    /// HTTP client used for the requests.
    http: reqwest::Client,

    /// Timeouts override (derived from the tool when not set).
    timeouts: Option<Timeouts>,
}

impl RunnerClient {
    /// Create a new client for the runner at the base url provided.
    ///
    /// # Errors
    ///
    /// Returns an error when the http client cannot be set up.
    pub fn new(base_url: &str) -> Result<Self> {
        // Dedicated client with no default credentials: tokens are only sent
        // explicitly for tools that require them
        let http = reqwest::Client::builder()
            .user_agent("clomonitor")
            .build()
            .context("error setting up runner http client")?;
        Ok(Self {
            backoff: Backoff {
                base: RETRY_BACKOFF_BASE,
                budget_margin: RETRY_BUDGET_MARGIN,
                max: RETRY_BACKOFF_MAX,
            },
            base_url: base_url.trim_end_matches('/').to_string(),
            http,
            timeouts: None,
        })
    }

    /// Run the request provided in the runner.
    ///
    /// # Errors
    ///
    /// Returns an error when the runner cannot be reached within the budget,
    /// rejects the request or the run fails.
    pub async fn run(
        &self,
        request: &ToolRequest,
        github_token: Option<&str>,
    ) -> Result<RunOutput> {
        // Validate the request and the credentials it needs
        let tool = request.tool();
        request.validate()?;
        if tool.requires_github_token() && github_token.is_none() {
            bail!("{tool} requires a GitHub token");
        }

        // Resolve the timeouts and the credentials to forward (only for
        // tools that require them)
        let timeouts = self.timeouts.unwrap_or(Timeouts {
            client: tool.client_timeout(),
            server: tool.deadline(),
        });
        let credentials = github_token.filter(|_| tool.requires_github_token());

        // Attempt the run, retrying with backoff while the budget allows it
        let start = Instant::now();
        let attempts = async {
            let mut attempt: u32 = 0;
            loop {
                attempt += 1;
                match self.attempt(request, credentials, timeouts).await? {
                    Attempt::Ok(output) => return Ok(output),
                    Attempt::Retry {
                        reason,
                        retry_after,
                    } => {
                        if attempt > MAX_RETRIES {
                            bail!("{reason} (giving up after {attempt} attempts)");
                        }
                        let backoff = retry_after
                            .unwrap_or(self.backoff.base * attempt)
                            .clamp(self.backoff.base, self.backoff.max);
                        let remaining = timeouts.client.saturating_sub(start.elapsed());
                        if remaining < backoff + timeouts.server + self.backoff.budget_margin {
                            bail!("{reason} (not enough time left to retry)");
                        }
                        sleep(backoff).await;
                    }
                }
            }
        };

        // Enforce the overall client deadline
        match timeout(timeouts.client, attempts).await {
            Ok(result) => result,
            Err(_) => Err(format_err!(
                "runner request for {tool} timed out after {}s",
                timeouts.client.as_secs()
            )),
        }
    }

    /// Override the retry backoff settings (mostly useful for tests).
    #[must_use]
    pub fn with_backoff(mut self, base: Duration, max: Duration, budget_margin: Duration) -> Self {
        self.backoff = Backoff {
            base,
            budget_margin,
            max,
        };
        self
    }

    /// Override the timeouts (mostly useful for tests).
    #[must_use]
    pub fn with_timeouts(mut self, client: Duration, server: Duration) -> Self {
        self.timeouts = Some(Timeouts { client, server });
        self
    }

    /// Perform a single request attempt.
    async fn attempt(
        &self,
        request: &ToolRequest,
        credentials: Option<&str>,
        timeouts: Timeouts,
    ) -> Result<Attempt> {
        // Build the request, attaching the credentials only when provided
        let tool = request.tool();
        let mut req = self
            .http
            .post(format!("{}/run/{}", self.base_url, tool.id()))
            .json(&request.to_json())
            .timeout(timeouts.server + self.backoff.budget_margin);
        if let Some(token) = credentials {
            req = req.header(AUTHORIZATION, format!("Bearer {token}"));
        }

        // Send the request, treating connection errors as retryable
        let response = match req.send().await {
            Ok(response) => response,
            Err(err) if err.is_connect() => {
                return Ok(Attempt::Retry {
                    reason: format!("error connecting to runner: {err}"),
                    retry_after: None,
                });
            }
            Err(err) => return Err(err).context(format!("error requesting {tool} run")),
        };

        // Classify the response status: retryable, fatal or successful
        let status = response.status();
        match status {
            StatusCode::OK => {}
            StatusCode::TOO_MANY_REQUESTS | StatusCode::SERVICE_UNAVAILABLE => {
                let retry_after = response
                    .headers()
                    .get(RETRY_AFTER)
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.trim().parse::<u64>().ok())
                    .map(Duration::from_secs);
                return Ok(Attempt::Retry {
                    reason: format!("runner unavailable (status {status})"),
                    retry_after,
                });
            }
            _ => {
                let body = read_body(response, MAX_ERROR_BODY_BYTES)
                    .await
                    .unwrap_or_default();
                let error = serde_json::from_slice::<ErrorResponse>(&body).map_or_else(
                    |_| String::from_utf8_lossy(&body).trim().to_string(),
                    |e| e.error,
                );
                bail!("unexpected status code from runner running {tool}: {status} - {error}");
            }
        }

        // Read the run output making sure it is for the tool requested
        let body = read_body(response, MAX_OUTPUT_BYTES).await?;
        let output: RunOutput =
            serde_json::from_slice(&body).context("error parsing runner response")?;
        if output.tool != tool {
            bail!(
                "runner returned output for {} instead of {tool}",
                output.tool
            );
        }
        Ok(Attempt::Ok(output))
    }
}

/// Outcome of a single request attempt.
enum Attempt {
    /// The run completed and its output was returned.
    Ok(RunOutput),
    /// The attempt failed in a way that can be retried.
    Retry {
        /// Reason of the failure (reported when giving up).
        reason: String,

        /// Backoff requested by the runner.
        retry_after: Option<Duration>,
    },
}

/// Retry backoff settings.
#[derive(Debug, Clone, Copy)]
struct Backoff {
    /// Base backoff (multiplied by the attempt number).
    base: Duration,
    /// Margin added to the server deadline when checking the retry budget.
    budget_margin: Duration,
    /// Maximum backoff.
    max: Duration,
}

/// Runner error response body.
#[derive(Debug, Deserialize)]
struct ErrorResponse {
    /// Error message.
    error: String,
}

/// Timeouts used for a run (derived from the tool unless overridden).
#[derive(Debug, Clone, Copy)]
struct Timeouts {
    /// Overall client deadline (covers retries).
    client: Duration,
    /// Server side deadline for a single run.
    server: Duration,
}

/// Read a response body enforcing the maximum size provided.
async fn read_body(mut response: reqwest::Response, max_bytes: usize) -> Result<Vec<u8>> {
    // Reject bodies declared larger than the maximum upfront
    if let Some(len) = response.content_length()
        && usize::try_from(len).is_ok_and(|len| len > max_bytes)
    {
        bail!("runner response larger than {max_bytes} bytes");
    }

    // Stream the body enforcing the maximum while reading
    let mut body = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .context("error reading runner response")?
    {
        if body.len() + chunk.len() > max_bytes {
            bail!("runner response larger than {max_bytes} bytes");
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{body_json, header, header_exists, method, path},
    };

    use crate::tools::{AfdocsRequest, ScorecardRequest, Tool};

    use super::*;

    #[tokio::test]
    async fn run_afdocs_success_without_credentials() {
        // Setup runner expecting the request body
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/run/afdocs"))
            .and(body_json(
                json!({"url": "https://docs.example.org/", "max_links": 20}),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(run_output(Tool::Afdocs)))
            .expect(1)
            .mount(&server)
            .await;

        // Run the request providing a token afdocs does not need
        let output = client(&server)
            .run(&afdocs_request(), Some("secret"))
            .await
            .unwrap();

        // Check the output and that the token was never forwarded
        assert_eq!(output.tool, Tool::Afdocs);
        assert_eq!(output.tool_version, "1.2.3");
        assert_eq!(output.output, json!({"ok": true}));
        for request in server.received_requests().await.unwrap() {
            assert!(request.headers.get("authorization").is_none());
            assert!(!String::from_utf8_lossy(&request.body).contains("secret"));
        }
    }

    #[tokio::test]
    async fn run_connect_error_retries() {
        // Setup a client pointing to port 1 (tcpmux), reserved and unused
        let client = RunnerClient::new("http://127.0.0.1:1")
            .unwrap()
            .with_timeouts(Duration::from_secs(5), Duration::from_millis(100))
            .with_backoff(
                Duration::from_millis(10),
                Duration::from_millis(20),
                Duration::from_millis(10),
            );

        // Run the request
        let err = client
            .run(&afdocs_request(), None)
            .await
            .unwrap_err()
            .to_string();

        // Check the connection error was retried before giving up
        assert!(
            err.contains("error connecting") && err.contains("giving up"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn run_invalid_or_mismatched_payload() {
        // Setup runner replying first with another tool output, then junk
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/run/afdocs"))
            .and(header_exists("content-type"))
            .respond_with(ResponseTemplate::new(200).set_body_json(run_output(Tool::Scorecard)))
            .up_to_n_times(1)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/run/afdocs"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&server)
            .await;

        // Check the mismatched tool is rejected
        let err = client(&server)
            .run(&afdocs_request(), None)
            .await
            .unwrap_err()
            .to_string();
        assert!(err.contains("instead of afdocs"), "{err}");

        // Check the invalid payload is rejected
        let err = client(&server)
            .run(&afdocs_request(), None)
            .await
            .unwrap_err()
            .to_string();
        assert!(err.contains("error parsing runner response"), "{err}");
    }

    #[tokio::test]
    async fn run_no_retry_when_budget_exhausted() {
        // Setup runner always unavailable, expecting a single attempt
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/run/afdocs"))
            .respond_with(ResponseTemplate::new(503))
            .expect(1)
            .mount(&server)
            .await;

        // Run the request with a budget too small for a retry
        let err = client(&server)
            .with_timeouts(Duration::from_millis(400), Duration::from_millis(500))
            .run(&afdocs_request(), None)
            .await
            .unwrap_err()
            .to_string();

        // Check the retry was skipped
        assert!(err.contains("not enough time left to retry"), "{err}");
    }

    #[tokio::test]
    async fn run_oversized_payload() {
        // Setup runner replying with a body over the output cap
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/run/afdocs"))
            .respond_with(
                ResponseTemplate::new(200).set_body_bytes(vec![b' '; MAX_OUTPUT_BYTES + 1]),
            )
            .mount(&server)
            .await;

        // Run the request
        let err = client(&server)
            .run(&afdocs_request(), None)
            .await
            .unwrap_err()
            .to_string();

        // Check the oversized body is rejected
        assert!(err.contains("larger than"), "{err}");
    }

    #[tokio::test]
    async fn run_retries_on_429_then_gives_up() {
        // Setup runner always throttling, expecting the initial attempt plus
        // the maximum number of retries
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/run/afdocs"))
            .respond_with(ResponseTemplate::new(429))
            .expect(3)
            .mount(&server)
            .await;

        // Run the request
        let err = client(&server)
            .run(&afdocs_request(), None)
            .await
            .unwrap_err()
            .to_string();

        // Check the client gave up reporting the status
        assert!(
            err.contains("status 429") && err.contains("giving up"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn run_retries_on_503_then_succeeds() {
        // Setup runner unavailable once, then successful
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/run/afdocs"))
            .respond_with(ResponseTemplate::new(503).insert_header("Retry-After", "0"))
            .up_to_n_times(1)
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/run/afdocs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(run_output(Tool::Afdocs)))
            .expect(1)
            .mount(&server)
            .await;

        // Run the request and check it succeeds after the retry
        client(&server).run(&afdocs_request(), None).await.unwrap();
    }

    #[tokio::test]
    async fn run_scorecard_requires_token() {
        // Run a scorecard request without a token
        let server = MockServer::start().await;
        let err = client(&server)
            .run(&scorecard_request(), None)
            .await
            .unwrap_err()
            .to_string();

        // Check the request is rejected before reaching the runner
        assert!(err.contains("requires a GitHub token"), "{err}");
        assert!(server.received_requests().await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn run_scorecard_sends_bearer_token() {
        // Setup runner expecting the bearer token and the request body
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/run/scorecard"))
            .and(header("authorization", "Bearer secret"))
            .and(body_json(
                json!({"checks": ["Code-Review"], "repo_url": "https://github.com/org/repo"}),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(run_output(Tool::Scorecard)))
            .expect(1)
            .mount(&server)
            .await;

        // Run the request and check the runner expectations are met
        client(&server)
            .run(&scorecard_request(), Some("secret"))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn run_slow_response_hits_client_deadline() {
        // Setup runner replying after the client deadline
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/run/afdocs"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_delay(Duration::from_secs(3))
                    .set_body_json(run_output(Tool::Afdocs)),
            )
            .mount(&server)
            .await;

        // Run the request with a short client deadline
        let err = client(&server)
            .with_timeouts(Duration::from_millis(300), Duration::from_secs(5))
            .run(&afdocs_request(), None)
            .await
            .unwrap_err()
            .to_string();

        // Check the client deadline is reported
        assert!(err.contains("timed out"), "{err}");
    }

    #[tokio::test]
    async fn run_unexpected_status_is_fatal() {
        // Setup runner failing with a non-retryable status, expecting a
        // single attempt
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/run/afdocs"))
            .respond_with(
                ResponseTemplate::new(504).set_body_json(json!({"error": "deadline exceeded"})),
            )
            .expect(1)
            .mount(&server)
            .await;

        // Run the request
        let err = client(&server)
            .run(&afdocs_request(), None)
            .await
            .unwrap_err()
            .to_string();

        // Check the status and the error body are reported
        assert!(
            err.contains("504") && err.contains("deadline exceeded"),
            "{err}"
        );
    }

    // Helpers.

    /// Build a valid afdocs request.
    fn afdocs_request() -> ToolRequest {
        ToolRequest::Afdocs(AfdocsRequest {
            url: "https://docs.example.org/".to_string(),
            max_links: 20,
        })
    }

    /// Build a client for the mock server provided with short timeouts and
    /// backoff.
    fn client(server: &MockServer) -> RunnerClient {
        RunnerClient::new(&server.uri())
            .unwrap()
            .with_timeouts(Duration::from_secs(5), Duration::from_millis(500))
            .with_backoff(
                Duration::from_millis(50),
                Duration::from_millis(100),
                Duration::from_millis(100),
            )
    }

    /// Build a run output response body for the tool provided.
    fn run_output(tool: Tool) -> serde_json::Value {
        json!({
            "duration_ms": 42,
            "output": {"ok": true},
            "tool": tool.id(),
            "tool_version": "1.2.3",
        })
    }

    /// Build a valid scorecard request.
    fn scorecard_request() -> ToolRequest {
        ToolRequest::Scorecard(ScorecardRequest {
            checks: vec!["Code-Review".to_string()],
            repo_url: "https://github.com/org/repo".to_string(),
        })
    }
}
