use crate::registrar::{Foundation, Project};
use anyhow::Result;
use async_trait::async_trait;
use deadpool_postgres::Pool;
#[cfg(test)]
use mockall::automock;
use std::{collections::HashMap, sync::Arc};
use tokio_postgres::types::Json;

/// Type alias to represent a DB trait object.
pub(crate) type DynDB = Arc<dyn DB + Send + Sync>;

/// Trait that defines some operations a DB implementation must support.
#[async_trait]
#[cfg_attr(test, automock)]
pub(crate) trait DB {
    /// Get foundations registered in the database.
    async fn foundations(&self) -> Result<Vec<Foundation>>;

    /// Get projects for the foundation provided.
    async fn foundation_projects(
        &self,
        foundation_id: &str,
    ) -> Result<HashMap<String, Option<String>>>;

    /// Register project provided in the database.
    async fn register_project(&self, foundation_id: &str, project: &Project) -> Result<()>;

    /// Unregister project provided from the database.
    async fn unregister_project(&self, foundation_id: &str, project_name: &str) -> Result<()>;
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
    async fn foundations(&self) -> Result<Vec<Foundation>> {
        let db = self.pool.get().await?;
        let foundations = db
            .query("select foundation_id, data_url from foundation", &[])
            .await?
            .iter()
            .map(|row| Foundation {
                foundation_id: row.get("foundation_id"),
                data_url: row.get("data_url"),
            })
            .collect();
        Ok(foundations)
    }

    async fn foundation_projects(
        &self,
        foundation_id: &str,
    ) -> Result<HashMap<String, Option<String>>> {
        let db = self.pool.get().await?;
        let projects = db
            .query(
                "select name, digest from project where foundation_id = $1::text",
                &[&foundation_id],
            )
            .await?
            .iter()
            .map(|row| (row.get("name"), row.get("digest")))
            .collect();
        Ok(projects)
    }

    async fn register_project(&self, foundation_id: &str, project: &Project) -> Result<()> {
        let db = self.pool.get().await?;
        db.execute(
            "select register_project($1::text, $2::jsonb)",
            &[&foundation_id, &Json(project)],
        )
        .await?;
        Ok(())
    }

    async fn unregister_project(&self, foundation_id: &str, project_name: &str) -> Result<()> {
        let db = self.pool.get().await?;
        db.execute(
            "select unregister_project($1::text, $2::text)",
            &[&foundation_id, &project_name],
        )
        .await?;
        Ok(())
    }
}
