use super::path::{matches, Globs};
use anyhow::Error;
use askalono::*;
use lazy_static::lazy_static;
use std::fs;

/// SPDX licenses data. Used to detect license used by repositories.
const LICENSES_DATA: &[u8] = include_bytes!("data/licenses.bin.zstd");

/// CNCF approved licenses.
/// https://github.com/cncf/foundation/blob/master/allowed-third-party-license-policy.md
static APPROVED_LICENSES: [&str; 10] = [
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-2-Clause-FreeBSD",
    "BSD-3-Clause",
    "ISC",
    "MIT",
    "PostgreSQL",
    "Python-2.0",
    "X11",
    "Zlib",
];

/// Check if the license provided is an approved one.
pub(crate) fn is_approved(spdx_id: &str) -> bool {
    APPROVED_LICENSES.contains(&spdx_id)
}

/// Detect repository's license and return its SPDX id if possible.
pub(crate) fn detect<P>(globs: Globs<P>) -> Result<Option<String>, Error>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
{
    lazy_static! {
        static ref LICENSES: Store = Store::from_cache(LICENSES_DATA).unwrap();
    }
    let mut spdx_id: Option<String> = None;
    matches(globs)?.iter().any(|path| {
        if let Ok(content) = fs::read_to_string(path) {
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
    use super::*;
    use crate::linter::patterns::*;
    use std::path::Path;

    const TESTDATA_PATH: &str = "src/linter/check/testdata";

    #[test]
    fn approved_license() {
        assert!(is_approved("Apache-2.0"));
        assert!(is_approved("MIT"));
    }

    #[test]
    fn non_approved_license() {
        assert!(!is_approved("AGPL-1.0-only"));
    }

    #[test]
    fn detect_identified() {
        assert_eq!(
            detect(Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: LICENSE_FILE,
                case_sensitive: true,
            })
            .unwrap()
            .unwrap(),
            "Apache-2.0".to_string()
        );
    }

    #[test]
    fn detect_not_identified() {
        assert!(matches!(
            detect(Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: vec!["OWNERS"],
                case_sensitive: true,
            })
            .unwrap(),
            None
        ));
    }

    #[test]
    fn detect_file_not_located() {
        assert!(matches!(
            detect(Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: vec!["nonexisting"],
                case_sensitive: true,
            })
            .unwrap(),
            None
        ));
    }

    #[test]
    fn detect_invalid_glob_pattern() {
        assert!(matches!(
            detect(Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: vec!["invalid***"],
                case_sensitive: true,
            }),
            Err(_)
        ));
    }
}
