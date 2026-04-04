use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::linter::util;

/// Candidate manifest file paths, searched in order. First match wins.
/// v2 locations are checked first, with v1 as a fallback.
const MANIFEST_CANDIDATES: &[&str] = &[
    "security-insights.yml",
    ".github/security-insights.yml",
    ".gitlab/security-insights.yml",
    "SECURITY-INSIGHTS.yml",
];

/// Lightweight struct used to peek at the schema version before full parsing.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct VersionProbe {
    header: VersionProbeHeader,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct VersionProbeHeader {
    schema_version: String,
}

/// OpenSSF Security Insights manifest.
///
/// Supports both v1 (SECURITY-INSIGHTS.yml) and v2 (security-insights.yml)
/// of the specification.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SecurityInsights {
    /// Relative path (from repository root) where the manifest was found.
    pub manifest_path: PathBuf,
    /// Parsed manifest content.
    pub version: SecurityInsightsVersion,
}

/// Parsed manifest content, either v1 or v2.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum SecurityInsightsVersion {
    V2(SecurityInsightsV2),
    V1(SecurityInsightsV1),
}

impl SecurityInsights {
    /// Create a new SecurityInsights instance from the first manifest file
    /// found at the root path provided.
    pub(crate) fn new(root: &Path) -> Result<Option<Self>> {
        for candidate in MANIFEST_CANDIDATES {
            let full_path = root.join(candidate);
            if !full_path.exists() {
                continue;
            }
            let content = util::fs::read_to_string(&full_path)
                .context("error reading security insights manifest file")?;

            // Peek at schema-version to decide which struct to deserialize into.
            let probe: VersionProbe = serde_yaml::from_str(&content)
                .context("invalid security insights manifest (cannot read header)")?;

            let version = if probe.header.schema_version.starts_with("2.") {
                let v2: SecurityInsightsV2 = serde_yaml::from_str(&content)
                    .context("invalid security insights v2 manifest")?;
                SecurityInsightsVersion::V2(v2)
            } else if probe.header.schema_version.starts_with("1.") {
                let v1: SecurityInsightsV1 = serde_yaml::from_str(&content)
                    .context("invalid security insights v1 manifest")?;
                SecurityInsightsVersion::V1(v1)
            } else {
                return Ok(None);
            };

            return Ok(Some(SecurityInsights {
                manifest_path: PathBuf::from(candidate),
                version,
            }));
        }
        Ok(None)
    }

    /// Return the dependency policy URL, abstracting over v1 and v2 paths.
    pub(crate) fn dependency_policy_url(&self) -> Option<&str> {
        match &self.version {
            SecurityInsightsVersion::V1(v1) => v1
                .dependencies
                .as_ref()
                .and_then(|d| d.env_dependencies_policy.as_ref())
                .and_then(|p| p.policy_url.as_deref()),
            SecurityInsightsVersion::V2(v2) => v2
                .repository
                .as_ref()
                .and_then(|r| r.documentation.as_ref())
                .and_then(|d| d.dependency_management_policy.as_deref()),
        }
    }
}

// ---------------------------------------------------------------------------
// v2 types
// ---------------------------------------------------------------------------

/// OpenSSF Security Insights v2 manifest.
///
/// Note: the types defined below do not contain *all* the fields available in
/// the specification, just the ones needed by CLOMonitor.
///
/// Covers schema versions 2.0.0 through 2.2.0+. Minor versions only add
/// optional fields, so a single set of structs with `Option` handles all.
///
/// For more details please see the spec documentation:
/// https://github.com/ossf/security-insights/blob/v2.2.0/spec/schema.cue
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct SecurityInsightsV2 {
    pub header: HeaderV2,
    pub project: Option<ProjectV2>,
    pub repository: Option<RepositoryV2>,
}

/// High-level metadata about the schema (v2).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct HeaderV2 {
    pub schema_version: String,
    pub last_updated: String,
    pub last_reviewed: String,
    pub url: String,
    pub comment: Option<String>,
    pub project_si_source: Option<String>,
}

/// Project information (v2).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct ProjectV2 {
    pub name: String,
}

/// Repository information (v2).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct RepositoryV2 {
    pub url: String,
    pub status: String,
    pub documentation: Option<RepositoryDocumentationV2>,
}

/// Repository documentation links (v2).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct RepositoryDocumentationV2 {
    pub dependency_management_policy: Option<String>,
}

// ---------------------------------------------------------------------------
// v1 types (legacy)
// ---------------------------------------------------------------------------

/// OpenSSF Security Insights v1 manifest.
///
/// Note: the types defined below do not contain *all* the fields available in
/// the specification, just the ones needed by CLOMonitor.
///
/// For more details please see the spec documentation:
/// https://github.com/ossf/security-insights-spec/blob/v1.0.0/specification.md
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct SecurityInsightsV1 {
    pub contribution_policy: ContributionPolicy,
    pub dependencies: Option<Dependencies>,
    pub distribution_points: Vec<String>,
    pub header: HeaderV1,
    pub project_lifecycle: ProjectLifecycle,
    pub security_artifacts: Option<SecurityArtifacts>,
    pub security_contacts: Vec<SecurityContact>,
    pub vulnerability_reporting: VulnerabilityReportingV1,
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

/// High-level information about the project (v1).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct HeaderV1 {
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

/// Policies and procedures about how to report properly a security issue (v1).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct VulnerabilityReportingV1 {
    accepts_vulnerability_reports: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA_PATH_V2: &str = "src/testdata/security-insights-v2";
    const TESTDATA_PATH_V1: &str = "src/testdata/security-insights-v1";

    // -----------------------------------------------------------------------
    // General discovery tests
    // -----------------------------------------------------------------------

    #[test]
    fn new_returns_none_when_file_does_not_exist() {
        let result = SecurityInsights::new(&Path::new(TESTDATA_PATH_V2).join("not-found")).unwrap();
        assert!(result.is_none());
    }

    // -----------------------------------------------------------------------
    // v1 tests
    // -----------------------------------------------------------------------

    #[test]
    fn new_returns_error_when_v1_file_is_invalid() {
        let result = SecurityInsights::new(&Path::new(TESTDATA_PATH_V1).join("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn new_parses_valid_v1_manifest() {
        let result = SecurityInsights::new(Path::new(TESTDATA_PATH_V1)).unwrap();
        assert!(result.is_some());
        let si = result.unwrap();

        assert_eq!(si.manifest_path, PathBuf::from("SECURITY-INSIGHTS.yml"));
        let SecurityInsightsVersion::V1(v1) = &si.version else {
            panic!("expected V1");
        };

        assert_eq!(v1.header.expiration_date, "2024-09-28T01:00:00.000Z");
        assert_eq!(
            v1.header.project_url,
            "https://github.com/ossf/security-insights-spec"
        );
        assert_eq!(v1.header.schema_version, "1.0.0");
        assert!(v1.contribution_policy.accepts_automated_pull_requests);
        assert!(v1.contribution_policy.accepts_pull_requests);
        assert!(!v1.project_lifecycle.bug_fixes_only);
        assert_eq!(v1.project_lifecycle.status, "active");
        assert!(v1.vulnerability_reporting.accepts_vulnerability_reports);
        assert_eq!(v1.security_contacts.len(), 1);
        assert_eq!(v1.security_contacts[0].kind, "email");
        assert_eq!(v1.security_contacts[0].value, "security@openssf.org");
    }

    // -----------------------------------------------------------------------
    // v2 tests
    // -----------------------------------------------------------------------

    #[test]
    fn new_returns_error_when_v2_file_is_invalid() {
        let result = SecurityInsights::new(&Path::new(TESTDATA_PATH_V2).join("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn new_parses_valid_v2_manifest() {
        let result = SecurityInsights::new(Path::new(TESTDATA_PATH_V2)).unwrap();
        assert!(result.is_some());
        let si = result.unwrap();

        assert_eq!(si.manifest_path, PathBuf::from("security-insights.yml"));
        let SecurityInsightsVersion::V2(v2) = &si.version else {
            panic!("expected V2");
        };

        assert_eq!(v2.header.schema_version, "2.0.0");
        assert_eq!(
            v2.header.url,
            "https://example.com/foo/bar/raw/branch/main/security-insights.yml"
        );
        assert_eq!(v2.repository.as_ref().unwrap().status, "active");
    }

    #[test]
    fn new_finds_v2_in_github_dir() {
        let result =
            SecurityInsights::new(Path::new("src/testdata/security-insights-v2-github")).unwrap();
        assert!(result.is_some());
        let si = result.unwrap();

        assert_eq!(
            si.manifest_path,
            PathBuf::from(".github/security-insights.yml")
        );
        assert!(matches!(si.version, SecurityInsightsVersion::V2(_)));
    }

    // -----------------------------------------------------------------------
    // Helper method tests
    // -----------------------------------------------------------------------

    #[test]
    fn dependency_policy_url_v1() {
        let si = SecurityInsights::new(Path::new(TESTDATA_PATH_V1))
            .unwrap()
            .unwrap();
        // The v1 test fixture does not have env-dependencies-policy set.
        assert!(si.dependency_policy_url().is_none());
    }

    #[test]
    fn dependency_policy_url_v2() {
        let si = SecurityInsights::new(Path::new(TESTDATA_PATH_V2))
            .unwrap()
            .unwrap();
        assert_eq!(
            si.dependency_policy_url(),
            Some("https://example.com/dependency-management-policy")
        );
    }

    #[test]
    fn new_returns_none_for_unsupported_version() {
        let result =
            SecurityInsights::new(Path::new("src/testdata/security-insights-unsupported")).unwrap();
        assert!(result.is_none());
    }
}
