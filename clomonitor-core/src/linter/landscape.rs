use anyhow::{Context, Result};
use cached::proc_macro::cached;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};
use time::{macros::format_description, Date};

/// Key used in the extra section of the landscape yaml file for the project
/// name in CLOMonitor.
const CLOMONITOR_NAME_KEY: &str = "clomonitor_name";

/// Foundation Landscape information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Landscape {
    landscape: Vec<Category>,
}

/// Landscape category.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Category {
    subcategories: Vec<SubCategory>,
}

/// Landscape subcategory.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct SubCategory {
    items: Vec<Item>,
}

/// Landscape item.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Item {
    extra: Option<HashMap<String, String>>,
}

/// Create a new Landscape instance from the corresponding foundation
/// landscape.yml file.
#[cached(time = 1800, sync_writes = true, result = true)]
pub(crate) async fn new(url: String) -> Result<Landscape> {
    let content = reqwest::get(url).await?.text().await?;
    Ok(serde_yaml::from_str(&content)?)
}

impl Landscape {
    /// Return the project's annual review information if available.
    pub(crate) fn get_annual_review_info(
        &self,
        project_name: &str,
    ) -> Result<Option<AnnualReview>> {
        // Prepare project's annual review from available info (if any)
        if let Some(project) = self.get_project(project_name) {
            let annual_review = AnnualReview::from(project.extra.as_ref().unwrap())?;
            return Ok(annual_review);
        }

        Ok(None)
    }

    /// Return the project's summary table information if available.
    pub(crate) fn get_summary_table_info(&self, project_name: &str) -> Option<SummaryTable> {
        // Prepare project's summary table from available info (if any)
        if let Some(project) = self.get_project(project_name) {
            let summary_table = SummaryTable::from(project.extra.as_ref().unwrap());
            if summary_table != SummaryTable::default() {
                return Some(summary_table);
            }
        }

        None
    }

    /// Return the requested project if available.
    fn get_project(&self, project_name: &str) -> Option<&Item> {
        for category in &self.landscape {
            for subcategory in &category.subcategories {
                for item in &subcategory.items {
                    if let Some(extra) = &item.extra {
                        if let Some(clomonitor_name) = extra.get(CLOMONITOR_NAME_KEY) {
                            if clomonitor_name == project_name {
                                return Some(item);
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

/// Project's annual review information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct AnnualReview {
    pub date: Date,
    pub url: String,
}

impl AnnualReview {
    fn from(extra: &HashMap<String, String>) -> Result<Option<Self>> {
        let Some(date) = extra.get("annual_review_date") else {
            return Ok(None);
        };
        let Some(url) = extra.get("annual_review_url") else {
            return Ok(None);
        };

        let format = format_description!("[year]-[month]-[day]");
        let date = Date::parse(date, &format).context("invalid annual review date in landscape")?;

        Ok(Some(AnnualReview {
            date,
            url: url.clone(),
        }))
    }
}

/// Project's summary table information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct SummaryTable {
    personas: Option<String>,
    tags: Option<String>,
    use_case: Option<String>,
    business_use_case: Option<String>,
    release_date: Option<String>,
    integrations: Option<String>,
    intro_url: Option<String>,
}

impl From<&HashMap<String, String>> for SummaryTable {
    fn from(extra: &HashMap<String, String>) -> Self {
        SummaryTable {
            personas: extra.get("summary_personas").cloned(),
            tags: extra.get("summary_tags").cloned(),
            use_case: extra.get("summary_use_case").cloned(),
            business_use_case: extra.get("summary_business_use_case").cloned(),
            release_date: extra.get("summary_release_rate").cloned(),
            integrations: extra.get("summary_integrations").cloned(),
            intro_url: extra.get("summary_intro_url").cloned(),
        }
    }
}

impl Display for SummaryTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let empty = "-".to_string();
        let output = format!(
            r"# Summary table information

**Personas**: {}

**Tags**: {}

**Use case**: {}

**Business use case**: {}

**Release date**: {}

**Integrations**: {}

**Intro URL**: {}
",
            self.personas.as_ref().unwrap_or(&empty),
            self.tags.as_ref().unwrap_or(&empty),
            self.use_case.as_ref().unwrap_or(&empty),
            self.business_use_case.as_ref().unwrap_or(&empty),
            self.release_date.as_ref().unwrap_or(&empty),
            self.integrations.as_ref().unwrap_or(&empty),
            self.intro_url.as_ref().unwrap_or(&empty),
        );
        write!(f, "{output}")
    }
}
