use std::sync::LazyLock;

use anyhow::Result;
use regex::RegexSet;

use crate::linter::{
    CheckSet,
    check::{CheckId, CheckInput, CheckOutput},
};

/// Check identifier.
pub(crate) const ID: CheckId = "analytics";

/// Check score weight.
pub(crate) const WEIGHT: usize = 0;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Community];

/// Google Analytics 4 regular expressions.
static GA4: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new([r"\bG-[A-Z0-9]{10}\b"]).expect("exprs in GA4 to be valid"));

/// Google Tag Manager regular expressions.
static GTM: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new([r"\bGTM-[A-Z0-9]{4,8}\b"]).expect("exprs in GTM to be valid"));

/// HubSpot Analytics regular expressions.
static HUBSPOT: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([r"(?:js\.hs-scripts\.com|js-[a-z0-9]+\.hs-scripts\.com)/\d{6,10}\.js"])
        .expect("exprs in HUBSPOT to be valid")
});

/// Plausible Analytics regular expressions.
static PLAUSIBLE: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([r"plausible\.io/js/script(?:\.[a-z-]+)*\.js"])
        .expect("exprs in PLAUSIBLE to be valid")
});

/// Scarf Analytics regular expressions.
static SCARF: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([r"static\.scarf\.sh/a\.png\?x-pxid=[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}"])
        .expect("exprs in SCARF to be valid")
});

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

    // Check Google Analytics 4
    if GA4.is_match(&content) {
        analytics_detected.push("GA4".to_string());
        details.push_str("· Google Analytics 4\n");
    }

    // Check Google Tag Manager
    if GTM.is_match(&content) {
        analytics_detected.push("GTM".to_string());
        details.push_str("· Google Tag Manager\n");
    }

    // Check HubSpot
    if HUBSPOT.is_match(&content) {
        analytics_detected.push("HubSpot".to_string());
        details.push_str("· HubSpot\n");
    }

    // Check Plausible Analytics
    if PLAUSIBLE.is_match(&content) {
        analytics_detected.push("Plausible".to_string());
        details.push_str("· Plausible Analytics\n");
    }

    // Check Scarf Analytics
    if SCARF.is_match(&content) {
        analytics_detected.push("Scarf".to_string());
        details.push_str("· Scarf Analytics\n");
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
    fn ga4_match() {
        assert!(GA4.is_match("G-ABCDEFGHIJ"));
        assert!(GA4.is_match("G-1234567890"));
        assert!(GA4.is_match("G-NVMH1T3GEK"));
        assert!(GA4.is_match("https://www.googletagmanager.com/gtag/js?id=G-ABCDEFGHIJ"));
        assert!(GA4.is_match("http://googletagmanager.com/gtag/js?id=G-1234567890"));
        assert!(GA4.is_match("gtag('config', 'G-ABCDEFGHIJ')"));
        assert!(GA4.is_match("gtag ( \"config\" , \"G-1234567890\" )"));

        assert!(!GA4.is_match("G-ABC")); // Too short
        assert!(!GA4.is_match("G-ABCDEFGHIJK")); // Too long
        assert!(!GA4.is_match("UA-12345678-1")); // Universal Analytics format
    }

    #[test]
    fn gtm_match() {
        assert!(GTM.is_match("GTM-ABCD"));
        assert!(GTM.is_match("GTM-ABC123"));
        assert!(GTM.is_match("GTM-12345678"));
        assert!(GTM.is_match("https://www.googletagmanager.com/gtm.js?id=GTM-ABC123"));
        assert!(GTM.is_match("http://googletagmanager.com/gtm.js?id=GTM-WXYZ"));
        assert!(GTM.is_match("https://www.googletagmanager.com/ns.html?id=GTM-ABC123"));
        assert!(GTM.is_match("http://googletagmanager.com/ns.html?id=GTM-12345"));

        assert!(!GTM.is_match("GTM-ABC")); // Too short
        assert!(!GTM.is_match("GTM-123456789")); // Too long
        assert!(!GTM.is_match("G-ABCDEFGHIJ")); // GA4 format
    }

    #[test]
    fn hubspot_match() {
        assert!(HUBSPOT.is_match("https://js.hs-scripts.com/123456.js"));
        assert!(HUBSPOT.is_match("//js.hs-scripts.com/1234567890.js"));
        assert!(HUBSPOT.is_match("https://js-na1.hs-scripts.com/987654.js"));
        assert!(HUBSPOT.is_match("https://js-eu1.hs-scripts.com/123456789.js"));

        assert!(!HUBSPOT.is_match("https://js.hs-scripts.com/12345.js")); // Too short Hub ID
        assert!(!HUBSPOT.is_match("https://js.hs-scripts.com/12345678901.js")); // Too long Hub ID
    }

    #[test]
    fn plausible_match() {
        assert!(PLAUSIBLE.is_match("https://plausible.io/js/script.js"));
        assert!(PLAUSIBLE.is_match("https://plausible.io/js/script.outbound-links.js"));
        assert!(
            PLAUSIBLE
                .is_match("data-domain=\"example.com\" src=\"https://plausible.io/js/script.js\"")
        );
    }

    #[test]
    fn scarf_match() {
        assert!(
            SCARF.is_match(
                "https://static.scarf.sh/a.png?x-pxid=a1b2c3d4-e5f6-7890-abcd-ef1234567890"
            )
        );
        assert!(
            SCARF.is_match(
                "http://static.scarf.sh/a.png?x-pxid=12345678-1234-1234-1234-123456789012"
            )
        );

        assert!(!SCARF.is_match("http://static.scarf.sh/a.png?x-pxid=1234")); // Too short pxid
        assert!(!SCARF.is_match("http://static.scarf.sh/a.png?x-pxid=not-a-valid-uuid")); // Invalid UUID format
        assert!(
            !SCARF.is_match(
                "http://static.scarf.sh/a.png?x-pxid=12345678-1234-1234-1234-12345678901"
            )
        );
        // Wrong segment lengths
    }
}
