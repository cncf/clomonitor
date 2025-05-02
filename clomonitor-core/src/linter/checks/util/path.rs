use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use anyhow::Result;
use glob::{glob_with, MatchOptions, PatternError};

/// Glob matching configuration.
#[derive(Debug, Clone)]
pub(crate) struct Globs<'a> {
    pub root: &'a Path,
    pub patterns: &'a [&'a str],
    pub case_sensitive: bool,
}

/// Find the first path that matches any of the globs provided.
pub(crate) fn find(globs: &Globs) -> Result<Option<PathBuf>> {
    match matches(globs)?.first() {
        Some(path) => Ok(Some(
            if globs.root.as_os_str() == OsStr::new(".") || globs.root.as_os_str().is_empty() {
                path
            } else {
                path.strip_prefix(globs.root)?
            }
            .to_owned(),
        )),
        None => Ok(None),
    }
}

/// Return all paths that match any of the globs provided.
pub(crate) fn matches(globs: &Globs) -> Result<Vec<PathBuf>, PatternError> {
    let options = MatchOptions {
        case_sensitive: globs.case_sensitive,
        ..Default::default()
    };
    globs
        .patterns
        .iter()
        .map(|pattern| globs.root.join(pattern))
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

    const TESTDATA_PATH: &str = "src/testdata";

    #[test]
    fn find_existing_path() {
        assert_eq!(
            find(&Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: &["maintainers*"],
                case_sensitive: false,
            })
            .unwrap(),
            Some(PathBuf::from("MAINTAINERS"))
        );
    }

    #[test]
    fn find_non_existing_path() {
        assert_eq!(
            find(&Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: &["nonexisting"],
                case_sensitive: false,
            })
            .unwrap(),
            None
        );
    }

    #[test]
    fn find_invalid_glob_pattern() {
        assert!(find(&Globs {
            root: Path::new(TESTDATA_PATH),
            patterns: &["invalid***"],
            case_sensitive: false,
        })
        .is_err());
    }

    #[test]
    fn matches_case_insensitive_found() {
        let testdata = Path::new(TESTDATA_PATH);

        assert_eq!(
            matches(&Globs {
                root: testdata,
                patterns: &["maintainers*"],
                case_sensitive: false,
            })
            .unwrap(),
            vec![testdata.join("MAINTAINERS")]
        );
    }

    #[test]
    fn matches_case_sensitive_found() {
        let testdata = Path::new(TESTDATA_PATH);

        assert_eq!(
            matches(&Globs {
                root: testdata,
                patterns: &["OWNERS*"],
                case_sensitive: true,
            })
            .unwrap(),
            vec![testdata.join("OWNERS")]
        );
    }

    #[test]
    fn matches_not_found() {
        assert_eq!(
            matches(&Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: &["nonexisting"],
                case_sensitive: false,
            })
            .unwrap(),
            Vec::<PathBuf>::new()
        );
    }

    #[test]
    fn matches_invalid_glob_pattern() {
        assert!(matches(&Globs {
            root: Path::new(TESTDATA_PATH),
            patterns: &["invalid***"],
            case_sensitive: true,
        })
        .is_err());
    }
}
