use std::sync::LazyLock;

use anyhow::Result;
use askalono::*;

use crate::linter::check::{CheckId, CheckInput, CheckOutput};
use crate::linter::checks::util::path;
use crate::linter::{util, CheckSet};

use super::util::path::Globs;

/// Check identifier.
pub(crate) const ID: CheckId = "license_spdx_id";

/// Check score weight.
pub(crate) const WEIGHT: usize = 5;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 3] = [CheckSet::Code, CheckSet::CodeLite, CheckSet::Docs];

/// SPDX licenses data. Used to detect license used by repositories.
const LICENSES_DATA: &[u8] = include_bytes!("licenses/licenses.bin.zstd");

/// Patterns used to locate a file in the repository.
pub(crate) const FILE_PATTERNS: [&str; 2] = ["LICENSE*", "COPYING*"];

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput<String>> {
    // File in repo
    if let Some(spdx_id) = detect(&Globs {
        root: &input.li.root,
        patterns: &FILE_PATTERNS,
        case_sensitive: true,
    })? {
        return Ok(CheckOutput::passed().value(Some(spdx_id)));
    }

    // License detected by Github
    if let Some(spdx_id) = input
        .gh_md
        .license_info
        .as_ref()
        .and_then(|l| l.spdx_id.as_ref())
    {
        if spdx_id != "NOASSERTION" {
            return Ok(CheckOutput::passed().value(Some(spdx_id.clone())));
        }
    }

    Ok(CheckOutput::not_passed())
}

/// Detect repository's license and return its SPDX id if possible.
pub(crate) fn detect(globs: &Globs) -> Result<Option<String>> {
    static LICENSES: LazyLock<Store> = LazyLock::new(|| {
        Store::from_cache(LICENSES_DATA).expect("valid licenses data file present")
    });

    let mut spdx_id: Option<String> = None;
    path::matches(globs)?.iter().any(|path| {
        if let Ok(content) = util::fs::read_to_string(path) {
            let m = LICENSES.analyze(&TextData::from(content));
            if m.score > 0.9 {
                spdx_id = Some(m.name.to_string());
                return true;
            }
        }
        false
    });
    Ok(spdx_id)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::linter::checks::license_spdx_id;

    use super::*;

    const TESTDATA_PATH: &str = "src/testdata";

    #[test]
    fn detect_identified() {
        assert_eq!(
            detect(&Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: &license_spdx_id::FILE_PATTERNS,
                case_sensitive: true,
            })
            .unwrap()
            .unwrap(),
            "Apache-2.0".to_string()
        );
    }

    #[test]
    fn detect_not_identified() {
        assert!(detect(&Globs {
            root: Path::new(TESTDATA_PATH),
            patterns: &["OWNERS"],
            case_sensitive: true,
        })
        .unwrap()
        .is_none());
    }

    #[test]
    fn detect_file_not_located() {
        assert!(detect(&Globs {
            root: Path::new(TESTDATA_PATH),
            patterns: &["nonexisting"],
            case_sensitive: true,
        })
        .unwrap()
        .is_none());
    }

    #[test]
    fn detect_invalid_glob_pattern() {
        assert!(detect(&Globs {
            root: Path::new(TESTDATA_PATH),
            patterns: &["invalid***"],
            case_sensitive: true,
        })
        .is_err());
    }
}
