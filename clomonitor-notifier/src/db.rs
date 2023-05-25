use crate::notifier::AnnualReviewNotification;
use anyhow::Result;
use async_trait::async_trait;
use deadpool_postgres::Pool;
#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

/// Trait that defines some operations a DB implementation must support.
#[async_trait]
#[cfg_attr(test, automock)]
pub(crate) trait DB {
    /// Returns the pending annual review notifications.
    async fn get_pending_annual_review_notifications(
        &self,
    ) -> Result<Vec<AnnualReviewNotification>>;

    /// Pre-register annual review notification.
    async fn pre_register_annual_review_notification(
        &self,
        project_id: &Uuid,
        repository_url: &str,
    ) -> Result<Uuid>;

    /// Update annual review notification details.
    async fn update_annual_review_notification(
        &self,
        notification_id: &Uuid,
        issue_number: Option<i64>,
        comment_id: Option<i64>,
    ) -> Result<()>;
}

/// Type alias to represent a DB trait object.
pub(crate) type DynDB = Box<dyn DB + Send + Sync>;

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
    /// [DB::get_pending_annual_review_notifications]
    async fn get_pending_annual_review_notifications(
        &self,
    ) -> Result<Vec<AnnualReviewNotification>> {
        let db = self.pool.get().await?;
        let pending_notifications = db
            .query(
                "select * from get_pending_annual_review_notifications()",
                &[],
            )
            .await?
            .iter()
            .map(|row| AnnualReviewNotification {
                project_id: row.get("project_id"),
                community_repo_url: row.get("community_repo_url"),
                issue_number: row.get("issue_number"),
            })
            .collect();
        Ok(pending_notifications)
    }

    /// [DB::pre_register_annual_review_notification]
    async fn pre_register_annual_review_notification(
        &self,
        project_id: &Uuid,
        repository_url: &str,
    ) -> Result<Uuid> {
        let db = self.pool.get().await?;
        let notification_id = db
            .query_one(
                "
                insert into annual_review_notification (
                    project_id,
                    repository_url
                ) values (
                    $1::uuid,
                    $2::text
                ) returning annual_review_notification_id;
                ",
                &[&project_id, &repository_url],
            )
            .await?
            .get("annual_review_notification_id");
        Ok(notification_id)
    }

    /// [DB::update_annual_review_notification]
    async fn update_annual_review_notification(
        &self,
        notification_id: &Uuid,
        issue_number: Option<i64>,
        comment_id: Option<i64>,
    ) -> Result<()> {
        let db = self.pool.get().await?;
        db.execute(
            "
            update annual_review_notification set
                issue_number = $1::bigint,
                comment_id = $2::bigint
            where annual_review_notification_id = $3::uuid;
            ",
            &[&issue_number, &comment_id, &notification_id],
        )
        .await?;
        Ok(())
    }
}
