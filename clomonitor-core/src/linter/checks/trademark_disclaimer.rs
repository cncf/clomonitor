use super::util::content;
use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::RegexSet;

/// Check identifier.
pub(crate) const ID: CheckId = "trademark_disclaimer";

/// Check score weight.
pub(crate) const WEIGHT: usize = 5;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Community];

lazy_static! {
    #[rustfmt::skip]
    pub(crate) static ref TRADEMARK_DISCLAIMER: RegexSet = RegexSet::new(vec![
        r"https://(?:w{3}\.)?linuxfoundation.org/trademark-usage",
        r"The Linux Foundation.* has registered trademarks and uses trademarks",
    ]).expect("exprs in TRADEMARK_DISCLAIMER to be valid");
}

/// Check main function.
pub(crate) async fn check(input: &CheckInput<'_>) -> Result<CheckOutput> {
    // Trademark disclaimer in website setup in Github
    if let Some(url) = &input.gh_md.homepage_url {
        if !url.is_empty()
            && content::remote_matches(&input.svc.http_client, url, &TRADEMARK_DISCLAIMER).await?
        {
            return Ok(CheckOutput::passed());
        }
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trademark_disclaimer_match() {
        assert!(TRADEMARK_DISCLAIMER.is_match("https://www.linuxfoundation.org/trademark-usage"));
        assert!(TRADEMARK_DISCLAIMER.is_match("https://linuxfoundation.org/trademark-usage"));
        assert!(TRADEMARK_DISCLAIMER.is_match(
            "The Linux Foundation® (TLF) has registered trademarks and uses trademarks."
        ));
    }
}
