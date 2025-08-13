use std::sync::LazyLock;

use anyhow::Result;
use regex::RegexSet;

use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};

use super::util::content;

/// Check identifier.
pub(crate) const ID: CheckId = "trademark_disclaimer";

/// Check score weight.
pub(crate) const WEIGHT: usize = 5;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Community];

pub(crate) static TRADEMARK_DISCLAIMER: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        r"https://(?:w{3}\.)?linuxfoundation.org/(?:legal/)?trademark-usage",
        r"The Linux Foundation.* has registered trademarks and uses trademarks",
        r"Copyright © .+ a Series of LF Projects, LLC",
    ])
    .expect("exprs in TRADEMARK_DISCLAIMER to be valid")
});

/// Check main function.
pub(crate) async fn check(input: &CheckInput<'_>) -> Result<CheckOutput> {
    // Trademark disclaimer in website setup in Github
    if let Some(url) = &input.gh_md.homepage_url
        && !url.is_empty()
        && content::remote_matches(url, &TRADEMARK_DISCLAIMER).await?
    {
        return Ok(CheckOutput::passed());
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trademark_disclaimer_match() {
        assert!(TRADEMARK_DISCLAIMER.is_match("https://www.linuxfoundation.org/trademark-usage"));
        assert!(
            TRADEMARK_DISCLAIMER.is_match("https://www.linuxfoundation.org/legal/trademark-usage")
        );
        assert!(TRADEMARK_DISCLAIMER.is_match("https://linuxfoundation.org/trademark-usage"));
        assert!(TRADEMARK_DISCLAIMER.is_match("https://linuxfoundation.org/legal/trademark-usage"));
        assert!(TRADEMARK_DISCLAIMER.is_match(
            "The Linux Foundation® (TLF) has registered trademarks and uses trademarks."
        ));
        assert!(
            TRADEMARK_DISCLAIMER.is_match("Copyright © Kubernetes a Series of LF Projects, LLC")
        );
    }
}
