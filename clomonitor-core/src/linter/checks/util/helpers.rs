use super::{
    content,
    path::{self, Globs},
};
use crate::linter::{
    check::{CheckInput, CheckOutput},
    checks::readme,
    datasource::github,
    metadata::{Exemption, Metadata},
    CheckSet, CHECKS,
};
use anyhow::Result;
use regex::{Regex, RegexSet};
use std::path::Path;

/// Check if a file matching the patterns provided is found in the repo or if
/// any of the regular expressions provided matches the README file content.
pub(crate) fn find_file_or_readme_ref(
    input: &CheckInput,
    patterns: &[&str],
    re: &RegexSet,
) -> Result<CheckOutput> {
    // File in repo
    if let Some(path) = path::find(&Globs {
        root: &input.li.root,
        patterns,
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

    // Reference in README file
    if readme_matches(&input.li.root, re)? {
        return Ok(CheckOutput::passed());
    }

    Ok(CheckOutput::not_passed())
}

/// Check if the README file content matches any of the regular expressions
/// provided.
pub(crate) fn readme_matches(root: &Path, re: &RegexSet) -> Result<bool> {
    content::matches(&readme_globs(root), re)
}

/// Check if the README file content matches any of the regular expressions
/// provided, returning the value from the first capture group.
pub(crate) fn readme_capture(root: &Path, regexps: &[&Regex]) -> Result<Option<String>> {
    content::find(&readme_globs(root), regexps)
}

// Returns a Globs instance used to locate the README file.
pub(crate) fn readme_globs(root: &Path) -> Globs {
    Globs {
        root,
        patterns: &readme::FILE_PATTERNS,
        case_sensitive: true,
    }
}

/// Check if the repository is exempt from passing the provided check.
pub(crate) fn find_exemption(check_id: &str, cm_md: Option<&Metadata>) -> Option<Exemption> {
    if let Some(exemption) = cm_md
        .as_ref()
        .and_then(|md| md.exemptions.as_ref())
        .and_then(|exemptions| {
            exemptions
                .iter()
                .find(|exemption| exemption.check == check_id)
        })
    {
        if !exemption.reason.is_empty() {
            return Some(exemption.clone());
        }
    }

    None
}

/// Check if the check provided should be skipped.
pub(crate) fn should_skip_check(check_id: &str, check_sets: &[CheckSet]) -> bool {
    // Skip if the check doesn't belong to any of the check sets provided
    if !CHECKS[check_id]
        .check_sets
        .iter()
        .any(|check_set| check_sets.contains(check_set))
    {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linter::{
        adopters,
        datasource::github::md::{MdRepository, MdRepositoryOwner, MdRepositoryOwnerOn},
        sbom, LinterInput,
    };
    use anyhow::format_err;
    use std::path::PathBuf;

    const TESTDATA_PATH: &str = "src/testdata";

    #[test]
    fn find_file_or_readme_ref_file_found() {
        assert_eq!(
            find_file_or_readme_ref(
                &CheckInput {
                    li: &LinterInput {
                        root: PathBuf::from(TESTDATA_PATH),
                        ..LinterInput::default()
                    },
                    cm_md: None,
                    gh_md: MdRepository {
                        name: "repo".to_string(),
                        owner: MdRepositoryOwner {
                            login: "owner".to_string(),
                            on: MdRepositoryOwnerOn::Organization,
                        },
                        ..MdRepository::default()
                    },
                    scorecard: Err(format_err!("no scorecard available")),
                    security_insights: Ok(None),
                },
                &["README*"],
                &RegexSet::new(["nothing"]).unwrap(),
            )
            .unwrap(),
            CheckOutput::passed().url(Some(
                "https://github.com/owner/repo/blob/master/README.md".to_string()
            )),
        );
    }

    #[test]
    fn find_file_or_readme_ref_ref_found() {
        assert_eq!(
            find_file_or_readme_ref(
                &CheckInput {
                    li: &LinterInput {
                        root: PathBuf::from(TESTDATA_PATH),
                        ..LinterInput::default()
                    },
                    cm_md: None,
                    gh_md: MdRepository::default(),
                    scorecard: Err(format_err!("no scorecard available")),
                    security_insights: Ok(None),
                },
                &["ADOPTERS*"],
                &RegexSet::new([r"(?im)^#+.*adopters.*$"]).unwrap(),
            )
            .unwrap(),
            CheckOutput::passed(),
        );
    }

    #[test]
    fn find_file_or_readme_ref_not_found() {
        assert_eq!(
            find_file_or_readme_ref(
                &CheckInput {
                    li: &LinterInput {
                        root: PathBuf::from(TESTDATA_PATH),
                        ..LinterInput::default()
                    },
                    cm_md: None,
                    gh_md: MdRepository::default(),
                    scorecard: Err(format_err!("no scorecard available")),
                    security_insights: Ok(None),
                },
                &["inexistent_file*"],
                &RegexSet::new(["inexistent_ref"]).unwrap(),
            )
            .unwrap(),
            CheckOutput::not_passed(),
        );
    }

    #[test]
    fn find_exemption_found() {
        assert_eq!(
            find_exemption(
                "check-id",
                Some(&Metadata {
                    exemptions: Some(vec![Exemption {
                        check: "check-id".to_string(),
                        reason: "sample reason".to_string(),
                    }]),
                    license_scanning: None
                })
            ),
            Some(Exemption {
                check: "check-id".to_string(),
                reason: "sample reason".to_string(),
            }),
        );
    }

    #[test]
    fn find_exemption_not_found_in_md() {
        assert_eq!(
            find_exemption(
                "not-found",
                Some(&Metadata {
                    exemptions: Some(vec![Exemption {
                        check: "check-id".to_string(),
                        reason: "sample reason".to_string(),
                    }]),
                    license_scanning: None
                })
            ),
            None,
        );
    }

    #[test]
    fn find_exemption_not_found_no_exemptions_in_md() {
        assert_eq!(
            find_exemption(
                "check-id",
                Some(&Metadata {
                    exemptions: None,
                    license_scanning: None
                })
            ),
            None,
        );
    }

    #[test]
    fn find_exemption_not_found_no_md() {
        assert_eq!(find_exemption("check-id", None), None,);
    }

    #[test]
    fn should_skip_check_affirmative() {
        assert!(should_skip_check(adopters::ID, &[CheckSet::Code]));
        assert!(should_skip_check(sbom::ID, &[CheckSet::Community]));
    }

    #[test]
    fn should_skip_check_negative() {
        assert!(!should_skip_check(
            adopters::ID,
            &[CheckSet::Code, CheckSet::Community]
        ));
        assert!(!should_skip_check(
            sbom::ID,
            &[CheckSet::Code, CheckSet::Community]
        ));
    }
}
