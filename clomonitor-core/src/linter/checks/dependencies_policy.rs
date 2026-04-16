use anyhow::{Result, format_err};

use crate::linter::{CheckId, CheckOutput, CheckSet, check::CheckInput};

/// Check identifier.
pub(crate) const ID: CheckId = "dependencies_policy";

/// Check score weight.
pub(crate) const WEIGHT: usize = 2;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    if let Some(policy_url) = input
        .security_insights
        .as_ref()
        .map_err(|e| format_err!("{e:?}"))?
        .as_ref()
        .and_then(|manifest| manifest.dependencies_policy_url())
    {
        return Ok(CheckOutput::passed().url(Some(policy_url.to_string())));
    }
    Ok(CheckOutput::not_passed())
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
    fn not_passed_when_policy_url_is_missing() {
        let output = check(&CheckInput {
            li: &LinterInput::default(),
            cm_md: None,
            gh_md: MdRepository::default(),
            scorecard: Err(format_err!("no scorecard available")),
            security_insights: SecurityInsights::new(
                &Path::new("src/testdata/security-insights-v2/invalid-no-policy")
                    .canonicalize()
                    .unwrap(),
            ),
        })
        .unwrap();

        assert_eq!(output, CheckOutput::not_passed());
    }

    #[test]
    fn passed_when_policy_url_is_available_in_v1() {
        let output = check(&CheckInput {
            li: &LinterInput::default(),
            cm_md: None,
            gh_md: MdRepository::default(),
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
                "https://example.com/v1/dependencies-policy".to_string()
            ))
        );
    }

    #[test]
    fn passed_when_policy_url_is_available_in_v2() {
        let output = check(&CheckInput {
            li: &LinterInput::default(),
            cm_md: None,
            gh_md: MdRepository::default(),
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
                "https://example.com/v2/dependency-management-policy".to_string(),
            ))
        );
    }
}
