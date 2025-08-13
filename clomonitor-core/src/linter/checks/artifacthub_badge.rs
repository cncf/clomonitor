use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;

use crate::linter::{
    CheckSet,
    check::{CheckId, CheckInput, CheckOutput},
};

use super::util::helpers::readme_capture;

/// Check identifier.
pub(crate) const ID: CheckId = "artifacthub_badge";

/// Check score weight.
pub(crate) const WEIGHT: usize = 1;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

static ARTIFACTHUB_URL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(https://artifacthub.io/packages/[^"'\)]+)"#)
        .expect("exprs in ARTIFACTHUB_URL to be valid")
});

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // Reference in README file
    let url = readme_capture(&input.li.root, &[&ARTIFACTHUB_URL])?;
    if url.is_some() {
        return Ok(CheckOutput::passed().url(url));
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifacthub_url_extract() {
        assert_eq!(
            ARTIFACTHUB_URL.captures(r#"[![Artifact HUB]("https://img.shields.io/endpoint?url=https://artifacthub.io/badge/repository/artifact-hub)](https://artifacthub.io/packages/helm/artifact-hub/artifact-hub)"#).unwrap()[1].to_string(),
            "https://artifacthub.io/packages/helm/artifact-hub/artifact-hub"
        );
    }
}
