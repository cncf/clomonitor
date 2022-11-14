use super::util::{github, helpers::find_file_or_readme_ref};
use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::RegexSet;

/// Check identifier.
pub(crate) const ID: CheckId = "contributing";

/// Check score weight.
pub(crate) const WEIGHT: usize = 4;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 3] =
    [CheckSet::Code, CheckSet::CodeLite, CheckSet::Community];

/// Patterns used to locate a file in the repository.
const FILE_PATTERNS: [&str; 3] = [
    "contributing*",
    ".github/contributing*",
    "docs/contributing*",
];

lazy_static! {
    #[rustfmt::skip]
    static ref README_REF: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*contributing.*$",
        r"(?im)^contributing$",
        r"(?i)\[.*contributing.*\]\(.*\)",
    ]).expect("exprs in README_REF to be valid");
}

/// Check main function.
pub(crate) async fn check(input: &CheckInput<'_>) -> Result<CheckOutput> {
    // File in repo or reference in README file
    let r = find_file_or_readme_ref(input, &FILE_PATTERNS, &README_REF)?;
    if r.passed {
        return Ok(r);
    }

    // File in .github repo
    if let Some(url) =
        github::has_community_health_file(&input.svc.http_client, "CONTRIBUTING.md", &input.gh_md)
            .await?
    {
        return Ok(CheckOutput::passed().url(Some(url)));
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_ref_match() {
        assert!(README_REF.is_match("# Contributing"));
        assert!(README_REF.is_match(
            r"
...
## Some stuff, contributing and others
...
            "
        ));
        assert!(README_REF.is_match(
            r"
...
Contributing
------------
...
            "
        ));
        assert!(README_REF.is_match("[Project contributing](...)"));
    }
}
