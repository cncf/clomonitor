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

/// Check if exists at least a path that matches the globs provided.
pub(crate) fn exists<P>(globs: Globs<P>) -> Result<bool, PatternError>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
{
    Ok(!matches(globs)?.is_empty())
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
    use crate::linter::patterns::*;

    const TESTDATA_PATH: &str = "src/linter/check/testdata";

    #[test]
    fn exists_existing_path() {
        assert!(exists(Globs {
            root: Path::new(TESTDATA_PATH),
            patterns: MAINTAINERS_FILE,
            case_sensitive: false,
        })
        .unwrap());
    }

    #[test]
    fn exists_non_existing_path() {
        assert!(!exists(Globs {
            root: Path::new(TESTDATA_PATH),
            patterns: vec!["nonexisting"],
            case_sensitive: false,
        })
        .unwrap());
    }

    #[test]
    fn exists_invalid_glob_pattern() {
        assert!(matches!(
            exists(Globs {
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
