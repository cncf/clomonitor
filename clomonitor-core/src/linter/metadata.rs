use std::ffi::OsStr;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

use super::util;

/// Metadata file name.
pub(crate) const METADATA_FILE: &str = ".clomonitor.yml";

/// CLOMonitor metadata.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Metadata {
    pub exemptions: Option<Vec<Exemption>>,
    pub license_scanning: Option<LicenseScanning>,
}

impl Metadata {
    /// Create a new metadata instance from the contents of the file located at
    /// the path provided.
    pub(crate) fn from<P: AsRef<OsStr>>(path: P) -> Result<Option<Self>> {
        if !Path::new(&path).exists() {
            return Ok(None);
        }
        let content = util::fs::read_to_string(path.as_ref())
            .context("error reading clomonitor metadata file")?;
        Ok(serde_yaml::from_str(&content)?)
    }
}

/// Metadata check exemption entry.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub(crate) struct Exemption {
    pub check: String,
    pub reason: String,
}

/// License scanning section of the metadata.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub(crate) struct LicenseScanning {
    pub url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA_PATH: &str = "src/testdata";

    #[test]
    fn metadata_from_path_success() {
        assert_eq!(
            Metadata::from(Path::new(TESTDATA_PATH).join(METADATA_FILE))
                .unwrap()
                .unwrap(),
            Metadata {
                license_scanning: Some(LicenseScanning {
                    url: Some("https://license-scanning-results.url".to_string()),
                }),
                exemptions: Some(vec![Exemption {
                    check: "artifacthub_badge".to_string(),
                    reason: "this is a sample reason".to_string(),
                }])
            },
        );
    }

    #[test]
    fn metadata_from_path_not_found() {
        assert!(matches!(
            Metadata::from(Path::new(TESTDATA_PATH).join("not-found")),
            Ok(None)
        ));
    }

    #[test]
    fn metadata_from_path_invalid_metadata_file() {
        assert!(Metadata::from(Path::new(TESTDATA_PATH).join(".clomonitor-invalid.yaml")).is_err());
    }
}
