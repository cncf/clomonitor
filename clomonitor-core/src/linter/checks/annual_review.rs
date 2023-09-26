use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    landscape, CheckSet,
};
use anyhow::Result;
use time::{Duration, OffsetDateTime};

/// Check identifier.
pub(crate) const ID: CheckId = "annual_review";

/// Check score weight.
pub(crate) const WEIGHT: usize = 1;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Community];

/// One year duration.
const ONE_YEAR: Duration = Duration::new(60 * 60 * 24 * 365, 0);

/// Grace period duration (60 days).
const GRACE_PERIOD: Duration = Duration::new(60 * 60 * 24 * 60, 0);

/// Check main function.
pub(crate) async fn check(input: &CheckInput<'_>) -> Result<CheckOutput> {
    // Check if project information is available
    let Some(project) = &input.li.project.as_ref() else {
        return Ok(CheckOutput::exempt()
            .exemption_reason(Some("Project information not available".to_string())));
    };
    let Some(project_accepted_date) = project.accepted_at.as_ref() else {
        return Ok(CheckOutput::exempt()
            .exemption_reason(Some("Project accepted date not available".to_string())));
    };

    // This check only applies to CNCF Sandbox projects
    if project.foundation.foundation_id != "cncf"
        || project.maturity.as_ref().unwrap_or(&String::new()) != "sandbox"
    {
        return Ok(CheckOutput::exempt().exemption_reason(Some(
            "This check only applies to CNCF Sandbox projects".to_string(),
        )));
    }

    // Check if landscape information is available
    let Some(landscape_url) = project.foundation.landscape_url.as_ref() else {
        return Ok(CheckOutput::exempt()
            .exemption_reason(Some("Landscape information not available".to_string())));
    };
    let landscape = landscape::new(landscape_url.clone()).await?;

    // Check if the project has been required to present the annual review yet
    let current_date = OffsetDateTime::now_utc().date();
    if current_date - *project_accepted_date < ONE_YEAR + GRACE_PERIOD {
        return Ok(CheckOutput::exempt().exemption_reason(Some(
            "The project has not been required to present the annual review yet".to_string(),
        )));
    }

    // Check annual review info in landscape
    if let Some(last_annual_review) = landscape.get_annual_review_info(&project.name)? {
        let due = last_annual_review.date + ONE_YEAR + GRACE_PERIOD;
        if current_date < due {
            Ok(CheckOutput::passed().url(Some(last_annual_review.url)))
        } else {
            Ok(CheckOutput::not_passed()
                .details(Some(
                    "Annual review information in landscape is outdated".to_string(),
                ))
                .url(Some(last_annual_review.url)))
        }
    } else {
        Ok(CheckOutput::not_passed().details(Some(
            "Annual review information not found in landscape".to_string(),
        )))
    }
}
