use anyhow::Error;
use glob::{glob_with, MatchOptions, PatternError};
use std::path::{Path, PathBuf};

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

/// Find the first path that matches any of the globs provided.
pub(crate) fn find<P>(globs: Globs<P>) -> Result<Option<PathBuf>, Error>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
{
    let root = globs.root.to_owned();
    match matches(globs)?.first() {
        Some(path) => Ok(Some(path.strip_prefix(root)?.to_owned())),
        None => Ok(None),
    }
}

/// Return all paths that match any of the globs provided.
pub(crate) fn matches<P>(globs: Globs<P>) -> Result<Vec<PathBuf>, PatternError>
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
    use crate::linter::check::patterns::*;

    const TESTDATA_PATH: &str = "src/linter/check/testdata";

    #[test]
    fn find_existing_path() {
        assert_eq!(
            find(Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: MAINTAINERS_FILE,
                case_sensitive: false,
            })
            .unwrap(),
            Some(PathBuf::from("MAINTAINERS"))
        );
    }

    #[test]
    fn find_non_existing_path() {
        assert_eq!(
            find(Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: vec!["nonexisting"],
                case_sensitive: false,
            })
            .unwrap(),
            None
        );
    }

    #[test]
    fn find_invalid_glob_pattern() {
        assert!(matches!(
            find(Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: vec!["invalid***"],
                case_sensitive: false,
            }),
            Err(_)
        ));
    }

    #[test]
    fn matches_case_insensitive_found() {
        let testdata = Path::new(TESTDATA_PATH);

        assert_eq!(
            matches(Globs {
                root: testdata,
                patterns: MAINTAINERS_FILE,
                case_sensitive: false,
            })
            .unwrap(),
            vec![testdata.join("MAINTAINERS"), testdata.join("OWNERS"),]
        );
    }

    #[test]
    fn matches_case_sensitive_found() {
        let testdata = Path::new(TESTDATA_PATH);

        assert_eq!(
            matches(Globs {
                root: testdata,
                patterns: ["OWNERS*"],
                case_sensitive: true,
            })
            .unwrap(),
            vec![testdata.join("OWNERS")]
        );
    }

    #[test]
    fn matches_not_found() {
        assert_eq!(
            matches(Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: vec!["nonexisting"],
                case_sensitive: false,
            })
            .unwrap(),
            Vec::<PathBuf>::new()
        );
    }

    #[test]
    fn matches_invalid_glob_pattern() {
        assert!(matches!(
            matches(Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: vec!["invalid***"],
                case_sensitive: true,
            }),
            Err(_)
        ));
    }
}
