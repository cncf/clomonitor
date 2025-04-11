use crate::linter::util;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// OpenSSF Security Insights manifest file name.
pub(crate) const SECURITY_INSIGHTS_MANIFEST_FILE: &str = "SECURITY-INSIGHTS.yml";

/// OpenSSF Security Insights manifest.
///
/// Note: the types defined below do not contain *all* the fields available in
/// the specification, just the ones needed by CLOMonitor.
///
/// For more details please see the spec documentation:
/// https://github.com/ossf/security-insights-spec/blob/v1.0.0/specification.md
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct SecurityInsights {
    pub contribution_policy: ContributionPolicy,
    pub dependencies: Option<Dependencies>,
    pub distribution_points: Vec<String>,
    pub header: Header,
    pub project_lifecycle: ProjectLifecycle,
    pub security_artifacts: Option<SecurityArtifacts>,
    pub security_contacts: Vec<SecurityContact>,
    pub vulnerability_reporting: VulnerabilityReporting,
}

impl SecurityInsights {
    /// Create a new SecurityInsights instance from the manifest file located
    /// at the path provided.
    pub(crate) fn new(path: &Path) -> Result<Option<Self>> {
        let manifest_path = path.join(SECURITY_INSIGHTS_MANIFEST_FILE);
        if !Path::new(&manifest_path).exists() {
            return Ok(None);
        }
        let content = util::fs::read_to_string(manifest_path)
            .context("error reading security insights manifest file")?;
        serde_yaml::from_str(&content).context("invalid security insights manifest")
    }
}

/// Project's contribution rules, requirements, and policies.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct ContributionPolicy {
    pub accepts_automated_pull_requests: bool,
    pub accepts_pull_requests: bool,
}

/// Overview of the project's supply chain.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Dependencies {
    pub env_dependencies_policy: Option<EnvDependenciesPolicy>,
    pub sbom: Option<Sbom>,
}

/// Dependencies policy information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct EnvDependenciesPolicy {
    pub comment: Option<String>,
    pub policy_url: Option<String>,
}

/// High-level information about the project.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Header {
    pub expiration_date: String,
    pub project_url: String,
    pub schema_version: String,
}

/// Status of the project.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct ProjectLifecycle {
    pub bug_fixes_only: bool,
    pub core_maintainers: Option<Vec<String>>,
    pub status: String,
}

/// SBOM information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", transparent)]
pub(crate) struct Sbom {
    pub entries: Vec<SbomEntry>,
}

/// SBOM entry information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(clippy::struct_field_names)]
pub(crate) struct SbomEntry {
    pub sbom_creation: Option<String>,
    pub sbom_file: Option<String>,
    pub sbom_format: Option<String>,
    pub sbom_url: Option<String>,
}

/// Security-focused documentation.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct SecurityArtifacts {
    pub self_assessment: Option<SelfAssessment>,
}

/// Self-assessment information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(clippy::struct_field_names)]
pub(crate) struct SelfAssessment {
    pub comment: Option<String>,
    pub evidence_url: Option<Vec<String>>,
    pub self_assessment_created: bool,
}

/// Security contact information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct SecurityContact {
    #[serde(alias = "type")]
    pub kind: String,
    pub value: String,
}

/// Policies and procedures about how to report properly a security issue.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct VulnerabilityReporting {
    accepts_vulnerability_reports: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA_PATH: &str = "src/testdata/security-insights-v1";

    #[test]
    fn new_returns_none_when_file_does_not_exist() {
        let result = SecurityInsights::new(&Path::new(TESTDATA_PATH).join("not-found")).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn new_returns_error_when_file_is_invalid() {
        let result = SecurityInsights::new(&Path::new(TESTDATA_PATH).join("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn new_parses_valid_manifest() {
        let result = SecurityInsights::new(Path::new(TESTDATA_PATH)).unwrap();
        assert!(result.is_some());
        let insights = result.unwrap();

        assert_eq!(insights.header.expiration_date, "2024-09-28T01:00:00.000Z");
        assert_eq!(
            insights.header.project_url,
            "https://github.com/ossf/security-insights-spec"
        );
        assert_eq!(insights.header.schema_version, "1.0.0");
        assert!(insights.contribution_policy.accepts_automated_pull_requests);
        assert!(insights.contribution_policy.accepts_pull_requests);
        assert!(!insights.project_lifecycle.bug_fixes_only);
        assert_eq!(insights.project_lifecycle.status, "active");
        assert!(
            insights
                .vulnerability_reporting
                .accepts_vulnerability_reports
        );
        assert_eq!(insights.security_contacts.len(), 1);
        assert_eq!(insights.security_contacts[0].kind, "email");
        assert_eq!(insights.security_contacts[0].value, "security@openssf.org");
    }
}
