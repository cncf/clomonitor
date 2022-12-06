use anyhow::Result;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use serde_json::Value;
use std::sync::Arc;
use time::Date;
use uuid::Uuid;

/// Type alias to represent a DB trait object.
pub(crate) type DynDB = Arc<dyn DB + Send + Sync>;

/// Trait that defines some operations a DB implementation must support.
#[async_trait]
pub(crate) trait DB {
    /// Delete the provided project's snapshot.
    async fn delete_project_snapshot(&self, project_id: &Uuid, date: &Date) -> Result<()>;

    /// Get project's data.
    async fn project_data(&self, project_id: &Uuid) -> Result<Option<Value>>;

    /// Get the dates of all the project's snapshots.
    async fn project_snapshots(&self, project_id: &Uuid) -> Result<Vec<Date>>;

    /// Get the ids of all projects registered in the database.
    async fn projects_ids(&self) -> Result<Vec<Uuid>>;

    /// Store the provided project's snapshot.
    async fn store_project_snapshot(&self, project_id: &Uuid, data: Value) -> Result<()>;
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
    async fn delete_project_snapshot(&self, project_id: &Uuid, date: &Date) -> Result<()> {
        let db = self.pool.get().await?;
        match db
            .execute(
                "delete from project_snapshot where project_id = $1 and date = $2",
                &[&project_id, &date],
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    async fn project_data(&self, project_id: &Uuid) -> Result<Option<Value>> {
        let db = self.pool.get().await?;
        let data: Option<Value> = db
            .query_one("select get_project_by_id($1::uuid)", &[&project_id])
            .await?
            .get(0);
        Ok(data)
    }

    async fn project_snapshots(&self, project_id: &Uuid) -> Result<Vec<Date>> {
        let db = self.pool.get().await?;
        let snapshots = db
            .query(
                "select date from project_snapshot where project_id = $1 order by date desc",
                &[&project_id],
            )
            .await?
            .iter()
            .map(|row| row.get("date"))
            .collect();
        Ok(snapshots)
    }

    async fn projects_ids(&self) -> Result<Vec<Uuid>> {
        let db = self.pool.get().await?;
        let projects = db
            .query("select project_id from project", &[])
            .await?
            .iter()
            .map(|row| row.get("project_id"))
            .collect();
        Ok(projects)
    }

    async fn store_project_snapshot(&self, project_id: &Uuid, data: Value) -> Result<()> {
        let db = self.pool.get().await?;
        match db
            .execute(
                "insert into project_snapshot (project_id, data) values ($1::uuid, $2::jsonb)",
                &[&project_id, &data],
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}
