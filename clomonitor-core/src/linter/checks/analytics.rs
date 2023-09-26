use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

/// Check identifier.
pub(crate) const ID: CheckId = "analytics";

/// Check score weight.
pub(crate) const WEIGHT: usize = 1;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Community];

lazy_static! {
    #[rustfmt::skip]
    static ref GA3: Regex = Regex::new(
        "UA-[0-9]+-[0-9]+",
    ).expect("exprs in GA3 to be valid");

    #[rustfmt::skip]
    static ref GA4: Regex = Regex::new(
        "G-[A-Z0-9]+",
    ).expect("exprs in GA4 to be valid");

    #[rustfmt::skip]
    static ref HUBSPOT: Regex = Regex::new(
        r"//js.hs-scripts.com/.+\.js",
    ).expect("exprs in HUBSPOT to be valid");
}

/// Check main function.
pub(crate) async fn check(input: &CheckInput<'_>) -> Result<CheckOutput<Vec<String>>> {
    // Get website content
    let content = match &input.gh_md.homepage_url {
        Some(url) if !url.is_empty() => reqwest::get(url).await?.text().await?,
        _ => return Ok(CheckOutput::not_passed()),
    };

    let mut analytics_detected: Vec<String> = Vec::new();
    let mut details: String =
        "# Analytics providers detected in project's website \n\n".to_string();

    // Check Google Analytics 3 (Universal Analytics) tracking ID
    if GA3.is_match(&content) {
        analytics_detected.push("GA3".to_string());
        details.push_str("· Google Analytics 3 (Universal Analytics)\n");
    }

    // Check Google Analytics 4 measurement ID
    if GA4.is_match(&content) {
        analytics_detected.push("GA4".to_string());
        details.push_str("· Google Analytics 4\n");
    }

    // Check HubSpot tracking code
    if HUBSPOT.is_match(&content) {
        analytics_detected.push("HubSpot".to_string());
        details.push_str("· HubSpot\n");
    }

    // Return check output
    if !analytics_detected.is_empty() {
        return Ok(CheckOutput::passed()
            .value(Some(analytics_detected))
            .details(Some(details)));
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ga3_match() {
        assert!(GA3.is_match("UA-123456-0"));
        assert!(GA3.is_match("UA-12345678-90"));
    }

    #[test]
    fn ga4_match() {
        assert!(GA4.is_match("G-XXXXXXXXXX"));
        assert!(GA4.is_match("G-NVMH1T3GEK"));
        assert!(GA4.is_match("G-12345678"));
    }

    #[test]
    fn hubspot_match() {
        assert!(HUBSPOT.is_match("https://js.hs-scripts.com/123.js"));
    }
}
