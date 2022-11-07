use super::path::{self, Globs};
use anyhow::Result;
use regex::{Regex, RegexSet};
use std::fs;

/// Check if the content of any of the files that match the globs provided
/// matches any of the regular expressions given, returning the captured value
/// when there is a match. This function expects that the regular expressions
/// provided contain one capture group.
pub(crate) fn find(globs: &Globs, regexps: &[&Regex]) -> Result<Option<String>> {
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
pub(crate) fn matches(globs: &Globs, re: &RegexSet) -> Result<bool> {
    Ok(path::matches(globs)?.iter().any(|path| {
        if let Ok(content) = fs::read_to_string(path) {
            return re.is_match(&content);
        }
        false
    }))
}

/// Check if the content of the url provided matches any of the regular
/// expressions given.
pub(crate) async fn remote_matches(
    http_client: &reqwest::Client,
    url: &str,
    re: &RegexSet,
) -> Result<bool> {
    let content = http_client.get(url).send().await?.text().await?;
    Ok(re.is_match(&content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linter::checks::{
        adopters,
        license_scanning::{FOSSA_URL, SNYK_URL},
        readme,
    };
    use std::path::Path;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    const TESTDATA_PATH: &str = "src/testdata";

    #[test]
    fn find_found() {
        assert_eq!(
            find(
                &Globs {
                    root: Path::new(TESTDATA_PATH),
                    patterns: &readme::FILE_PATTERNS,
                    case_sensitive: true,
                },
                &[&FOSSA_URL, &SNYK_URL]
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
                &Globs {
                    root: Path::new(TESTDATA_PATH),
                    patterns: &readme::FILE_PATTERNS,
                    case_sensitive: true,
                },
                &[&Regex::new("non-existing pattern").unwrap()]
            )
            .unwrap(),
            None
        );
    }

    #[test]
    fn find_file_not_found() {
        assert_eq!(
            find(
                &Globs {
                    root: Path::new(TESTDATA_PATH),
                    patterns: &["nonexisting"],
                    case_sensitive: true,
                },
                &[&Regex::new("pattern").unwrap()]
            )
            .unwrap(),
            None
        );
    }

    #[test]
    fn find_invalid_glob_pattern() {
        assert!(matches!(
            find(
                &Globs {
                    root: Path::new(TESTDATA_PATH),
                    patterns: &["invalid***"],
                    case_sensitive: true,
                },
                &[&Regex::new("pattern").unwrap()]
            ),
            Err(_)
        ));
    }

    #[test]
    fn matches_match() {
        assert!(matches(
            &Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: &readme::FILE_PATTERNS,
                case_sensitive: true,
            },
            &adopters::README_REF
        )
        .unwrap());
    }

    #[test]
    fn matches_no_match() {
        assert!(!matches(
            &Globs {
                root: Path::new(TESTDATA_PATH),
                patterns: &readme::FILE_PATTERNS,
                case_sensitive: true,
            },
            &RegexSet::new(["non-existing pattern"]).unwrap(),
        )
        .unwrap());
    }

    #[test]
    fn matches_file_not_found() {
        assert!(!matches(
            &Globs {
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
                &Globs {
                    root: Path::new(TESTDATA_PATH),
                    patterns: &["invalid***"],
                    case_sensitive: true,
                },
                &RegexSet::new(["pattern"]).unwrap(),
            ),
            Err(_)
        ));
    }

    #[tokio::test]
    async fn remote_matches_match() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_string("sample data"))
            .expect(1)
            .named("root GET")
            .mount(&mock_server)
            .await;

        assert!(remote_matches(
            &reqwest::Client::new(),
            &mock_server.uri(),
            &RegexSet::new(["data"]).unwrap(),
        )
        .await
        .unwrap());
    }

    #[tokio::test]
    async fn remote_matches_no_match() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_string("sample data"))
            .expect(1)
            .named("root GET")
            .mount(&mock_server)
            .await;

        assert!(!remote_matches(
            &reqwest::Client::new(),
            &mock_server.uri(),
            &RegexSet::new(["notfound"]).unwrap(),
        )
        .await
        .unwrap());
    }

    #[tokio::test]
    async fn remote_matches_request_failed() {
        assert!(matches!(
            remote_matches(
                &reqwest::Client::new(),
                "http://localhost:0",
                &RegexSet::new(["data"]).unwrap(),
            )
            .await,
            Err(_)
        ));
    }
}
