use anyhow::Error;
use askalono::*;
use glob::{glob_with, MatchOptions, PatternError};
use lazy_static::lazy_static;
use regex::{Regex, RegexSet};
use std::fs;
use std::path::{Path, PathBuf};

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

/// Glob matching configuration.
#[derive(Debug)]
pub(crate) struct Globs<'a, P>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
{
    pub root: &'a Path,
    pub patterns: P,
    pub case_sensitive: bool,
}

/// Check if the content of any of the files that match the globs provided
/// matches any of the regular expressions given, returning the captured value
/// when there is a match. This function expects that the regular expressions
/// provided contain one capture group.
pub(crate) fn content_find<P, R>(globs: Globs<P>, regexps: R) -> Result<Option<String>, Error>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
    R: IntoIterator,
    R::Item: AsRef<str>,
{
    let mut res = Vec::<Regex>::new();
    for regexp in regexps {
        res.push(Regex::new(regexp.as_ref())?);
    }
    for path in matching_paths(globs)?.iter() {
        if let Ok(content) = fs::read_to_string(path) {
            for re in res.iter() {
                if let Some(c) = re.captures(&content) {
                    return Ok(Some(c[1].to_string()));
                }
            }
        }
    }
    Ok(None)
}

/// Check if the content of any of the files that match the globs provided
/// matches any of the regular expressions given.
pub(crate) fn content_matches<P, R>(globs: Globs<P>, regexps: R) -> Result<bool, Error>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
    R: IntoIterator,
    R::Item: AsRef<str>,
{
    let re = RegexSet::new(regexps)?;
    Ok(matching_paths(globs)?.iter().any(|path| {
        if let Ok(content) = fs::read_to_string(path) {
            return re.is_match(&content);
        }
        false
    }))
}

/// Check if the license provided is an approved one.
pub(crate) fn is_approved_license(spdx_id: &str) -> bool {
    APPROVED_LICENSES.contains(&spdx_id)
}

/// Check repository's license and return its SPDX id if possible.
pub(crate) fn license<P>(globs: Globs<P>) -> Result<Option<String>, Error>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
{
    lazy_static! {
        static ref LICENSES: Store = Store::from_cache(LICENSES_DATA).unwrap();
    }
    let mut spdx_id: Option<String> = None;
    matching_paths(globs)?.iter().any(|path| {
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

/// Check if exists at least a path that matches the globs provided.
pub(crate) fn path_exists<P>(globs: Globs<P>) -> Result<bool, PatternError>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
{
    Ok(!matching_paths(globs)?.is_empty())
}

/// Return all paths that match any of the globs provided.
fn matching_paths<P>(globs: Globs<P>) -> Result<Vec<PathBuf>, PatternError>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
{
    let options = MatchOptions {
        case_sensitive: globs.case_sensitive,
        ..Default::default()
    };
    globs
        .patterns
        .into_iter()
        .map(|pattern| globs.root.join(pattern.as_ref()))
        .map(|pattern| pattern.to_string_lossy().into_owned())
        .try_fold(Vec::new(), |mut paths, pattern| {
            match glob_with(&pattern, options) {
                Ok(pattern_paths) => {
                    paths.extend(pattern_paths.filter_map(Result::ok));
                    Ok(paths)
                }
                Err(err) => Err(err),
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linter::patterns::*;

    const TESTDATA_PATH: &str = "src/linter/testdata";

    #[test]
    fn content_find_found() {
        assert_eq!(
            content_find(
                Globs {
                    root: Path::new(TESTDATA_PATH),
                    patterns: README_FILE,
                    case_sensitive: true,
                },
                LICENSE_SCANNING_URL
            )
            .unwrap()
            .unwrap(),
            "https://snyk.io/test/github/username/repo".to_string()
        );
    }

    #[test]
    fn content_find_not_found() {
        assert_eq!(
            content_find(
                Globs {
                    root: Path::new(TESTDATA_PATH),
                    patterns: README_FILE,
                    case_sensitive: true,
                },
                [r"non-existing pattern"]
            )
            .unwrap(),
            None
        );
    }

    #[test]
    fn content_find_file_not_found() {
        assert_eq!(
            content_find(
                Globs {
                    root: Path::new(TESTDATA_PATH),
                    patterns: vec!["nonexisting"],
                    case_sensitive: true,
                },
                [r"pattern"]
            )
            .unwrap(),
            None
        );
    }

    #[test]
    fn content_find_invalid_glob_pattern() {
        assert!(matches!(
            content_find(
                Globs {
                    root: Path::new(TESTDATA_PATH),
                    patterns: vec!["invalid***"],
                    case_sensitive: true,
                },
                [r"pattern"]
            ),
            Err(_)
        ));
    }

    #[test]
    fn content_find_invalid_regexp() {
        assert!(matches!(
            content_find(
                Globs {
                    root: Path::new(TESTDATA_PATH),
                    patterns: README_FILE,
                    case_sensitive: true,
                },
                [r"***"]
            ),
            Err(_)
        ));
    }

    #[test]
    fn content_matches_match() {
        assert!(content_matches(
            Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: README_FILE,
                case_sensitive: true,
            },
            ADOPTERS_HEADER
        )
        .unwrap());
    }

    #[test]
    fn content_matches_no_match() {
        assert!(!content_matches(
            Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: README_FILE,
                case_sensitive: true,
            },
            [r"non-existing pattern"]
        )
        .unwrap());
    }

    #[test]
    fn content_matches_file_not_found() {
        assert!(!content_matches(
            Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: vec!["nonexisting"],
                case_sensitive: true,
            },
            [r"pattern"]
        )
        .unwrap());
    }

    #[test]
    fn content_matches_invalid_glob_pattern() {
        assert!(matches!(
            content_matches(
                Globs {
                    root: Path::new(TESTDATA_PATH),
                    patterns: vec!["invalid***"],
                    case_sensitive: true,
                },
                [r"pattern"]
            ),
            Err(_)
        ));
    }

    #[test]
    fn content_matches_invalid_regexp() {
        assert!(matches!(
            content_matches(
                Globs {
                    root: Path::new(TESTDATA_PATH),
                    patterns: README_FILE,
                    case_sensitive: true,
                },
                [r"***"]
            ),
            Err(_)
        ));
    }

    #[test]
    fn approved_license() {
        assert!(is_approved_license("Apache-2.0"));
        assert!(is_approved_license("MIT"));
    }

    #[test]
    fn non_approved_license() {
        assert!(!is_approved_license("AGPL-1.0-only"));
    }

    #[test]
    fn license_identified() {
        assert_eq!(
            license(Globs {
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
    fn license_not_identified() {
        assert!(matches!(
            license(Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: vec!["OWNERS"],
                case_sensitive: true,
            })
            .unwrap(),
            None
        ));
    }

    #[test]
    fn license_file_not_located() {
        assert!(matches!(
            license(Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: vec!["nonexisting"],
                case_sensitive: true,
            })
            .unwrap(),
            None
        ));
    }

    #[test]
    fn license_invalid_glob_pattern() {
        assert!(matches!(
            license(Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: vec!["invalid***"],
                case_sensitive: true,
            }),
            Err(_)
        ));
    }

    #[test]
    fn matching_paths_case_insensitive_found() {
        let testdata = Path::new(TESTDATA_PATH);

        assert_eq!(
            matching_paths(Globs {
                root: testdata,
                patterns: MAINTAINERS_FILE,
                case_sensitive: false,
            })
            .unwrap(),
            vec![testdata.join("MAINTAINERS"), testdata.join("OWNERS"),]
        );
    }

    #[test]
    fn matching_paths_case_sensitive_found() {
        let testdata = Path::new(TESTDATA_PATH);

        assert_eq!(
            matching_paths(Globs {
                root: testdata,
                patterns: ["OWNERS*"],
                case_sensitive: true,
            })
            .unwrap(),
            vec![testdata.join("OWNERS")]
        );
    }

    #[test]
    fn matching_paths_not_found() {
        assert_eq!(
            matching_paths(Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: vec!["nonexisting"],
                case_sensitive: false,
            })
            .unwrap(),
            Vec::<PathBuf>::new()
        );
    }

    #[test]
    fn matching_paths_invalid_glob_pattern() {
        assert!(matches!(
            matching_paths(Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: vec!["invalid***"],
                case_sensitive: true,
            }),
            Err(_)
        ));
    }

    #[test]
    fn path_exists_existing_path() {
        assert!(path_exists(Globs {
            root: Path::new(TESTDATA_PATH),
            patterns: MAINTAINERS_FILE,
            case_sensitive: false,
        })
        .unwrap());
    }

    #[test]
    fn path_exists_non_existing_path() {
        assert!(!path_exists(Globs {
            root: Path::new(TESTDATA_PATH),
            patterns: vec!["nonexisting"],
            case_sensitive: false,
        })
        .unwrap());
    }

    #[test]
    fn path_exists_invalid_glob_pattern() {
        assert!(matches!(
            path_exists(Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: vec!["invalid***"],
                case_sensitive: false,
            }),
            Err(_)
        ));
    }
}
