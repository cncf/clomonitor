use crate::{
    handlers::RepositoryReportMDTemplate,
    views::{Day, ProjectId, Total},
};
use anyhow::Result;
use async_trait::async_trait;
use clomonitor_core::score::Score;
use deadpool_postgres::Pool;
#[cfg(test)]
use mockall::automock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::Date;
use tokio_postgres::types::Json;

// Lock key used when updating the projects views in the database.
const LOCK_KEY_UPDATE_PROJECTS_VIEWS: i64 = 1;

/// Type alias to represent a DB trait object.
pub(crate) type DynDB = Arc<dyn DB + Send + Sync>;

/// Type alias to represent a json string.
type JsonString = String;

/// Type alias to represent a counter value.
type Count = i64;

/// Trait that defines some operations a DB implementation must support.
#[async_trait]
#[cfg_attr(test, automock)]
pub(crate) trait DB {
    /// Get project's data in json format.
    async fn project_data(
        &self,
        foundation: &str,
        project_name: &str,
    ) -> Result<Option<JsonString>>;

    /// Get project's rating.
    async fn project_rating(&self, foundation: &str, project_name: &str) -> Result<Option<String>>;

    /// Get project's score.
    async fn project_score(&self, foundation: &str, project_name: &str) -> Result<Option<Score>>;

    /// Get project's snapshot data.
    async fn project_snapshot(
        &self,
        foundation: &str,
        project_name: &str,
        date: &Date,
    ) -> Result<Option<JsonString>>;

    /// Get all repositories including checks details.
    async fn repositories_with_checks(&self) -> Result<String>;

    /// Get some repository info to prepare report in markdown format.
    async fn repository_report_md(
        &self,
        foundation: &str,
        project_name: &str,
        repository_name: &str,
    ) -> Result<Option<RepositoryReportMDTemplate>>;

    /// Search projects that match the criteria provided.
    async fn search_projects(&self, input: &SearchProjectsInput) -> Result<(Count, JsonString)>;

    /// Get some general stats.
    async fn stats(&self, foundation: Option<&str>) -> Result<JsonString>;

    /// Update the number of views of the projects provided.
    async fn update_projects_views(&self, data: Vec<(ProjectId, Day, Total)>) -> Result<()>;
}

/// DB implementation backed by PostgreSQL.
pub(crate) struct PgDB {
    pool: Pool,
}

impl PgDB {
    /// Create a new PgDB instance.
    pub(crate) fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DB for PgDB {
    async fn project_data(
        &self,
        foundation: &str,
        project_name: &str,
    ) -> Result<Option<JsonString>> {
        let db = self.pool.get().await?;
        let project: Option<JsonString> = db
            .query_one(
                "select get_project_by_name($1::text, $2::text)::text",
                &[&foundation, &project_name],
            )
            .await?
            .get(0);
        Ok(project)
    }

    async fn project_rating(&self, foundation: &str, project_name: &str) -> Result<Option<String>> {
        let db = self.pool.get().await?;
        let rating = db
            .query_opt(
                "
                select rating
                from project p
                where p.foundation_id = $1::text
                and p.name = $2::text
                ",
                &[&foundation, &project_name],
            )
            .await?
            .and_then(|row| row.get("rating"));
        Ok(rating)
    }

    async fn project_score(&self, foundation: &str, project_name: &str) -> Result<Option<Score>> {
        let db = self.pool.get().await?;
        let score = db
            .query_opt(
                "
                select score
                from project p
                where p.foundation_id = $1::text
                and p.name = $2::text
                ",
                &[&foundation, &project_name],
            )
            .await?
            .and_then(|row| {
                let score: Option<Json<Score>> = row.get("score");
                score.map(|Json(score)| score)
            });
        Ok(score)
    }

    async fn project_snapshot(
        &self,
        foundation: &str,
        project_name: &str,
        date: &Date,
    ) -> Result<Option<JsonString>> {
        let db = self.pool.get().await?;
        let snapshot = db
            .query_opt(
                "
                select data::text
                from project_snapshot s
                join project p using (project_id)
                where p.foundation_id = $1
                and p.name = $2
                and s.date = $3
                ",
                &[&foundation, &project_name, &date],
            )
            .await?
            .and_then(|row| row.get("data"));
        Ok(snapshot)
    }

    async fn repositories_with_checks(&self) -> Result<String> {
        let db = self.pool.get().await?;
        let repos = db
            .query("select get_repositories_with_checks()", &[])
            .await?
            .iter()
            .map(|row| format!("{}\n", row.get::<_, String>(0)))
            .collect();
        Ok(repos)
    }

    async fn repository_report_md(
        &self,
        foundation: &str,
        project_name: &str,
        repository_name: &str,
    ) -> Result<Option<RepositoryReportMDTemplate>> {
        let db = self.pool.get().await?;
        let report_md = db
            .query_one(
                "select get_repository_report($1::text, $2::text, $3::text)",
                &[&foundation, &project_name, &repository_name],
            )
            .await?
            .get::<_, Option<Json<RepositoryReportMDTemplate>>>(0)
            .map(|Json(report_md)| report_md);
        Ok(report_md)
    }

    async fn search_projects(&self, input: &SearchProjectsInput) -> Result<(Count, JsonString)> {
        let db = self.pool.get().await?;
        let row = db
            .query_one(
                "select total_count, projects::text from search_projects($1::jsonb)",
                &[&Json(input)],
            )
            .await?;
        let count: i64 = row.get("total_count");
        let projects: String = row.get("projects");
        Ok((count, projects))
    }

    async fn stats(&self, foundation: Option<&str>) -> Result<JsonString> {
        let db = self.pool.get().await?;
        let stats = db
            .query_one("select get_stats($1::text)::text", &[&foundation])
            .await?
            .get(0);
        Ok(stats)
    }

    async fn update_projects_views(&self, data: Vec<(ProjectId, Day, Total)>) -> Result<()> {
        let db = self.pool.get().await?;
        db.execute(
            "select update_projects_views($1::bigint, $2::jsonb)",
            &[&LOCK_KEY_UPDATE_PROJECTS_VIEWS, &Json(&data)],
        )
        .await?;
        Ok(())
    }
}

/// Query input used when searching for projects.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct SearchProjectsInput {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub text: Option<String>,
    pub foundation: Option<Vec<String>>,
    pub maturity: Option<Vec<String>>,
    pub rating: Option<Vec<char>>,
    pub accepted_from: Option<String>,
    pub accepted_to: Option<String>,
    pub passing_check: Option<Vec<String>>,
    pub not_passing_check: Option<Vec<String>>,
}
