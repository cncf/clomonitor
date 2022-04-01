use super::path::{self, Globs};
use anyhow::Error;
use lazy_static::lazy_static;
use regex::{Regex, RegexSet};
use std::fs;

/// Check if the content of any of the files that match the globs provided
/// matches any of the regular expressions given, returning the captured value
/// when there is a match. This function expects that the regular expressions
/// provided contain one capture group.
pub(crate) fn find(globs: Globs, regexps: Vec<&Regex>) -> Result<Option<String>, Error> {
    for path in path::matches(globs)?.iter() {
        if let Ok(content) = fs::read_to_string(path) {
            for re in regexps.iter() {
                if let Some(c) = re.captures(&content) {
                    if c.len() > 1 {
                        return Ok(Some(c[1].to_string()));
                    }
                }
            }
        }
    }
    Ok(None)
}

/// Check if the content of any of the files that match the globs provided
/// matches any of the regular expressions given.
pub(crate) fn matches(globs: Globs, re: &RegexSet) -> Result<bool, Error> {
    Ok(path::matches(globs)?.iter().any(|path| {
        if let Ok(content) = fs::read_to_string(path) {
            return re.is_match(&content);
        }
        false
    }))
}

/// Check if the content of the url provided matches any of the regular
/// expressions given.
pub(crate) async fn remote_matches(url: &str, re: &RegexSet) -> Result<bool, Error> {
    lazy_static! {
        static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
    }
    let content = HTTP_CLIENT.get(url).send().await?.text().await?;
    Ok(re.is_match(&content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linter::check::patterns::{ADOPTERS_IN_README, FOSSA_URL, README_FILE, SNYK_URL};
    use std::path::Path;

    const TESTDATA_PATH: &str = "src/linter/check/testdata";

    #[test]
    fn find_found() {
        assert_eq!(
            find(
                Globs {
                    root: Path::new(TESTDATA_PATH),
                    patterns: &README_FILE,
                    case_sensitive: true,
                },
                vec![&*FOSSA_URL, &*SNYK_URL]
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
                    patterns: &README_FILE,
                    case_sensitive: true,
                },
                vec![&Regex::new("non-existing pattern").unwrap()]
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
                    patterns: &["nonexisting"],
                    case_sensitive: true,
                },
                vec![&Regex::new("pattern").unwrap()]
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
                    patterns: &["invalid***"],
                    case_sensitive: true,
                },
                vec![&Regex::new("pattern").unwrap()]
            ),
            Err(_)
        ));
    }

    #[test]
    fn matches_match() {
        assert!(matches(
            Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: &README_FILE,
                case_sensitive: true,
            },
            &*ADOPTERS_IN_README
        )
        .unwrap());
    }

    #[test]
    fn matches_no_match() {
        assert!(!matches(
            Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: &README_FILE,
                case_sensitive: true,
            },
            &RegexSet::new(["non-existing pattern"]).unwrap(),
        )
        .unwrap());
    }

    #[test]
    fn matches_file_not_found() {
        assert!(!matches(
            Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: &["nonexisting"],
                case_sensitive: true,
            },
            &RegexSet::new(["pattern"]).unwrap(),
        )
        .unwrap());
    }

    #[test]
    fn matches_invalid_glob_pattern() {
        assert!(matches!(
            matches(
                Globs {
                    root: Path::new(TESTDATA_PATH),
                    patterns: &["invalid***"],
                    case_sensitive: true,
                },
                &RegexSet::new(["pattern"]).unwrap(),
            ),
            Err(_)
        ));
    }
}
