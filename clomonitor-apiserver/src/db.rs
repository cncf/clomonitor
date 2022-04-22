use anyhow::Result;
use async_trait::async_trait;
use clomonitor_core::score::Score;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_postgres::types::Json;

/// Type alias to represent a DB trait object.
pub type DynDB = Arc<dyn DB + Send + Sync>;

/// Type alias to represent a json string.
type JsonString = String;

/// Type alias to represent a counter value.
type Count = i64;

/// Trait that defines some operations a DB implementation must support.
#[async_trait]
pub trait DB {
    /// Get project's details in json format.
    async fn project(
        &self,
        foundation: &str,
        org: &str,
        project: &str,
    ) -> Result<Option<JsonString>>;

    /// Get project's rating.
    async fn project_rating(
        &self,
        foundation: &str,
        org: &str,
        project: &str,
    ) -> Result<Option<String>>;

    /// Get project's score.
    async fn project_score(
        &self,
        foundation: &str,
        org: &str,
        project: &str,
    ) -> Result<Option<Score>>;

    /// Search projects that match the criteria provided.
    async fn search_projects(&self, input: &SearchProjectsInput) -> Result<(Count, JsonString)>;

    /// Get some general stats.
    async fn stats(&self, foundation: Option<&String>) -> Result<JsonString>;
}

/// DB implementation backed by PostgreSQL.
pub struct PgDB {
    pool: Pool,
}

impl PgDB {
    /// Create a new PgDB instance.
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DB for PgDB {
    async fn project(
        &self,
        foundation: &str,
        org: &str,
        project: &str,
    ) -> Result<Option<JsonString>> {
        let row = self
            .pool
            .get()
            .await?
            .query_one(
                "select get_project($1::text, $2::text, $3::text)::text",
                &[&foundation, &org, &project],
            )
            .await?;
        let project: Option<String> = row.get(0);
        Ok(project)
    }

    async fn project_rating(
        &self,
        foundation: &str,
        org: &str,
        project: &str,
    ) -> Result<Option<String>> {
        let rows = self
            .pool
            .get()
            .await?
            .query(
                "
                select rating
                from project p
                join organization o using (organization_id)
                where o.foundation::text = $1::text
                and o.name = $2::text
                and p.name = $3::text
                ",
                &[&foundation, &org, &project],
            )
            .await?;
        if rows.len() != 1 {
            return Ok(None);
        }
        let rating: Option<String> = rows.first().unwrap().get("rating");
        Ok(rating)
    }

    async fn project_score(
        &self,
        foundation: &str,
        org: &str,
        project: &str,
    ) -> Result<Option<Score>> {
        let rows = self
            .pool
            .get()
            .await?
            .query(
                "
                select score
                from project p
                join organization o using (organization_id)
                where o.foundation::text = $1::text
                and o.name = $2::text
                and p.name = $3::text
                ",
                &[&foundation, &org, &project],
            )
            .await?;
        if rows.len() != 1 {
            return Ok(None);
        }
        let score: Option<Json<Score>> = rows.first().unwrap().get("score");
        match score {
            Some(Json(score)) => Ok(Some(score)),
            None => Ok(None),
        }
    }

    async fn search_projects(&self, input: &SearchProjectsInput) -> Result<(Count, JsonString)> {
        let row = self
            .pool
            .get()
            .await?
            .query_one(
                "select total_count, projects::text from search_projects($1::jsonb)",
                &[&Json(input)],
            )
            .await?;
        let count: i64 = row.get("total_count");
        let projects: String = row.get("projects");
        Ok((count, projects))
    }

    async fn stats(&self, foundation: Option<&String>) -> Result<JsonString> {
        let row = self
            .pool
            .get()
            .await?
            .query_one("select get_stats($1::text)::text", &[&foundation])
            .await?;
        let stats: String = row.get(0);
        Ok(stats)
    }
}

/// Query input used when searching for projects.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchProjectsInput {
    limit: Option<usize>,
    offset: Option<usize>,
    sort_by: Option<String>,
    sort_direction: Option<String>,
    text: Option<String>,
    foundation: Option<Vec<String>>,
    maturity: Option<Vec<String>>,
    rating: Option<Vec<char>>,
    accepted_from: Option<String>,
    accepted_to: Option<String>,
}
