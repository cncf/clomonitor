use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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
        let content = fs::read_to_string(manifest_path)?;
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
