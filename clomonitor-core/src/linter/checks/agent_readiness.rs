use anyhow::Result;

use crate::linter::{
    CheckSet,
    check::{CheckId, CheckInput, CheckOutput},
};

use super::{
    datasource::github,
    util::path::{self, Globs},
};

/// Check identifier.
pub(crate) const ID: CheckId = "agent_readiness";

/// Check score weight.
pub(crate) const WEIGHT: usize = 1;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 2] = [CheckSet::Community, CheckSet::Docs];

/// Names of the files probed on the project's website.
const WEBSITE_FILES: [&str; 2] = ["llms.txt", "llms-full.txt"];

/// Patterns used to locate a file in the repository.
const FILE_PATTERNS: [&str; 4] = [
    "llms.txt",
    "llms-full.txt",
    "docs/llms.txt",
    "docs/llms-full.txt",
];

/// Check main function.
pub(crate) async fn check(input: &CheckInput<'_>) -> Result<CheckOutput> {
    // llms.txt (or llms-full.txt) file in project's website
    if let Some(url) = &input.gh_md.homepage_url
        && !url.is_empty()
    {
        for file_name in WEBSITE_FILES {
            let file_url = format!("{}/{}", url.trim_end_matches('/'), file_name);
            if let Ok(response) = reqwest::get(&file_url).await
                && response.status().is_success()
                && let Ok(content) = response.text().await
                && is_llms_txt_content(&content)
            {
                return Ok(CheckOutput::passed().url(Some(file_url)));
            }
        }
    }

    // llms.txt (or llms-full.txt) file in repository
    if let Some(path) = path::find(&Globs {
        root: &input.li.root,
        patterns: &FILE_PATTERNS,
        case_sensitive: false,
    })? {
        let url = github::build_url(
            &path,
            &input.gh_md.owner.login,
            &input.gh_md.name,
            &github::default_branch(input.gh_md.default_branch_ref.as_ref()),
        );
        return Ok(CheckOutput::passed().url(Some(url)));
    }

    Ok(CheckOutput::not_passed())
}

/// Check if the content provided looks like an llms.txt file. Some websites
/// return an HTML page (e.g. a SPA fallback page) with a 200 status code for
/// any path requested, so we make sure the content isn't an HTML document.
fn is_llms_txt_content(content: &str) -> bool {
    const HTML_MARKERS: [&str; 4] = ["<!doctype", "<html", "<head", "<body"];

    let content = content.trim();
    if content.is_empty() {
        return false;
    }
    let start: String = content.chars().take(16).collect::<String>().to_lowercase();
    !HTML_MARKERS.iter().any(|marker| start.starts_with(marker))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use anyhow::format_err;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    use crate::linter::{
        LinterInput,
        datasource::github::md::{MdRepository, MdRepositoryOwner, MdRepositoryOwnerOn},
    };

    use super::*;

    const TESTDATA_PATH: &str = "src/testdata/agent-readiness";

    const LLMS_TXT: &str = "# Project\n\n> Docs index for AI agents.\n\n## Docs\n\n- [Getting started](https://example.test/docs/getting-started.md)\n";

    fn check_input(root: &str, gh_md: MdRepository) -> (LinterInput, MdRepository) {
        (
            LinterInput {
                root: PathBuf::from(root),
                ..LinterInput::default()
            },
            gh_md,
        )
    }

    macro_rules! new_check_input {
        ($li:expr, $gh_md:expr) => {
            CheckInput {
                li: &$li,
                cm_md: None,
                gh_md: $gh_md,
                scorecard: Err(format_err!("no scorecard available")),
                security_insights: Ok(None),
            }
        };
    }

    #[test]
    fn is_llms_txt_content_valid() {
        assert!(is_llms_txt_content(LLMS_TXT));
        assert!(is_llms_txt_content("# Title only"));
        assert!(is_llms_txt_content("plain text index"));
    }

    #[test]
    fn is_llms_txt_content_empty() {
        assert!(!is_llms_txt_content(""));
        assert!(!is_llms_txt_content("   \n\t  "));
    }

    #[test]
    fn is_llms_txt_content_html() {
        assert!(!is_llms_txt_content(
            "<!DOCTYPE html><html><body>404</body></html>"
        ));
        assert!(!is_llms_txt_content("<!doctype html>"));
        assert!(!is_llms_txt_content(
            "<html lang=\"en\"><head></head></html>"
        ));
        assert!(!is_llms_txt_content(
            "\n  <HTML><BODY>SPA shell</BODY></HTML>"
        ));
        assert!(!is_llms_txt_content(
            "<head><title>Not found</title></head>"
        ));
        assert!(!is_llms_txt_content("<body>fallback</body>"));
    }

    #[tokio::test]
    async fn check_passed_llms_txt_found_in_website() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/llms.txt"))
            .respond_with(ResponseTemplate::new(200).set_body_string(LLMS_TXT))
            .mount(&mock_server)
            .await;

        let (li, gh_md) = check_input(
            TESTDATA_PATH,
            MdRepository {
                homepage_url: Some(mock_server.uri()),
                ..MdRepository::default()
            },
        );
        let input = new_check_input!(li, gh_md);

        assert_eq!(
            check(&input).await.unwrap(),
            CheckOutput::passed().url(Some(format!("{}/llms.txt", mock_server.uri()))),
        );
    }

    #[tokio::test]
    async fn check_passed_llms_full_txt_found_in_website() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/llms.txt"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;
        Mock::given(method("GET"))
            .and(path("/llms-full.txt"))
            .respond_with(ResponseTemplate::new(200).set_body_string(LLMS_TXT))
            .mount(&mock_server)
            .await;

        let (li, gh_md) = check_input(
            "src/testdata",
            MdRepository {
                homepage_url: Some(format!("{}/", mock_server.uri())),
                ..MdRepository::default()
            },
        );
        let input = new_check_input!(li, gh_md);

        assert_eq!(
            check(&input).await.unwrap(),
            CheckOutput::passed().url(Some(format!("{}/llms-full.txt", mock_server.uri()))),
        );
    }

    #[tokio::test]
    async fn check_passed_file_found_in_repo_when_website_serves_html_fallback() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("<!DOCTYPE html><html><body>SPA</body></html>"),
            )
            .mount(&mock_server)
            .await;

        let (li, gh_md) = check_input(
            TESTDATA_PATH,
            MdRepository {
                name: "repo".to_string(),
                owner: MdRepositoryOwner {
                    login: "owner".to_string(),
                    on: MdRepositoryOwnerOn::Organization,
                },
                homepage_url: Some(mock_server.uri()),
                ..MdRepository::default()
            },
        );
        let input = new_check_input!(li, gh_md);

        assert_eq!(
            check(&input).await.unwrap(),
            CheckOutput::passed().url(Some(
                "https://github.com/owner/repo/blob/master/llms.txt".to_string()
            )),
        );
    }

    #[tokio::test]
    async fn check_passed_file_found_in_repo_no_website() {
        let (li, gh_md) = check_input(
            TESTDATA_PATH,
            MdRepository {
                name: "repo".to_string(),
                owner: MdRepositoryOwner {
                    login: "owner".to_string(),
                    on: MdRepositoryOwnerOn::Organization,
                },
                ..MdRepository::default()
            },
        );
        let input = new_check_input!(li, gh_md);

        assert_eq!(
            check(&input).await.unwrap(),
            CheckOutput::passed().url(Some(
                "https://github.com/owner/repo/blob/master/llms.txt".to_string()
            )),
        );
    }

    #[tokio::test]
    async fn check_not_passed() {
        let (li, gh_md) = check_input("src/testdata", MdRepository::default());
        let input = new_check_input!(li, gh_md);

        assert_eq!(check(&input).await.unwrap(), CheckOutput::not_passed());
    }
}
