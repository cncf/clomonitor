use std::{
    fs,
    path::{Component, Path, PathBuf},
};

use anyhow::{Context, Result, format_err};
use serde::Deserialize;

use crate::linter::util;

pub(crate) mod v1;
pub(crate) mod v2;

/// Primary Security Insights manifest file name.
const PRIMARY_MANIFEST_FILE: &str = "security-insights.yml";

/// Legacy Security Insights manifest file name.
const LEGACY_MANIFEST_FILE: &str = "SECURITY-INSIGHTS.yml";

/// Version-specific Security Insights manifest representation.
#[derive(Debug, Clone, PartialEq)]
enum Manifest {
    V1(Box<v1::SecurityInsights>),
    V2(Box<v2::SecurityInsights>),
}

/// OpenSSF Security Insights manifest.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SecurityInsights {
    manifest: Manifest,
    manifest_rel_path: PathBuf,
}

impl SecurityInsights {
    /// Create a new SecurityInsights instance by scanning the supported
    /// manifest locations under the path provided, stopping at the first v2
    /// manifest found and otherwise falling back to the first v1 manifest
    /// found.
    pub(crate) fn new(path: &Path) -> Result<Option<Self>> {
        let mut first_v1_manifest = None;

        // Check supported manifest locations in order of preference
        for manifest_rel_path in [
            Path::new(PRIMARY_MANIFEST_FILE),
            Path::new(".github").join(PRIMARY_MANIFEST_FILE).as_path(),
            Path::new(LEGACY_MANIFEST_FILE),
        ] {
            if let Some(manifest_path) = resolve_manifest_path(path, manifest_rel_path)? {
                let content = util::fs::read_to_string(&manifest_path)
                    .context("error reading security insights manifest file")?;

                match detect_manifest_version(&content)? {
                    ManifestVersion::V1 => {
                        // Store the first v1 manifest found in case no v2 manifest is found
                        if first_v1_manifest.is_none() {
                            first_v1_manifest = Some((content, manifest_rel_path.to_path_buf()));
                        }
                    }
                    ManifestVersion::V2 => {
                        // Stop at the first v2 manifest found
                        let manifest = v2::SecurityInsights::parse_content(&content)?;
                        return Ok(Some(Self {
                            manifest: Manifest::V2(Box::new(manifest)),
                            manifest_rel_path: manifest_rel_path.to_path_buf(),
                        }));
                    }
                }
            }
        }

        // No v2 manifest found, fallback to v1 if available
        if let Some((content, manifest_rel_path)) = first_v1_manifest {
            let manifest = v1::SecurityInsights::parse_content(&content)?;
            return Ok(Some(Self {
                manifest: Manifest::V1(Box::new(manifest)),
                manifest_rel_path,
            }));
        }

        Ok(None)
    }

    /// Return the dependencies policy url when it is available.
    pub(crate) fn dependencies_policy_url(&self) -> Option<&str> {
        match &self.manifest {
            Manifest::V1(manifest) => manifest
                .dependencies
                .as_ref()
                .and_then(|dependencies| dependencies.env_dependencies_policy.as_ref())
                .and_then(|policy| policy.policy_url.as_deref()),
            Manifest::V2(manifest) => manifest
                .repository
                .as_ref()
                .and_then(|repository| repository.documentation.as_ref())
                .and_then(|documentation| documentation.dependency_management_policy.as_deref()),
        }
    }

    /// Return the relative path of the matched manifest.
    pub(crate) fn manifest_rel_path(&self) -> &Path {
        &self.manifest_rel_path
    }
}

/// Find the manifest path matching the relative path provided exactly.
fn resolve_manifest_path(root: &Path, manifest_rel_path: &Path) -> Result<Option<PathBuf>> {
    // Walk each path component ensuring the on-disk names match exactly
    let mut current = root.to_path_buf();
    for component in manifest_rel_path.components() {
        let Component::Normal(name) = component else {
            return Ok(None);
        };

        // Stop if any parent component is not a directory
        if !current.is_dir() {
            return Ok(None);
        }

        // Compare directory entries directly to avoid filesystem case folding
        let mut next = None;
        for entry in fs::read_dir(&current)? {
            let entry = entry?;
            if entry.file_name() == name {
                if entry.file_type()?.is_symlink() {
                    return Ok(None);
                }

                next = Some(entry.path());
                break;
            }
        }

        let Some(path) = next else {
            return Ok(None);
        };
        current = path;
    }

    Ok(Some(current))
}

// Manifest version detection

/// Minimal Security Insights manifest header used for version detection.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Header {
    schema_version: String,
}

/// Version declared by the manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ManifestVersion {
    V1,
    V2,
}

/// Minimal Security Insights manifest used for version detection.
#[derive(Debug, Deserialize)]
struct VersionedManifest {
    header: Header,
}

/// Detect the manifest version using the `header.schema-version` field.
fn detect_manifest_version(content: &str) -> Result<ManifestVersion> {
    let manifest: VersionedManifest =
        serde_yaml::from_str(content).context("invalid security insights manifest")?;

    if manifest.header.schema_version.starts_with("1.") {
        Ok(ManifestVersion::V1)
    } else if manifest.header.schema_version.starts_with("2.") {
        Ok(ManifestVersion::V2)
    } else {
        Err(format_err!("invalid security insights manifest"))
    }
}

#[cfg(test)]
mod tests {
    #[cfg(unix)]
    use std::os::unix::fs::symlink;

    use tempfile::tempdir;

    use super::*;

    const TESTDATA_PATH: &str = "src/testdata";

    #[test]
    fn new_falls_back_to_v1_manifest() {
        let result =
            SecurityInsights::new(&Path::new(TESTDATA_PATH).join("security-insights-v1/root"))
                .unwrap();
        assert!(result.is_some());

        let insights = result.unwrap();
        assert_eq!(
            insights.dependencies_policy_url(),
            Some("https://example.com/v1/dependencies-policy")
        );
        assert_eq!(
            insights.manifest_rel_path(),
            Path::new(LEGACY_MANIFEST_FILE)
        );
    }

    #[cfg(unix)]
    #[test]
    fn new_ignores_only_symlinked_github_path() {
        let repository_root = tempdir().unwrap();
        let external_manifest_root = Path::new(TESTDATA_PATH)
            .join("security-insights-v2/github/.github")
            .canonicalize()
            .unwrap();

        symlink(
            &external_manifest_root,
            repository_root.path().join(".github"),
        )
        .unwrap();

        let result = SecurityInsights::new(repository_root.path());

        assert!(result.unwrap().is_none());
    }

    #[cfg(unix)]
    #[test]
    fn new_ignores_symlinked_lower_priority_github_path() {
        let repository_root = tempdir().unwrap();
        let external_manifest_root = Path::new(TESTDATA_PATH)
            .join("security-insights-v2/github/.github")
            .canonicalize()
            .unwrap();

        fs::copy(
            Path::new(TESTDATA_PATH)
                .join("security-insights-v2/root")
                .join(PRIMARY_MANIFEST_FILE),
            repository_root.path().join(PRIMARY_MANIFEST_FILE),
        )
        .unwrap();
        symlink(
            &external_manifest_root,
            repository_root.path().join(".github"),
        )
        .unwrap();

        let result = SecurityInsights::new(repository_root.path()).unwrap();
        assert!(result.is_some());

        let insights = result.unwrap();
        assert_eq!(
            insights.dependencies_policy_url(),
            Some("https://example.com/v2/dependency-management-policy")
        );
        assert_eq!(
            insights.manifest_rel_path(),
            Path::new(PRIMARY_MANIFEST_FILE)
        );
    }

    #[cfg(unix)]
    #[test]
    fn new_ignores_symlinked_root_manifest_and_uses_github_v2() {
        let repository_root = tempdir().unwrap();
        let external_manifest_path = Path::new(TESTDATA_PATH)
            .join("security-insights-v2/root")
            .join(PRIMARY_MANIFEST_FILE)
            .canonicalize()
            .unwrap();

        symlink(
            &external_manifest_path,
            repository_root.path().join(PRIMARY_MANIFEST_FILE),
        )
        .unwrap();
        fs::create_dir(repository_root.path().join(".github")).unwrap();
        fs::copy(
            Path::new(TESTDATA_PATH)
                .join("security-insights-v2/github/.github")
                .join(PRIMARY_MANIFEST_FILE),
            repository_root
                .path()
                .join(".github")
                .join(PRIMARY_MANIFEST_FILE),
        )
        .unwrap();

        let result = SecurityInsights::new(repository_root.path()).unwrap();
        assert!(result.is_some());

        let insights = result.unwrap();
        assert_eq!(
            insights.dependencies_policy_url(),
            Some("https://example.com/v2/github/dependency-management-policy")
        );
        assert_eq!(
            insights.manifest_rel_path(),
            Path::new(".github").join(PRIMARY_MANIFEST_FILE).as_path()
        );
    }

    #[test]
    fn new_prefers_later_v2_manifest_over_earlier_invalid_v1_manifest() {
        let result = SecurityInsights::new(
            &Path::new(TESTDATA_PATH)
                .join("security-insights-v1-and-v2/prefer-legacy-v2-over-invalid-github-v1"),
        )
        .unwrap();
        assert!(result.is_some());

        let insights = result.unwrap();
        assert_eq!(
            insights.dependencies_policy_url(),
            Some("https://example.com/v2/legacy-invalid-v1/dependency-management-policy")
        );
        assert_eq!(
            insights.manifest_rel_path(),
            Path::new(LEGACY_MANIFEST_FILE)
        );
    }

    #[test]
    fn new_prefers_later_v2_manifest_over_earlier_v1_manifest() {
        let result = SecurityInsights::new(
            &Path::new(TESTDATA_PATH)
                .join("security-insights-v1-and-v2/prefer-legacy-v2-over-github-v1"),
        )
        .unwrap();
        assert!(result.is_some());

        let insights = result.unwrap();
        assert_eq!(
            insights.dependencies_policy_url(),
            Some("https://example.com/v2/legacy/dependency-management-policy")
        );
        assert_eq!(
            insights.manifest_rel_path(),
            Path::new(LEGACY_MANIFEST_FILE)
        );
    }

    #[test]
    fn new_returns_error_when_v1_manifest_is_invalid() {
        let result =
            SecurityInsights::new(&Path::new(TESTDATA_PATH).join("security-insights-v1/invalid"));

        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid security insights manifest"
        );
    }

    #[test]
    fn new_returns_error_when_v2_manifest_is_invalid() {
        let result = SecurityInsights::new(
            &Path::new(TESTDATA_PATH).join("security-insights-v1-and-v2/invalid-github-v2"),
        );

        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid security insights manifest"
        );
    }

    #[test]
    fn new_returns_none_when_file_does_not_exist() {
        let result =
            SecurityInsights::new(&Path::new(TESTDATA_PATH).join("security-insights-not-found"))
                .unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn new_stops_at_first_v2_manifest_found() {
        let result = SecurityInsights::new(
            &Path::new(TESTDATA_PATH).join("security-insights-v1-and-v2/prefer-github-v2"),
        )
        .unwrap();
        assert!(result.is_some());

        let insights = result.unwrap();
        assert_eq!(
            insights.dependencies_policy_url(),
            Some("https://example.com/v2/github/dependency-management-policy")
        );
        assert_eq!(
            insights.manifest_rel_path(),
            Path::new(".github").join(PRIMARY_MANIFEST_FILE).as_path()
        );
    }

    #[test]
    fn new_uses_github_v2_manifest_when_available() {
        let result =
            SecurityInsights::new(&Path::new(TESTDATA_PATH).join("security-insights-v2/github"))
                .unwrap();
        assert!(result.is_some());

        let insights = result.unwrap();
        assert_eq!(
            insights.dependencies_policy_url(),
            Some("https://example.com/v2/github/dependency-management-policy")
        );
        assert_eq!(
            insights.manifest_rel_path(),
            Path::new(".github").join(PRIMARY_MANIFEST_FILE).as_path()
        );
    }

    #[test]
    fn new_uses_root_v2_manifest_when_available() {
        let result =
            SecurityInsights::new(&Path::new(TESTDATA_PATH).join("security-insights-v2/root"))
                .unwrap();
        assert!(result.is_some());

        let insights = result.unwrap();
        assert_eq!(
            insights.dependencies_policy_url(),
            Some("https://example.com/v2/dependency-management-policy")
        );
        assert_eq!(
            insights.manifest_rel_path(),
            Path::new(PRIMARY_MANIFEST_FILE)
        );
    }

    #[test]
    fn new_uses_root_v2_manifest_when_github_v2_is_also_available() {
        let result = SecurityInsights::new(
            &Path::new(TESTDATA_PATH).join("security-insights-v2/prefer-root"),
        )
        .unwrap();
        assert!(result.is_some());

        let insights = result.unwrap();
        assert_eq!(
            insights.dependencies_policy_url(),
            Some("https://example.com/v2/root/dependency-management-policy")
        );
        assert_eq!(
            insights.manifest_rel_path(),
            Path::new(PRIMARY_MANIFEST_FILE)
        );
    }

    #[test]
    fn new_uses_schema_version_for_legacy_manifest_path() {
        let result = SecurityInsights::new(
            &Path::new(TESTDATA_PATH).join("security-insights-v1-and-v2/v2-in-legacy-path"),
        )
        .unwrap();
        assert!(result.is_some());

        let insights = result.unwrap();
        assert_eq!(
            insights.dependencies_policy_url(),
            Some("https://example.com/v2/dependency-management-policy")
        );
        assert_eq!(
            insights.manifest_rel_path(),
            Path::new(LEGACY_MANIFEST_FILE)
        );
    }

    #[test]
    fn new_uses_schema_version_for_root_security_insights_manifest() {
        let result = SecurityInsights::new(
            &Path::new(TESTDATA_PATH).join("security-insights-v1-and-v2/v1-in-primary-path"),
        )
        .unwrap();
        assert!(result.is_some());

        let insights = result.unwrap();
        assert_eq!(
            insights.dependencies_policy_url(),
            Some("https://example.com/v1/dependencies-policy")
        );
        assert_eq!(
            insights.manifest_rel_path(),
            Path::new(PRIMARY_MANIFEST_FILE)
        );
    }

    #[test]
    fn new_validates_v2_required_fields() {
        let result = SecurityInsights::new(
            &Path::new(TESTDATA_PATH).join("security-insights-v2/invalid-header"),
        );

        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid security insights manifest"
        );
    }

    #[test]
    fn new_validates_v2_required_repository_fields() {
        let result = SecurityInsights::new(
            &Path::new(TESTDATA_PATH).join("security-insights-v2/invalid-repository"),
        );

        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid security insights manifest"
        );
    }

    #[test]
    fn new_validates_v2_required_sections() {
        let result = SecurityInsights::new(
            &Path::new(TESTDATA_PATH).join("security-insights-v2/invalid-missing-sections"),
        );

        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid security insights manifest"
        );
    }
}
