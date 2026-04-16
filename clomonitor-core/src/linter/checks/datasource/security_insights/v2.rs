use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};

/// OpenSSF Security Insights v2 manifest.
///
/// Note: the types defined below do not contain *all* the fields available in
/// the specification, just the ones needed by CLOMonitor to validate the
/// minimum required manifest shape and extract fields used by checks.
///
/// For more details please see the spec documentation:
/// https://security-insights.openssf.org/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct SecurityInsights {
    pub header: Header,
    pub project: Option<Project>,
    pub repository: Option<Repository>,
}

impl SecurityInsights {
    /// Parse a v2 Security Insights manifest from the content provided.
    pub(super) fn parse_content(content: &str) -> Result<Self> {
        let manifest: Self =
            serde_yaml::from_str(content).context("invalid security insights manifest")?;

        manifest.validate()?;

        Ok(manifest)
    }

    /// Ensure the manifest contains the minimum required v2 structure.
    fn validate(&self) -> Result<()> {
        // Validate required header fields
        ensure!(
            self.header.schema_version.starts_with("2."),
            "invalid security insights manifest"
        );
        ensure_non_empty(&self.header.last_reviewed)?;
        ensure_non_empty(&self.header.last_updated)?;
        ensure_non_empty(&self.header.url)?;

        // A valid manifest must describe at least a project or a repository
        ensure!(
            self.project.is_some() || self.repository.is_some(),
            "invalid security insights manifest"
        );

        if let Some(project) = &self.project {
            ensure_non_empty(&project.name)?;
            ensure!(
                !project.administrators.is_empty(),
                "invalid security insights manifest"
            );
            for administrator in &project.administrators {
                ensure_non_empty(&administrator.name)?;
            }
            ensure!(
                !project.repositories.is_empty(),
                "invalid security insights manifest"
            );
            for repository in &project.repositories {
                ensure_non_empty(&repository.comment)?;
                ensure_non_empty(&repository.name)?;
                ensure_non_empty(&repository.url)?;
            }
        }

        if let Some(repository) = &self.repository {
            ensure_non_empty(&repository.license.expression)?;
            ensure_non_empty(&repository.license.url)?;
            ensure_non_empty(&repository.security.assessments.self_assessment.comment)?;
            ensure_non_empty(&repository.status)?;
            ensure_non_empty(&repository.url)?;
            ensure!(
                !repository.core_team.is_empty(),
                "invalid security insights manifest"
            );
            for member in &repository.core_team {
                ensure_non_empty(&member.name)?;
            }
            if let Some(documentation) = &repository.documentation
                && let Some(policy) = &documentation.dependency_management_policy
            {
                ensure_non_empty(policy)?;
            }
        }

        Ok(())
    }
}

/// Assessment information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Assessment {
    pub comment: String,
}

/// Assessments information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Assessments {
    #[serde(rename = "self")]
    pub self_assessment: Assessment,
}

/// Contact information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Contact {
    pub name: String,
    pub primary: bool,
}

/// High-level information about the manifest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Header {
    pub last_reviewed: String,
    pub last_updated: String,
    pub schema_version: String,
    pub url: String,
}

/// License information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct License {
    pub expression: String,
    pub url: String,
}

/// High-level information about the project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Project {
    pub administrators: Vec<Contact>,
    pub name: String,
    pub repositories: Vec<ProjectRepository>,
    pub vulnerability_reporting: VulnerabilityReporting,
}

/// Project repository information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct ProjectRepository {
    pub comment: String,
    pub name: String,
    pub url: String,
}

/// Repository information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Repository {
    pub accepts_automated_change_request: bool,
    pub accepts_change_request: bool,
    pub core_team: Vec<Contact>,
    pub license: License,
    pub security: SecurityPosture,
    pub status: String,
    pub url: String,

    pub documentation: Option<RepositoryDocumentation>,
}

/// Repository documentation information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct RepositoryDocumentation {
    pub dependency_management_policy: Option<String>,
}

/// Security posture information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct SecurityPosture {
    pub assessments: Assessments,
}

/// Vulnerability reporting information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct VulnerabilityReporting {
    pub bug_bounty_available: bool,
    pub reports_accepted: bool,
}

/// Ensure a required string field is not empty.
fn ensure_non_empty(value: &str) -> Result<()> {
    ensure!(!value.is_empty(), "invalid security insights manifest");
    Ok(())
}
