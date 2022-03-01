use anyhow::Error;
use serde::Deserialize;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

/// Metadata file name.
pub const METADATA_FILE: &str = ".clomonitor.yml";

/// CLOMonitor metadata.
#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub license_scanning: Option<LicenseScanning>,
}

impl Metadata {
    /// Create a new metadata instance from the contents of the file located at
    /// the path provided.
    pub fn from<P: AsRef<OsStr>>(path: P) -> Result<Option<Self>, Error> {
        if !Path::new(&path).exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(path.as_ref())?;
        Ok(serde_yaml::from_str(&content)?)
    }
}

/// License scanning section of the metadata.
#[derive(Debug, Deserialize)]
pub struct LicenseScanning {
    pub url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA_PATH: &str = "src/linter/testdata";

    #[test]
    fn metadata_from_path_success() {
        assert_eq!(
            Metadata::from(Path::new(TESTDATA_PATH).join(METADATA_FILE))
                .unwrap()
                .unwrap()
                .license_scanning
                .unwrap()
                .url
                .unwrap(),
            "https://license-scanning.url"
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
        assert!(matches!(
            Metadata::from(Path::new(TESTDATA_PATH).join("LICENSE")),
            Err(_)
        ));
    }
}
