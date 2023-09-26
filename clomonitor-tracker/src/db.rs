use crate::tracker::Repository;
use anyhow::Result;
use async_trait::async_trait;
use clomonitor_core::{
    linter::{CheckSet, Foundation, Project, Report},
    score::{self, Score},
};
use deadpool_postgres::{Pool, Transaction};
#[cfg(test)]
use mockall::automock;
use std::sync::Arc;
use tokio_postgres::types::Json;
use uuid::Uuid;

/// Type alias to represent a DB trait object.
pub(crate) type DynDB = Arc<dyn DB + Send + Sync>;

/// Trait that defines some operations a DB implementation must support.
#[async_trait]
#[allow(clippy::ref_option_ref)]
#[cfg_attr(test, automock)]
pub(crate) trait DB {
    /// Get all repositories registered in the database.
    async fn repositories(&self) -> Result<Vec<Repository>>;

    /// Store the provided tracking results in the database.
    async fn store_results(
        &self,
        repository_id: &Uuid,
        check_sets: &[CheckSet],
        report: Option<&Report>,
        errors: Option<&String>,
        remote_digest: &str,
    ) -> Result<()>;
}

/// DB implementation backed by PostgreSQL.
pub(crate) struct PgDB {
    pool: Pool,
}

#[async_trait]
impl DB for PgDB {
    async fn repositories(&self) -> Result<Vec<Repository>> {
        let db = self.pool.get().await?;
        let repositories = db
            .query(
                "
                select
                    r.repository_id,
                    r.url,
                    r.digest,
                    to_json(r.check_sets) as check_sets,
                    r.updated_at,
                    p.name as project_name,
                    p.accepted_at as project_accepted_at,
                    p.maturity::text as project_maturity,
                    f.foundation_id,
                    f.landscape_url as foundation_landscape_url
                from repository r
                join project p using (project_id)
                join foundation f using (foundation_id)
                where r.check_sets is not null
                and cardinality(r.check_sets) > 0
                ",
                &[],
            )
            .await?
            .iter()
            .map(|row| {
                let Json(check_sets): Json<Vec<CheckSet>> = row.get("check_sets");
                Repository {
                    repository_id: row.get("repository_id"),
                    url: row.get("url"),
                    check_sets,
                    digest: row.get("digest"),
                    updated_at: row.get("updated_at"),
                    project: Project {
                        name: row.get("project_name"),
                        accepted_at: row.get("project_accepted_at"),
                        maturity: row.get("project_maturity"),
                        foundation: Foundation {
                            foundation_id: row.get("foundation_id"),
                            landscape_url: row.get("foundation_landscape_url"),
                        },
                    },
                }
            })
            .collect();
        Ok(repositories)
    }

    async fn store_results(
        &self,
        repository_id: &Uuid,
        check_sets: &[CheckSet],
        report: Option<&Report>,
        errors: Option<&String>,
        remote_digest: &str,
    ) -> Result<()> {
        let mut db = self.pool.get().await?;
        let tx = db.transaction().await?;
        PgDB::store_report(&tx, repository_id, check_sets, report, errors).await?;
        PgDB::update_repository_score(&tx, repository_id, report).await?;
        PgDB::update_project_score(&tx, repository_id).await?;
        PgDB::update_repository_digest(&tx, repository_id, remote_digest).await?;
        tx.commit().await?;
        Ok(())
    }
}

impl PgDB {
    /// Create a new PgDB instance.
    pub(crate) fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Store the provided repository linter report.
    async fn store_report(
        tx: &Transaction<'_>,
        repository_id: &Uuid,
        check_sets: &[CheckSet],
        report: Option<&Report>,
        errors: Option<&String>,
    ) -> Result<()> {
        match report {
            Some(report) => {
                tx.execute(
                    "
                    insert into report (check_sets, data, errors, repository_id)
                    values ($1::check_set[], $2::jsonb, $3::text, $4::uuid)
                    on conflict (repository_id) do update
                    set
                        check_sets = excluded.check_sets,
                        data = excluded.data,
                        errors = excluded.errors,
                        updated_at = current_timestamp;
                    ",
                    &[&check_sets, &Json(&report), &errors, &repository_id],
                )
                .await?;
            }
            None => {
                tx.execute(
                    "
                    insert into report (errors, repository_id)
                    values ($1::text, $2::uuid)
                    on conflict (repository_id) do update
                    set
                        errors = excluded.errors,
                        updated_at = current_timestamp;
                    ",
                    &[&errors, &repository_id],
                )
                .await?;
            }
        }

        Ok(())
    }

    /// Update the score of the project the repository provided belongs to.
    async fn update_project_score(tx: &Transaction<'_>, repository_id: &Uuid) -> Result<()> {
        // Get project's id and lock project's row
        let project_id: Uuid = tx
            .query_one(
                "
                select project_id from project
                where project_id in (
                    select project_id from repository where repository_id = $1::uuid
                ) for update;
                ",
                &[&repository_id],
            )
            .await?
            .get("project_id");

        // Calculate project's score from the repositories' scores
        let repositories_scores: Vec<Score> = tx
            .query(
                "
                select score from repository
                where repository_id in (
                    select repository_id from repository where project_id = $1::uuid
                );
                ",
                &[&project_id],
            )
            .await?
            .iter()
            .filter_map(|row| {
                let score: Option<Json<Score>> = row.get("score");
                score.map(|Json(score)| score)
            })
            .collect();

        // Update project's score and rating
        if !repositories_scores.is_empty() {
            let project_score = score::merge(&repositories_scores[..]);
            tx.execute(
                "
                update project set
                    score = $1::jsonb,
                    rating = $2::text,
                    passed_checks = (select get_project_passed_checks($3::uuid)),
                    updated_at = current_timestamp
                where project_id = $3::uuid;
                ",
                &[
                    &Json(&project_score),
                    &project_score.rating().to_string(),
                    &project_id,
                ],
            )
            .await?;
        }

        Ok(())
    }

    /// Update the digest of the provided repository.
    async fn update_repository_digest(
        tx: &Transaction<'_>,
        repository_id: &Uuid,
        digest: &str,
    ) -> Result<()> {
        tx.execute(
            "update repository set digest = $1::text where repository_id = $2::uuid;",
            &[&digest, &repository_id],
        )
        .await?;
        Ok(())
    }

    /// Update the score of the provided repository.
    async fn update_repository_score(
        tx: &Transaction<'_>,
        repository_id: &Uuid,
        report: Option<&Report>,
    ) -> Result<()> {
        if let Some(report) = report {
            let score = score::calculate(report);
            tx.execute(
                "
                update repository set
                    score = $1::jsonb,
                    updated_at = current_timestamp
                where repository_id = $2::uuid;
                ",
                &[&Json(&score), &repository_id],
            )
            .await?;
        }

        Ok(())
    }
}
