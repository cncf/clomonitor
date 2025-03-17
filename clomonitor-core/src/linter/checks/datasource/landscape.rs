use anyhow::Result;
use cached::proc_macro::cached;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use time::Date;

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
    extra: Option<ItemExtra>,
}

/// Extra information for a landscape item.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(super) struct ItemExtra {
    pub clomonitor_name: Option<String>,
    pub summary_business_use_case: Option<String>,
    pub summary_integration: Option<String>,
    pub summary_integrations: Option<String>,
    pub summary_intro_url: Option<String>,
    pub summary_use_case: Option<String>,
    pub summary_personas: Option<String>,
    pub summary_release_rate: Option<String>,
    pub summary_tags: Option<String>,
}

/// Create a new Landscape instance from the corresponding foundation
/// landscape.yml file.
#[cached(time = 1800, sync_writes = "default", result = true)]
pub(crate) async fn new(url: String) -> Result<Landscape> {
    let content = reqwest::get(url).await?.text().await?;
    Ok(serde_yaml::from_str(&content)?)
}

impl Landscape {
    /// Return the project's summary table information if available.
    pub(crate) fn get_summary_table_info(&self, project_name: &str) -> Option<SummaryTable> {
        // Prepare project's summary table from available info (if any)
        if let Some(project) = self.get_project(project_name) {
            if let Some(extra) = &project.extra {
                let summary_table = SummaryTable::from(extra);
                if summary_table != SummaryTable::default() {
                    return Some(summary_table);
                }
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
                        if let Some(clomonitor_name) = &extra.clomonitor_name {
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

impl From<&ItemExtra> for SummaryTable {
    fn from(extra: &ItemExtra) -> Self {
        SummaryTable {
            personas: extra.summary_personas.clone(),
            tags: extra.summary_tags.clone(),
            use_case: extra.summary_use_case.clone(),
            business_use_case: extra.summary_business_use_case.clone(),
            release_date: extra.summary_release_rate.clone(),
            integrations: extra.summary_integrations.clone(),
            intro_url: extra.summary_intro_url.clone(),
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
