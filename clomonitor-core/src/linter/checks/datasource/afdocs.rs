//! AFDocs datasource (<https://afdocs.dev>).
//!
//! AFDocs analyses a documentation website and reports how ready it is to be
//! consumed by AI agents. CLOMonitor runs it once per eligible repository and
//! maps each of its seven check categories to an agent readiness check. How
//! the report is obtained depends on the [`ToolMode`] configured: running the
//! `afdocs` binary found in PATH (`Local`, used by the linter CLI), asking
//! `clomonitor-runner` to run it (`Runner`, used by the tracker) or not at all
//! (`Disabled`, checks are reported as failed). Both transports build the same
//! [`AfdocsRequest`] and produce the same JSON, so the resulting report only
//! differs in the transport recorded on it.

use std::{
    collections::{HashMap, HashSet},
    fmt::Write as _,
};

use anyhow::{Error, Result, bail, format_err};
use serde::Deserialize;

use crate::{
    linter::{
        ToolMode,
        check::{CheckOutput, Datasource},
        checks::CHECKS,
    },
    tools::{
        self, AFDOCS_MAX_LINKS, AfdocsRequest, LocalTool, RunOutput, Tool, ToolRequest,
        runner::RunnerClient,
    },
};

/// Minimum AFDocs category score required for a check to pass.
pub(crate) const AFDOCS_PASS_THRESHOLD: f64 = 70.0;

/// Details shown when no website url could be found for the repository.
const NO_TARGET_DETAILS: &str = "# Agent readiness

No website URL found for this repository, so the AFDocs analysis could not be run.

Set the repository homepage in GitHub or add an `agentReadiness.url` entry to the `.clomonitor.yml` metadata file (see <https://clomonitor.io/docs/topics/checks/#agent-readiness>) to point at the documentation site that should be analysed.";

/// Valid AFDocs check result statuses.
const STATUSES: [&str; 5] = ["pass", "warn", "fail", "skip", "error"];

// AFDocs category identifiers.
pub(crate) const AUTHENTICATION: &str = "authentication";
pub(crate) const CONTENT_DISCOVERABILITY: &str = "content-discoverability";
pub(crate) const CONTENT_STRUCTURE: &str = "content-structure";
pub(crate) const MARKDOWN_AVAILABILITY: &str = "markdown-availability";
pub(crate) const OBSERVABILITY: &str = "observability";
pub(crate) const PAGE_SIZE: &str = "page-size";
pub(crate) const URL_STABILITY: &str = "url-stability";

/// AFDocs categories and the checks they contain (AFDocs 0.20).
pub(crate) const CATEGORIES: [AfdocsCategory; 7] = [
    AfdocsCategory {
        checks: &[
            "llms-txt-exists",
            "llms-txt-valid",
            "llms-txt-size",
            "llms-txt-links-resolve",
            "llms-txt-links-markdown",
            "llms-txt-directive-html",
            "llms-txt-directive-md",
        ],
        id: CONTENT_DISCOVERABILITY,
        name: "Content Discoverability",
    },
    AfdocsCategory {
        checks: &["markdown-url-support", "content-negotiation"],
        id: MARKDOWN_AVAILABILITY,
        name: "Markdown Availability",
    },
    AfdocsCategory {
        checks: &[
            "rendering-strategy",
            "page-size-markdown",
            "page-size-html",
            "content-start-position",
        ],
        id: PAGE_SIZE,
        name: "Page Size and Truncation Risk",
    },
    AfdocsCategory {
        checks: &[
            "tabbed-content-serialization",
            "section-header-quality",
            "markdown-code-fence-validity",
        ],
        id: CONTENT_STRUCTURE,
        name: "Content Structure",
    },
    AfdocsCategory {
        checks: &["http-status-codes", "redirect-behavior"],
        id: URL_STABILITY,
        name: "URL Stability and Redirects",
    },
    AfdocsCategory {
        checks: &[
            "llms-txt-coverage",
            "markdown-content-parity",
            "cache-header-hygiene",
        ],
        id: OBSERVABILITY,
        name: "Observability and Content Health",
    },
    AfdocsCategory {
        checks: &["auth-gate-detection", "auth-alternative-access"],
        id: AUTHENTICATION,
        name: "Authentication and Access",
    },
];

/// AFDocs report (`afdocs check <url> --format json --score`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AfdocsReport {
    /// Results of every check run.
    pub results: Vec<AfdocsResult>,
    /// Time the analysis was run at (RFC 3339).
    pub timestamp: String,
    /// Website url analysed.
    pub url: String,

    /// Version of the AFDocs tool that produced the report.
    #[serde(default)]
    pub afdocs_version: Option<String>,
    /// Page sampling strategy used.
    #[serde(default)]
    pub sampling_strategy: Option<String>,
    /// Category scores and suggested fixes.
    #[serde(default)]
    pub scoring: Option<AfdocsScoring>,
    /// Where the target url came from (set by CLOMonitor, not by AFDocs).
    #[serde(default, skip_deserializing)]
    pub target_source: Option<AfdocsTargetSource>,
    /// Number of pages tested.
    #[serde(default)]
    pub tested_pages: Option<u64>,
    /// Transport used to obtain the report (set by CLOMonitor, not by AFDocs).
    #[serde(default, skip_deserializing)]
    pub transport: AfdocsTransport,
}

impl AfdocsReport {
    /// Build a validated AFDocs report from the output of a tool run.
    pub(crate) fn from_run_output(run: RunOutput, transport: AfdocsTransport) -> Result<Self> {
        if run.tool != Tool::Afdocs {
            bail!("unexpected tool output: {}", run.tool);
        }
        let mut report = Self::from_value(run.output)?;
        report.afdocs_version = Some(run.tool_version);
        report.transport = transport;
        Ok(report)
    }

    /// Build and validate an AFDocs report from its JSON value.
    pub(crate) fn from_value(value: serde_json::Value) -> Result<Self> {
        let report: Self = serde_json::from_value(value)
            .map_err(|err| format_err!("AFDocs output contract violation: invalid JSON ({err})"))?;
        report.validate()?;
        Ok(report)
    }

    /// Return the results that belong to the category provided.
    fn results_in(&self, category_id: &str) -> impl Iterator<Item = &AfdocsResult> {
        self.results
            .iter()
            .filter(move |r| r.category == category_id)
    }

    /// Describe how many pages were tested, for messages explaining that a
    /// category could not be scored.
    fn tested_pages_summary(&self) -> String {
        match self.tested_pages {
            Some(1) => "only 1 page was tested".to_string(),
            Some(n) => format!("only {n} pages were tested"),
            None => "too few pages were tested".to_string(),
        }
    }

    /// Validate the report against the expected AFDocs contract.
    fn validate(&self) -> Result<()> {
        let violation = |msg: String| format_err!("AFDocs output contract violation: {msg}");

        // Scoring must be present and include exactly the known categories
        let Some(scoring) = &self.scoring else {
            return Err(violation("scoring information missing".to_string()));
        };
        let expected: HashSet<&str> = CATEGORIES.iter().map(|c| c.id).collect();
        let found: HashSet<&str> = scoring.category_scores.keys().map(String::as_str).collect();
        if expected != found {
            let mut missing: Vec<_> = expected.difference(&found).copied().collect();
            let mut unexpected: Vec<_> = found.difference(&expected).copied().collect();
            missing.sort_unstable();
            unexpected.sort_unstable();
            return Err(violation(format!(
                "unexpected category scores (missing: [{}], unexpected: [{}])",
                missing.join(", "),
                unexpected.join(", ")
            )));
        }
        for (id, cs) in &scoring.category_scores {
            if let Some(score) = cs.score
                && !(0.0..=100.0).contains(&score)
            {
                return Err(violation(format!(
                    "category {id} score {score} out of range (0-100)"
                )));
            }
        }

        // Results must use known statuses and categories
        for r in &self.results {
            if !STATUSES.contains(&r.status.as_str()) {
                return Err(violation(format!(
                    "unknown status {:?} in check {}",
                    r.status, r.id
                )));
            }
            if category(&r.category).is_none() {
                return Err(violation(format!(
                    "unknown category {:?} in check {}",
                    r.category, r.id
                )));
            }
        }

        // Every expected check must be present exactly once in its category
        for c in &CATEGORIES {
            for check_id in c.checks {
                let occurrences: Vec<&AfdocsResult> =
                    self.results.iter().filter(|r| r.id == *check_id).collect();
                match occurrences.as_slice() {
                    [] => return Err(violation(format!("check {check_id} missing"))),
                    [r] if r.category != c.id => {
                        return Err(violation(format!(
                            "check {check_id} reported in category {:?} (expected {:?})",
                            r.category, c.id
                        )));
                    }
                    [_] => {}
                    _ => {
                        return Err(violation(format!(
                            "check {check_id} reported more than once"
                        )));
                    }
                }
            }
        }

        Ok(())
    }
}

/// AFDocs check category.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AfdocsCategory {
    /// Identifiers of the AFDocs checks in the category.
    pub checks: &'static [&'static str],
    /// Category identifier.
    pub id: &'static str,
    /// Human readable category name.
    pub name: &'static str,
}

impl AfdocsCategory {
    /// Return the AFDocs documentation url for this category.
    pub(crate) fn docs_url(&self) -> String {
        format!("https://afdocs.dev/checks/{}", self.id)
    }
}

/// AFDocs category result for a check.
#[derive(Debug)]
pub(crate) enum AfdocsCategoryResult<'a> {
    /// The category results in a valid AFDocs report.
    Available {
        /// Category the check is backed by.
        category: &'static AfdocsCategory,
        /// Report the category was taken from.
        report: &'a AfdocsReport,
        /// Score of the category in the report.
        score: AfdocsCategoryScore,
    },
    /// The AFDocs report could not be obtained.
    Failed(&'a Error),
    /// No website url is available for the repository.
    NoTarget,
}

/// AFDocs category score (null when there was not enough data).
#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct AfdocsCategoryScore {
    /// Letter grade derived from the score.
    pub grade: Option<String>,
    /// Score in the 0-100 range.
    pub score: Option<f64>,
}

/// AFDocs check result.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AfdocsResult {
    /// Identifier of the category the check belongs to.
    pub category: String,
    /// Check identifier.
    pub id: String,
    /// Result status (pass, warn, fail, skip or error).
    pub status: String,

    /// Human readable result message.
    #[serde(default)]
    pub message: String,
}

/// AFDocs scoring information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AfdocsScoring {
    /// Scores indexed by category identifier.
    pub category_scores: HashMap<String, AfdocsCategoryScore>,

    /// Suggested fixes indexed by check identifier.
    #[serde(default)]
    pub resolutions: HashMap<String, String>,
}

/// Website analysed by AFDocs for a repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AfdocsTarget {
    /// Where the url was obtained from.
    pub source: AfdocsTargetSource,
    /// Website url.
    pub url: String,
}

/// Origin of the website url analysed by AFDocs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AfdocsTargetSource {
    /// Repository homepage configured in GitHub.
    GithubHomepage,
    /// `agentReadiness.url` entry in the `.clomonitor.yml` metadata file.
    Metadata,
}

impl AfdocsTargetSource {
    /// Plain text explanation of the source, including how to override it.
    fn hint(self) -> &'static str {
        match self {
            Self::GithubHomepage => {
                "The target is the repository homepage configured in GitHub; set agentReadiness.url in the .clomonitor.yml metadata file to analyse a different site."
            }
            Self::Metadata => {
                "The target is the agentReadiness.url entry in the .clomonitor.yml metadata file."
            }
        }
    }

    /// Markdown explanation of the source, including how to override it.
    fn hint_md(self) -> &'static str {
        match self {
            Self::GithubHomepage => {
                "from the repository homepage configured in GitHub; set `agentReadiness.url` in the `.clomonitor.yml` metadata file to analyse a different site"
            }
            Self::Metadata => "from `agentReadiness.url` in the `.clomonitor.yml` metadata file",
        }
    }
}

/// Transport used to obtain an AFDocs report.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum AfdocsTransport {
    /// The `afdocs` binary found in PATH.
    Local,
    /// The CLOMonitor runner service.
    Runner,
    /// The transport was not recorded.
    #[default]
    Unknown,
}

impl AfdocsTransport {
    /// Human readable transport name used in the checks details.
    fn label(self) -> &'static str {
        match self {
            Self::Local => "local binary",
            Self::Runner => "CLOMonitor runner",
            Self::Unknown => "unknown transport",
        }
    }
}

impl<T> From<AfdocsCategoryResult<'_>> for CheckOutput<T> {
    fn from(result: AfdocsCategoryResult<'_>) -> Self {
        match result {
            AfdocsCategoryResult::Available {
                category,
                report,
                score,
            } => {
                // A null score means the site could not be analysed, which is
                // reported as a failure (not as a low score)
                let Some(value) = score.score else {
                    return CheckOutput::failed()
                        .url(Some(report.url.clone()))
                        .details(Some(details(report, category, &score)))
                        .fail_reason(Some(insufficient_data_reason(report, category)));
                };

                // Compare the category score against the pass threshold
                let mut output = if value >= AFDOCS_PASS_THRESHOLD {
                    CheckOutput::passed()
                } else {
                    CheckOutput::not_passed()
                };
                output.url = Some(report.url.clone());
                output.details = Some(details(report, category, &score));
                output
            }
            AfdocsCategoryResult::Failed(err) => {
                CheckOutput::failed().fail_reason(Some(format!("{err:#}")))
            }
            AfdocsCategoryResult::NoTarget => {
                CheckOutput::not_passed().details(Some(NO_TARGET_DETAILS.to_string()))
            }
        }
    }
}

/// Get the AFDocs report for the target provided using the mode requested.
pub(crate) async fn afdocs(target: &AfdocsTarget, mode: &ToolMode) -> Result<AfdocsReport> {
    // Run the tool using the transport configured
    let request = ToolRequest::Afdocs(AfdocsRequest {
        url: target.url.clone(),
        max_links: AFDOCS_MAX_LINKS,
    });
    let mut report = match mode {
        ToolMode::Disabled => return Err(format_err!("AFDocs is not configured")),
        ToolMode::Local => {
            let tool = LocalTool::locate(Tool::Afdocs).await?;
            let run = tools::run_local(&request, &tool, None, Tool::Afdocs.deadline()).await?;
            AfdocsReport::from_run_output(run, AfdocsTransport::Local)?
        }
        ToolMode::Runner { url } => {
            let run = RunnerClient::new(url)?.run(&request, None).await?;
            AfdocsReport::from_run_output(run, AfdocsTransport::Runner)?
        }
    };

    // Record where the target url came from so checks can explain it
    report.target_source = Some(target.source);
    Ok(report)
}

/// Return the category with the id provided, if known.
pub(crate) fn category(id: &str) -> Option<&'static AfdocsCategory> {
    CATEGORIES.iter().find(|c| c.id == id)
}

/// Get the AFDocs category result backing the check provided.
pub(crate) fn get_category<'a>(
    afdocs: Option<&'a Result<AfdocsReport>>,
    check_id: &str,
) -> AfdocsCategoryResult<'a> {
    let category_id = CHECKS[check_id]
        .datasource
        .as_ref()
        .and_then(Datasource::afdocs_category)
        .expect("check to be backed by afdocs");
    let category = category(category_id).expect("afdocs category to be defined");
    match afdocs {
        None => AfdocsCategoryResult::NoTarget,
        Some(Err(err)) => AfdocsCategoryResult::Failed(err),
        Some(Ok(report)) => {
            let score = report
                .scoring
                .as_ref()
                .and_then(|s| s.category_scores.get(category.id))
                .cloned()
                .unwrap_or_default();
            AfdocsCategoryResult::Available {
                category,
                report,
                score,
            }
        }
    }
}

// Helpers.

/// Sanitize an identifier so it can be safely rendered inside inline code.
fn code(text: &str) -> String {
    text.chars()
        .filter(|c| !matches!(c, '`' | '\n' | '\r'))
        .collect()
}

/// Build the markdown details for a category result.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn details(
    report: &AfdocsReport,
    category: &AfdocsCategory,
    score: &AfdocsCategoryScore,
) -> String {
    let mut md = format!("# {} AFDocs category\n\n", category.name);

    // Target
    let source = report
        .target_source
        .map(|s| format!(" ({})", s.hint_md()))
        .unwrap_or_default();
    let _ = writeln!(md, "**Target**: {}{source}\n", escape_md(&report.url));

    // Score
    match score.score {
        Some(value) => {
            let grade = score
                .grade
                .as_deref()
                .map(|g| format!(" ({})", escape_md(g)))
                .unwrap_or_default();
            let _ = writeln!(
                md,
                "**Score**: {}/100{grade} (check passes with score >= {})\n",
                value.round() as u64,
                AFDOCS_PASS_THRESHOLD as u64
            );
        }
        None => {
            let _ = writeln!(
                md,
                "**Score**: n/a (not enough data: {}, so the site could not be analysed and the check is reported as failed)\n",
                report.tested_pages_summary()
            );
        }
    }

    // Errors
    let results: Vec<&AfdocsResult> = report.results_in(category.id).collect();
    let errors = results.iter().filter(|r| r.status == "error").count();
    if errors > 0 {
        let _ = writeln!(
            md,
            "**Warning**: {errors} check(s) errored during collection and were excluded from the category score (see below).\n"
        );
    }

    // Checks
    md.push_str("**Checks**:\n\n");
    for r in &results {
        let _ = writeln!(
            md,
            "- `{}` `{}`: {}",
            r.status.to_uppercase(),
            code(&r.id),
            escape_md(&r.message)
        );
    }
    md.push('\n');

    // Suggested fixes
    if let Some(scoring) = &report.scoring {
        let mut fixes: Vec<(&String, &String)> = results
            .iter()
            .filter_map(|r| scoring.resolutions.get_key_value(&r.id))
            .collect();
        fixes.sort_unstable();
        if !fixes.is_empty() {
            md.push_str("**Suggested fixes**:\n\n");
            for (id, fix) in fixes {
                let _ = writeln!(md, "- `{}`: {}", code(id), escape_md(fix));
            }
            md.push('\n');
        }
    }

    // Run information
    let _ = writeln!(
        md,
        "**Run**: AFDocs {}, {}, {} sampling, max {} pages, {} pages tested, {}\n",
        escape_md(
            report
                .afdocs_version
                .as_deref()
                .unwrap_or("unknown version")
        ),
        report.transport.label(),
        escape_md(
            report
                .sampling_strategy
                .as_deref()
                .unwrap_or("deterministic")
        ),
        AFDOCS_MAX_LINKS,
        report
            .tested_pages
            .map_or_else(|| "n/a".to_string(), |p| p.to_string()),
        escape_md(&report.timestamp),
    );

    let _ = write!(
        md,
        "**Please see the [category documentation]({}) in afdocs.dev for more details**",
        category.docs_url()
    );
    md
}

/// Escape markdown special characters in upstream text and collapse
/// whitespace so it cannot alter the details structure.
fn escape_md(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for c in text
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
    {
        if matches!(
            c,
            '\\' | '`' | '*' | '_' | '[' | ']' | '<' | '>' | '~' | '|' | '#'
        ) {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}

/// Build the plain text reason shown when AFDocs did not have enough data to
/// score the category provided.
fn insufficient_data_reason(report: &AfdocsReport, category: &AfdocsCategory) -> String {
    let mut reason = format!(
        "AFDocs could not score {} for {}: {}, so there is not enough data (the site may be rendered client-side, the target may not be a documentation site, or it may be unreachable).",
        category.name,
        report.url,
        report.tested_pages_summary()
    );
    if let Some(source) = report.target_source {
        reason.push(' ');
        reason.push_str(source.hint());
    }
    reason
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::linter::checks::{
        authentication, content_discoverability, content_structure, markdown_availability,
        observability, page_size, url_stability,
    };

    use super::*;

    const TESTDATA_PATH: &str = "src/testdata/afdocs";

    fn fixture(name: &str) -> Vec<u8> {
        fs::read(format!("{TESTDATA_PATH}/{name}.json")).unwrap()
    }

    /// Parse a fixture like the datasource does (JSON value, then contract).
    fn parse(data: &[u8]) -> Result<AfdocsReport> {
        let value: serde_json::Value = serde_json::from_slice(data)
            .map_err(|err| format_err!("AFDocs output contract violation: invalid JSON ({err})"))?;
        AfdocsReport::from_value(value)
    }

    fn report() -> AfdocsReport {
        let mut report = parse(&fixture("report")).unwrap();
        report.target_source = Some(AfdocsTargetSource::GithubHomepage);
        report.transport = AfdocsTransport::Runner;
        report
    }

    #[test]
    fn categories_are_consistent_with_checks_registry() {
        let mut registered: Vec<&str> = CHECKS
            .values()
            .filter_map(|c| c.datasource.as_ref().and_then(Datasource::afdocs_category))
            .collect();
        registered.sort_unstable();
        let mut known: Vec<&str> = CATEGORIES.iter().map(|c| c.id).collect();
        known.sort_unstable();
        assert_eq!(registered, known);
        assert_eq!(
            category(CONTENT_DISCOVERABILITY).unwrap().docs_url(),
            "https://afdocs.dev/checks/content-discoverability"
        );
    }

    #[test]
    fn parse_valid_report() {
        let report = report();
        assert_eq!(report.url, "https://docs.example.org/");
        assert_eq!(report.tested_pages, Some(3));
        assert_eq!(report.afdocs_version.as_deref(), Some("0.20.0"));
        assert_eq!(report.results.len(), 23);
    }

    #[test]
    fn parse_real_afdocs_report() {
        // Report produced by afdocs 0.20.0 against https://clomonitor.io/docs/
        let report = parse(&fixture("real-clomonitor-io")).unwrap();
        assert_eq!(report.results.len(), 23);
        assert_eq!(report.sampling_strategy.as_deref(), Some("deterministic"));
        let afdocs = Some(Ok(report));
        let output: CheckOutput = get_category(afdocs.as_ref(), content_discoverability::ID).into();
        assert!(!output.passed && !output.failed);
        let details = output.details.unwrap();
        assert!(details.contains("**Target**: https://clomonitor.io/docs/\n"));
        assert!(details.contains("**Score**: 0/100 (F)"));
        let output: CheckOutput = get_category(afdocs.as_ref(), page_size::ID).into();
        assert!(!output.passed && output.failed);
        assert!(
            output
                .fail_reason
                .unwrap()
                .contains("only 1 page was tested")
        );
        assert!(output.details.unwrap().contains("**Score**: n/a"));
    }

    #[test]
    fn parse_contract_violations() {
        for (name, expected) in [
            ("missing-scoring", "scoring information missing"),
            ("missing-category", "missing: [observability]"),
            ("unknown-status", "unknown status \"unknown\""),
            ("duplicate-ids", "reported more than once"),
            ("missing-check", "check cache-header-hygiene missing"),
            ("score-out-of-range", "out of range"),
            ("invalid", "invalid JSON"),
        ] {
            let err = parse(&fixture(name)).unwrap_err().to_string();
            assert!(
                err.starts_with("AFDocs output contract violation") && err.contains(expected),
                "{name}: {err}"
            );
        }
    }

    #[test]
    fn check_output_passed() {
        let afdocs = Some(Ok(report()));
        let output: CheckOutput = get_category(afdocs.as_ref(), content_discoverability::ID).into();
        assert!(output.passed);
        assert!(!output.failed);
        assert_eq!(output.url.as_deref(), Some("https://docs.example.org/"));
        let details = output.details.unwrap();
        assert!(details.starts_with("# Content Discoverability AFDocs category\n\n"));
        assert!(details.contains(
            "**Target**: https://docs.example.org/ (from the repository homepage configured in GitHub; set `agentReadiness.url` in the `.clomonitor.yml` metadata file to analyse a different site)\n\n**Score**: 82/100 (B) (check passes with score >= 70)"
        ));
        assert!(details.contains("- `PASS` `llms-txt-exists`: llms.txt found at"));
        assert!(details.contains("- `WARN` `llms-txt-size`:"));
        assert!(details.contains("**Suggested fixes**:\n\n- `llms-txt-size`: Split llms.txt"));
        assert!(details.contains(
            "**Run**: AFDocs 0.20.0, CLOMonitor runner, deterministic sampling, max 20 pages, 3 pages tested, 2026-09-14T10:15:30.000Z\n"
        ));
        assert!(details.contains("(https://afdocs.dev/checks/content-discoverability)"));
        assert!(!details.to_lowercase().contains("advisory"));
        assert!(!details.contains("errored during collection"));
    }

    #[test]
    fn check_output_warn_heavy_not_passed() {
        let afdocs = Some(Ok(report()));
        let output: CheckOutput = get_category(afdocs.as_ref(), markdown_availability::ID).into();
        assert!(!output.passed);
        assert!(!output.failed);
        let details = output.details.unwrap();
        assert!(details.contains("**Score**: 45/100 (F)"));
        assert!(details.contains("- `FAIL` `markdown-url-support`"));
    }

    #[test]
    fn check_output_null_score_failed() {
        // Homepage target: the reason explains how to override it
        let afdocs = Some(Ok(report()));
        let output: CheckOutput = get_category(afdocs.as_ref(), page_size::ID).into();
        assert!(!output.passed);
        assert!(output.failed);
        assert_eq!(output.url.as_deref(), Some("https://docs.example.org/"));
        assert_eq!(
            output.fail_reason.as_deref(),
            Some(
                "AFDocs could not score Page Size and Truncation Risk for https://docs.example.org/: only 3 pages were tested, so there is not enough data (the site may be rendered client-side, the target may not be a documentation site, or it may be unreachable). The target is the repository homepage configured in GitHub; set agentReadiness.url in the .clomonitor.yml metadata file to analyse a different site."
            )
        );
        let details = output.details.unwrap();
        assert!(details.contains("**Score**: n/a (not enough data: only 3 pages were tested"));
        assert!(details.contains("reported as failed"));
        assert!(details.contains("- `SKIP` `rendering-strategy`"));

        // Metadata target: the reason points at the metadata entry
        let mut metadata_report = report();
        metadata_report.target_source = Some(AfdocsTargetSource::Metadata);
        let afdocs = Some(Ok(metadata_report));
        let output: CheckOutput = get_category(afdocs.as_ref(), page_size::ID).into();
        assert!(output.failed);
        let reason = output.fail_reason.unwrap();
        assert!(reason.ends_with(
            "The target is the agentReadiness.url entry in the .clomonitor.yml metadata file."
        ));
        assert!(output.details.unwrap().contains(
            "**Target**: https://docs.example.org/ (from `agentReadiness.url` in the `.clomonitor.yml` metadata file)\n"
        ));

        // Unknown target source and page count: no hint, generic wording
        let mut unknown_source_report = report();
        unknown_source_report.target_source = None;
        unknown_source_report.tested_pages = None;
        let afdocs = Some(Ok(unknown_source_report));
        let output: CheckOutput = get_category(afdocs.as_ref(), page_size::ID).into();
        let reason = output.fail_reason.unwrap();
        assert!(
            reason.contains(": too few pages were tested, so"),
            "{reason}"
        );
        assert!(reason.ends_with("or it may be unreachable)."), "{reason}");
        assert!(
            output
                .details
                .unwrap()
                .contains("**Target**: https://docs.example.org/\n\n")
        );
    }

    #[test]
    fn check_output_passed_with_native_errors() {
        let afdocs = Some(Ok(report()));
        let output: CheckOutput = get_category(afdocs.as_ref(), content_structure::ID).into();
        assert!(output.passed);
        assert!(!output.failed);
        let details = output.details.unwrap();
        assert!(details.contains("**Score**: 80/100 (B)"));
        assert!(details.contains("**Warning**: 1 check(s) errored during collection"));
        assert!(
            details.contains("- `ERROR` `section-header-quality`: Unexpected token \\< in JSON")
        );
    }

    #[test]
    fn check_output_not_passed_with_native_errors() {
        let afdocs = Some(Ok(report()));
        let output: CheckOutput = get_category(afdocs.as_ref(), url_stability::ID).into();
        assert!(!output.passed);
        assert!(!output.failed);
        let details = output.details.unwrap();
        assert!(details.contains("**Score**: 40/100 (F)"));
        assert!(details.contains("**Warning**: 1 check(s) errored"));
        assert!(details.contains("- `ERROR` `redirect-behavior`"));
    }

    #[test]
    fn check_output_full_score_with_skips() {
        let afdocs = Some(Ok(report()));
        let output: CheckOutput = get_category(afdocs.as_ref(), authentication::ID).into();
        assert!(output.passed);
        assert!(
            output
                .details
                .unwrap()
                .contains("- `SKIP` `auth-alternative-access`")
        );
        let output: CheckOutput = get_category(afdocs.as_ref(), observability::ID).into();
        assert!(output.passed);
    }

    #[test]
    fn check_output_no_target() {
        let afdocs: Option<Result<AfdocsReport>> = None;
        let output: CheckOutput = get_category(afdocs.as_ref(), page_size::ID).into();
        assert!(!output.passed);
        assert!(!output.failed);
        assert!(output.url.is_none());
        assert!(output.details.unwrap().contains("agentReadiness.url"));
    }

    #[test]
    fn check_output_failed() {
        let afdocs = Some(Err(format_err!("AFDocs is not configured")));
        let output: CheckOutput = get_category(afdocs.as_ref(), page_size::ID).into();
        assert!(!output.passed);
        assert!(output.failed);
        assert_eq!(
            output.fail_reason.as_deref(),
            Some("AFDocs is not configured")
        );
    }

    #[test]
    fn escape_md_escapes_and_collapses() {
        assert_eq!(
            escape_md("a *b* [c](x)\n\n# d `e` <f> ~g~ h|i j_k"),
            "a \\*b\\* \\[c\\](x) \\# d \\`e\\` \\<f\\> \\~g\\~ h\\|i j\\_k"
        );
        assert_eq!(code("id`\n"), "id");
    }

    #[test]
    fn report_from_run_output() {
        let value: serde_json::Value = serde_json::from_slice(&fixture("report")).unwrap();
        let run = RunOutput {
            duration_ms: 10,
            output: value.clone(),
            tool: Tool::Afdocs,
            tool_version: "0.20.5".to_string(),
        };
        let report = AfdocsReport::from_run_output(run, AfdocsTransport::Runner).unwrap();
        assert_eq!(report.afdocs_version.as_deref(), Some("0.20.5"));
        assert_eq!(report.transport, AfdocsTransport::Runner);

        // Wrong tool
        let run = RunOutput {
            duration_ms: 10,
            output: value,
            tool: Tool::Scorecard,
            tool_version: "4.13.0".to_string(),
        };
        assert!(AfdocsReport::from_run_output(run, AfdocsTransport::Runner).is_err());

        // Contract violation
        let run = RunOutput {
            duration_ms: 10,
            output: serde_json::from_slice(&fixture("missing-scoring")).unwrap(),
            tool: Tool::Afdocs,
            tool_version: "0.20.5".to_string(),
        };
        let err = AfdocsReport::from_run_output(run, AfdocsTransport::Runner)
            .unwrap_err()
            .to_string();
        assert!(err.contains("contract violation"), "{err}");
    }

    #[tokio::test]
    async fn afdocs_disabled() {
        let target = AfdocsTarget {
            source: AfdocsTargetSource::Metadata,
            url: "https://docs.example.org/".to_string(),
        };
        let err = afdocs(&target, &ToolMode::Disabled)
            .await
            .unwrap_err()
            .to_string();
        assert_eq!(err, "AFDocs is not configured");
    }
}
