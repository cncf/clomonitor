use anyhow::{Result, format_err};

use crate::linter::{CheckId, CheckOutput, CheckSet, check::CheckInput};

use super::datasource::github;

/// Check identifier.
pub(crate) const ID: CheckId = "security_insights";

/// Check score weight.
pub(crate) const WEIGHT: usize = 1;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

/// Check main function.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    let output = match input
        .security_insights
        .as_ref()
        .map_err(|e| format_err!("{e:?}"))?
    {
        Some(manifest) => {
            let url = github::build_url(
                manifest.manifest_rel_path(),
                &input.gh_md.owner.login,
                &input.gh_md.name,
                &github::default_branch(input.gh_md.default_branch_ref.as_ref()),
            );
            CheckOutput::passed().url(Some(url))
        }
        None => CheckOutput::not_passed(),
    };
    Ok(output)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::format_err;

    use crate::linter::{
        LinterInput,
        datasource::{github::md::MdRepository, security_insights::SecurityInsights},
    };

    use super::*;

    #[test]
    fn check_passes_with_v1_manifest() {
        let output = check(&CheckInput {
            li: &LinterInput::default(),
            cm_md: None,
            gh_md: github_metadata(),
            scorecard: Err(format_err!("no scorecard available")),
            security_insights: SecurityInsights::new(
                &Path::new("src/testdata/security-insights-v1/root")
                    .canonicalize()
                    .unwrap(),
            ),
        })
        .unwrap();

        assert_eq!(
            output,
            CheckOutput::passed().url(Some(
                "https://github.com/org/repo/blob/main/SECURITY-INSIGHTS.yml".to_string(),
            ))
        );
    }

    #[test]
    fn check_passes_with_v2_github_manifest() {
        let output = check(&CheckInput {
            li: &LinterInput::default(),
            cm_md: None,
            gh_md: github_metadata(),
            scorecard: Err(format_err!("no scorecard available")),
            security_insights: SecurityInsights::new(
                &Path::new("src/testdata/security-insights-v2/github")
                    .canonicalize()
                    .unwrap(),
            ),
        })
        .unwrap();

        assert_eq!(
            output,
            CheckOutput::passed().url(Some(
                "https://github.com/org/repo/blob/main/.github/security-insights.yml".to_string(),
            ))
        );
    }

    #[test]
    fn check_passes_with_v2_root_manifest() {
        let output = check(&CheckInput {
            li: &LinterInput::default(),
            cm_md: None,
            gh_md: github_metadata(),
            scorecard: Err(format_err!("no scorecard available")),
            security_insights: SecurityInsights::new(
                &Path::new("src/testdata/security-insights-v2/root")
                    .canonicalize()
                    .unwrap(),
            ),
        })
        .unwrap();

        assert_eq!(
            output,
            CheckOutput::passed().url(Some(
                "https://github.com/org/repo/blob/main/security-insights.yml".to_string(),
            ))
        );
    }

    // Helpers.

    fn github_metadata() -> MdRepository {
        MdRepository {
            default_branch_ref: Some(super::github::md::MdRepositoryDefaultBranchRef {
                name: "main".to_string(),
            }),
            name: "repo".to_string(),
            owner: super::github::md::MdRepositoryOwner {
                login: "org".to_string(),
                on: super::github::md::MdRepositoryOwnerOn::Organization,
            },
            ..MdRepository::default()
        }
    }
}
