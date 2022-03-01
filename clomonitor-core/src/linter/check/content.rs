use super::path::{self, Globs};
use anyhow::Error;
use regex::{Regex, RegexSet};
use reqwest;
use std::fs;

/// Check if the content of any of the files that match the globs provided
/// matches any of the regular expressions given, returning the captured value
/// when there is a match. This function expects that the regular expressions
/// provided contain one capture group.
pub(crate) fn find<P, R>(globs: Globs<P>, regexps: R) -> Result<Option<String>, Error>
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
    for path in path::matches(globs)?.iter() {
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
pub(crate) fn matches<P, R>(globs: Globs<P>, regexps: R) -> Result<bool, Error>
where
    P: IntoIterator,
    P::Item: AsRef<str>,
    R: IntoIterator,
    R::Item: AsRef<str>,
{
    let re = RegexSet::new(regexps)?;
    Ok(path::matches(globs)?.iter().any(|path| {
        if let Ok(content) = fs::read_to_string(path) {
            return re.is_match(&content);
        }
        false
    }))
}

/// Check if the content of the url provided matches any of the regular
/// expressions given.
pub(crate) async fn remote_matches<R>(url: &str, regexps: R) -> Result<bool, Error>
where
    R: IntoIterator,
    R::Item: AsRef<str>,
{
    let content = reqwest::get(url).await?.text().await?;
    let re = RegexSet::new(regexps)?;
    Ok(re.is_match(&content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linter::patterns::*;
    use std::path::Path;

    const TESTDATA_PATH: &str = "src/linter/check/testdata";

    #[test]
    fn find_found() {
        assert_eq!(
            find(
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
    fn find_not_found() {
        assert_eq!(
            find(
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
    fn find_file_not_found() {
        assert_eq!(
            find(
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
    fn find_invalid_glob_pattern() {
        assert!(matches!(
            find(
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
    fn find_invalid_regexp() {
        assert!(matches!(
            find(
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
    fn matches_match() {
        assert!(matches(
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
    fn matches_no_match() {
        assert!(!matches(
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
    fn matches_file_not_found() {
        assert!(!matches(
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
    fn matches_invalid_glob_pattern() {
        assert!(matches!(
            matches(
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
    fn matches_invalid_regexp() {
        assert!(matches!(
            matches(
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
}
