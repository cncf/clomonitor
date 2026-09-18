use anyhow::{Context, Result, format_err};
use serde::{Deserialize, Serialize};

use crate::tools::Tool;

use super::{
    CheckSet, LinterInput, ToolMode,
    checks::{
        CHECKS,
        util::helpers::{find_exemption, should_skip_check},
    },
    datasource::{
        afdocs::{self, AfdocsReport, AfdocsTarget, AfdocsTargetSource},
        github,
        scorecard::{Scorecard, scorecard},
        security_insights::SecurityInsights,
    },
    metadata::{Exemption, METADATA_FILE, Metadata},
};

/// Type alias to represent a check identifier.
pub type CheckId = &'static str;

/// Check configuration.
pub(crate) struct CheckConfig {
    pub weight: usize,
    pub check_sets: Vec<CheckSet>,
    pub datasource: Option<Datasource>,
}

/// Input used by checks to perform their operations.
#[derive(Debug)]
pub(crate) struct CheckInput<'a> {
    pub li: &'a LinterInput,
    pub cm_md: Option<Metadata>,
    pub gh_md: github::md::MdRepository,
    pub afdocs: Option<Result<AfdocsReport>>,
    pub scorecard: Result<Scorecard>,
    pub security_insights: Result<Option<SecurityInsights>>,
}

impl CheckInput<'_> {
    pub(crate) async fn new(li: &LinterInput) -> Result<CheckInput<'_>> {
        // Get CLOMonitor metadata
        let cm_md = Metadata::from(li.root.join(METADATA_FILE))?;

        // Check if required external tools are available (local mode only)
        let scorecard_needed = datasource_needed(li, cm_md.as_ref(), |ds| {
            matches!(ds, Datasource::Scorecard { .. })
        });
        if scorecard_needed && li.tools.scorecard == ToolMode::Local {
            Tool::Scorecard.locate()?;
        }

        // Get GitHub metadata. This must complete before fetching the scorecard:
        // both use the GitHub token, and using it concurrently may trigger
        // GitHub secondary rate limits.
        let gh_md = github::metadata(&li.url, &li.github_token).await?;

        // Resolve the AFDocs target (only when an AFDocs backed check will run)
        let afdocs_target = if datasource_needed(li, cm_md.as_ref(), |ds| {
            matches!(ds, Datasource::Afdocs { .. })
        }) {
            afdocs_target(cm_md.as_ref(), &gh_md)
        } else {
            None
        };

        // Get OpenSSF scorecard and AFDocs report concurrently (AFDocs does not
        // use the GitHub token)
        let scorecard_fut = async {
            if scorecard_needed {
                Box::pin(scorecard(&li.url, &li.github_token, &li.tools.scorecard))
                    .await
                    .context("error getting scorecard")
            } else {
                Err(format_err!(
                    "scorecard not needed for the check sets provided"
                ))
            }
        };
        let afdocs_fut = async {
            match &afdocs_target {
                Some(target) => Some(
                    Box::pin(afdocs::afdocs(target, &li.tools.afdocs))
                        .await
                        .context("error getting AFDocs report"),
                ),
                None => None,
            }
        };
        let (scorecard, afdocs) = tokio::join!(scorecard_fut, afdocs_fut);

        // Get OpenSSF security insights.
        let security_insights = SecurityInsights::new(&li.root);

        // Prepare and return check input
        let ci = CheckInput {
            li,
            cm_md,
            gh_md,
            afdocs,
            scorecard,
            security_insights,
        };
        Ok(ci)
    }
}

/// Check output information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckOutput<T = ()> {
    pub passed: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    pub exempt: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption_reason: Option<String>,

    pub failed: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_reason: Option<String>,
}

impl<T> CheckOutput<T> {
    /// Create a new CheckOutput instance with the passed field set to true.
    #[must_use]
    pub fn passed() -> Self {
        Self {
            passed: true,
            ..Default::default()
        }
    }

    /// Create a new CheckOutput instance with the passed field set to false.
    #[must_use]
    pub fn not_passed() -> Self {
        Self {
            passed: false,
            ..Default::default()
        }
    }

    /// Create a new CheckOutput instance with the exempt field set to true.
    #[must_use]
    pub fn exempt() -> Self {
        Self {
            exempt: true,
            ..Default::default()
        }
    }

    /// Create a new CheckOutput instance with the failed field set to true.
    #[must_use]
    pub fn failed() -> Self {
        Self {
            failed: true,
            ..Default::default()
        }
    }

    /// Url field setter.
    #[must_use]
    pub fn url(mut self, url: Option<String>) -> CheckOutput<T> {
        self.url = url;
        self
    }

    /// Value field setter.
    #[must_use]
    pub fn value(mut self, value: Option<T>) -> CheckOutput<T> {
        self.value = value;
        self
    }

    /// Details field setter.
    #[must_use]
    pub fn details(mut self, details: Option<String>) -> CheckOutput<T> {
        self.details = details;
        self
    }

    /// Exemption reason field setter.
    #[must_use]
    pub fn exemption_reason(mut self, reason: Option<String>) -> CheckOutput<T> {
        self.exemption_reason = reason;
        self
    }

    /// Fail reason field setter.
    #[must_use]
    pub fn fail_reason(mut self, reason: Option<String>) -> CheckOutput<T> {
        self.fail_reason = reason;
        self
    }
}

impl<T> Default for CheckOutput<T> {
    fn default() -> Self {
        Self {
            passed: false,
            url: None,
            value: None,
            details: None,
            exempt: false,
            exemption_reason: None,
            failed: false,
            fail_reason: None,
        }
    }
}

impl<T> From<Exemption> for CheckOutput<T> {
    fn from(exemption: Exemption) -> Self {
        Self::exempt().exemption_reason(Some(exemption.reason))
    }
}

/// External datasource a check relies on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Datasource {
    /// AFDocs category the check maps to.
    Afdocs { category: &'static str },
    /// OpenSSF Scorecard check name.
    Scorecard { name: String },
}

impl Datasource {
    /// Return the AFDocs category identifier when this is an AFDocs datasource.
    pub(crate) fn afdocs_category(&self) -> Option<&'static str> {
        match self {
            Self::Afdocs { category } => Some(category),
            Self::Scorecard { .. } => None,
        }
    }

    /// Return the scorecard check name when this is a scorecard datasource.
    pub(crate) fn scorecard_name(&self) -> Option<&str> {
        match self {
            Self::Scorecard { name } => Some(name),
            Self::Afdocs { .. } => None,
        }
    }
}

/// Check if any of the checks backed by a datasource matching the predicate
/// provided is enabled for the check sets requested and not exempt.
pub(crate) fn datasource_needed(
    li: &LinterInput,
    cm_md: Option<&Metadata>,
    matches_datasource: impl Fn(&Datasource) -> bool,
) -> bool {
    CHECKS.iter().any(|(check_id, config)| {
        config.datasource.as_ref().is_some_and(&matches_datasource)
            && !should_skip_check(check_id, &li.check_sets)
            && find_exemption(check_id, cm_md).is_none()
    })
}

/// Resolve the website AFDocs should analyse: the `agentReadiness.url`
/// override in the CLOMonitor metadata file or the repository homepage url in
/// GitHub.
pub(crate) fn afdocs_target(
    cm_md: Option<&Metadata>,
    gh_md: &github::md::MdRepository,
) -> Option<AfdocsTarget> {
    let non_empty = |url: &String| (!url.trim().is_empty()).then(|| url.trim().to_string());

    // Prefer the metadata override when set
    if let Some(url) = cm_md
        .and_then(|md| md.agent_readiness.as_ref())
        .and_then(|ar| ar.url.as_ref())
        .and_then(non_empty)
    {
        return Some(AfdocsTarget {
            source: AfdocsTargetSource::Metadata,
            url,
        });
    }

    // Fall back to the repository homepage configured in GitHub
    gh_md
        .homepage_url
        .as_ref()
        .and_then(non_empty)
        .map(|url| AfdocsTarget {
            source: AfdocsTargetSource::GithubHomepage,
            url,
        })
}

/// Wrapper macro that takes care of running some common pre-check operations
/// and the synchronous check function.
macro_rules! run {
    ($check:ident, $input:expr) => {
        (|| {
            // Check if this check should be skipped
            if should_skip_check($check::ID, &$input.li.check_sets) {
                return None;
            }

            // Check if an exemption has been declared for this check
            if let Some(exemption) = find_exemption($check::ID, $input.cm_md.as_ref()) {
                return Some(CheckOutput::from(exemption));
            }

            // Call sync check function and wrap returned check output in an option
            let output = match $check::check($input) {
                Ok(output) => output,
                Err(err) => CheckOutput::failed().fail_reason(Some(format!("{:#}", err))),
            };
            Some(output)
        })()
    };
}
pub(crate) use run;

/// Wrapper macro that takes care of running some common pre-check operations
/// and the asynchronous check function.
macro_rules! run_async {
    ($check:ident, $input:expr) => {
        async {
            // Check if this check should be skipped
            if should_skip_check($check::ID, &$input.li.check_sets) {
                return None;
            }

            // Check if an exemption has been declared for this check
            if let Some(exemption) = find_exemption($check::ID, $input.cm_md.as_ref()) {
                return Some(CheckOutput::from(exemption));
            }

            // Call async check function and wrap returned check output in an option
            let output = match $check::check($input).await {
                Ok(output) => output,
                Err(err) => CheckOutput::failed().fail_reason(Some(format!("{:#}", err))),
            };
            Some(output)
        }
    };
}
pub(crate) use run_async;

#[cfg(test)]
mod tests {
    use crate::linter::{
        checks::{code_review, content_discoverability},
        datasource::github::md::MdRepository,
        metadata::AgentReadiness,
    };

    use super::*;

    #[test]
    fn check_output_from_exemption() {
        let exemption = Exemption {
            check: "test".to_string(),
            reason: "test".to_string(),
        };

        assert_eq!(
            CheckOutput::<()>::from(exemption),
            CheckOutput {
                exempt: true,
                exemption_reason: Some("test".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn datasource_needed_depends_on_check_sets() {
        let is_scorecard = |ds: &Datasource| matches!(ds, Datasource::Scorecard { .. });
        let is_afdocs = |ds: &Datasource| matches!(ds, Datasource::Afdocs { .. });

        let code = LinterInput {
            check_sets: vec![CheckSet::Code],
            ..LinterInput::default()
        };
        assert!(datasource_needed(&code, None, is_scorecard));
        assert!(!datasource_needed(&code, None, is_afdocs));

        let community = LinterInput {
            check_sets: vec![CheckSet::Community],
            ..LinterInput::default()
        };
        assert!(!datasource_needed(&community, None, is_scorecard));
        assert!(datasource_needed(&community, None, is_afdocs));

        let none = LinterInput::default();
        assert!(!datasource_needed(&none, None, is_scorecard));
        assert!(!datasource_needed(&none, None, is_afdocs));
    }

    #[test]
    fn datasource_needed_ignores_exempt_checks() {
        let is_afdocs = |ds: &Datasource| matches!(ds, Datasource::Afdocs { .. });
        let community = LinterInput {
            check_sets: vec![CheckSet::Community],
            ..LinterInput::default()
        };

        // Exempting a single AFDocs check keeps the datasource needed
        let md = Metadata {
            exemptions: Some(vec![Exemption {
                check: content_discoverability::ID.to_string(),
                reason: "reason".to_string(),
            }]),
            ..Metadata::default()
        };
        assert!(datasource_needed(&community, Some(&md), is_afdocs));

        // Exempting all AFDocs checks makes the datasource unneeded
        let md = Metadata {
            exemptions: Some(
                CHECKS
                    .iter()
                    .filter(|(_, c)| c.datasource.as_ref().is_some_and(is_afdocs))
                    .map(|(id, _)| Exemption {
                        check: (*id).to_string(),
                        reason: "reason".to_string(),
                    })
                    .collect(),
            ),
            ..Metadata::default()
        };
        assert!(!datasource_needed(&community, Some(&md), is_afdocs));

        // Exempting a scorecard check does not affect AFDocs
        let code = LinterInput {
            check_sets: vec![CheckSet::Code],
            ..LinterInput::default()
        };
        let md = Metadata {
            exemptions: Some(vec![Exemption {
                check: code_review::ID.to_string(),
                reason: "reason".to_string(),
            }]),
            ..Metadata::default()
        };
        assert!(datasource_needed(&code, Some(&md), |ds| matches!(
            ds,
            Datasource::Scorecard { .. }
        )));
    }

    #[test]
    fn afdocs_target_prefers_metadata_override() {
        let md = Metadata {
            agent_readiness: Some(AgentReadiness {
                url: Some(" https://docs.example.org/ ".to_string()),
            }),
            ..Metadata::default()
        };
        let gh_md = MdRepository {
            homepage_url: Some("https://example.org".to_string()),
            ..MdRepository::default()
        };
        assert_eq!(
            afdocs_target(Some(&md), &gh_md),
            Some(AfdocsTarget {
                source: AfdocsTargetSource::Metadata,
                url: "https://docs.example.org/".to_string(),
            })
        );
    }

    #[test]
    fn afdocs_target_falls_back_to_homepage() {
        let md = Metadata {
            agent_readiness: Some(AgentReadiness {
                url: Some(String::new()),
            }),
            ..Metadata::default()
        };
        let gh_md = MdRepository {
            homepage_url: Some("https://example.org".to_string()),
            ..MdRepository::default()
        };
        let expected = Some(AfdocsTarget {
            source: AfdocsTargetSource::GithubHomepage,
            url: "https://example.org".to_string(),
        });
        assert_eq!(afdocs_target(Some(&md), &gh_md), expected);
        assert_eq!(afdocs_target(None, &gh_md), expected);
    }

    #[test]
    fn afdocs_target_none_when_unavailable() {
        let gh_md = MdRepository {
            homepage_url: Some("  ".to_string()),
            ..MdRepository::default()
        };
        assert_eq!(afdocs_target(None, &gh_md), None);
        assert_eq!(afdocs_target(None, &MdRepository::default()), None);
    }
}
