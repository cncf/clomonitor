use anyhow::Result;

use crate::linter::{
    CheckSet,
    check::{CheckId, CheckInput, CheckOutput},
};

use super::datasource::landscape;

/// Check identifier.
pub(crate) const ID: CheckId = "summary_table";

/// Check score weight.
pub(crate) const WEIGHT: usize = 1;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Community];

/// Check main function.
pub(crate) async fn check(input: &CheckInput<'_>) -> Result<CheckOutput> {
    // Get landscape (if necessary info is available)
    let mut landscape = None;
    if let Some(project) = &input.li.project.as_ref()
        && let Some(url) = project.foundation.landscape_url.as_ref()
    {
        landscape = Some(landscape::new(url.clone()).await?);
    }

    // Check project's summary table info in landscape
    if let Some(landscape) = landscape {
        let project_name = &input.li.project.as_ref().unwrap().name;
        if let Some(summary_table) = landscape.get_summary_table_info(project_name) {
            Ok(CheckOutput::passed().details(Some(format!("{summary_table}"))))
        } else {
            Ok(CheckOutput::not_passed())
        }
    } else {
        Ok(CheckOutput::exempt()
            .exemption_reason(Some("Landscape information not available".to_string())))
    }
}
