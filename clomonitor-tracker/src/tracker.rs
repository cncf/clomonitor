use anyhow::{format_err, Error};
use clomonitor_core::{
    linter::{lint, Report},
    score::{self, Score},
    Linter,
};
use deadpool_postgres::{Client as DbClient, Transaction};
use std::path::Path;
use std::time::Instant;
use tempdir::TempDir;
use tokio::process::Command;
use tokio_postgres::types::Json;
use tokio_postgres::Error as DbError;
use tracing::{debug, warn};
use uuid::Uuid;

/// Repository information.
#[derive(Debug)]
pub(crate) struct Repository {
    repository_id: Uuid,
    url: String,
    digest: Option<String>,
}

impl Repository {
    pub fn id(&self) -> Uuid {
        self.repository_id
    }
}

/// Get all repositories available in the database.
pub(crate) async fn get_repositories(db: DbClient) -> Result<Vec<Repository>, DbError> {
    debug!("getting repositories");
    let mut repositories: Vec<Repository> = Vec::new();
    let rows = db
        .query("select repository_id, url, digest from repository;", &[])
        .await?;
    for row in rows {
        repositories.push(Repository {
            repository_id: row.get("repository_id"),
            url: row.get("url"),
            digest: row.get("digest"),
        });
    }
    Ok(repositories)
}

/// Process a repository if it has changed since the last time it was processed.
/// This involves cloning the repository, linting it and storing the results.
pub(crate) async fn process_repository(mut db: DbClient, repo: Repository) -> Result<(), Error> {
    let start = Instant::now();

    // Skip if repository hasn't changed since the last time it was processed
    let remote_digest = get_remote_digest(&repo.url).await?;
    if let Some(digest) = repo.digest {
        if remote_digest == digest {
            return Ok(());
        }
    }

    debug!("processing repository [id: {}]", repo.repository_id);

    // Clone repository
    let tmp_dir = TempDir::new("clomonitor")?;
    clone_repository(&repo.url, tmp_dir.path()).await?;

    // Lint repository (only using core linter at the moment)
    let mut errors: Option<String> = None;
    let report = match lint(tmp_dir.path()) {
        Ok(report) => Some(report),
        Err(err) => {
            warn!(
                "error linting repository [id: {}]: {}",
                repo.repository_id,
                err.to_string()
            );
            errors = Some(err.to_string());
            None
        }
    };

    // Store processing results in database
    let tx = db.transaction().await?;
    store_linter_report(&tx, &repo.repository_id, Linter::Core, &report, errors).await?;
    update_repository_score(&tx, &repo.repository_id).await?;
    update_project_score(&tx, &repo.repository_id).await?;
    update_repository_digest(&tx, &repo.repository_id, &remote_digest).await?;
    tx.commit().await?;

    debug!(
        "repository processed in {}s [id: {}]",
        start.elapsed().as_secs(),
        repo.repository_id
    );
    Ok(())
}

/// Get the remote digest of a repository.
async fn get_remote_digest(url: &str) -> Result<String, Error> {
    let output = Command::new("git")
        .arg("ls-remote")
        .arg(url)
        .arg("HEAD")
        .output()
        .await?;
    if !output.status.success() {
        return Err(format_err!("{}", String::from_utf8_lossy(&output.stderr)));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.split_whitespace().next().unwrap().to_string())
}

/// Clone (shallow) a repository in the destination path provided.
async fn clone_repository(url: &str, dst: &Path) -> Result<(), Error> {
    let output = Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg(url)
        .arg(dst)
        .output()
        .await?;
    if !output.status.success() {
        return Err(format_err!("{}", String::from_utf8_lossy(&output.stderr)));
    }
    Ok(())
}

/// Store the provided linter report.
async fn store_linter_report(
    tx: &Transaction<'_>,
    repository_id: &Uuid,
    linter: Linter,
    report: &Option<Report>,
    errors: Option<String>,
) -> Result<(), Error> {
    tx.execute(
        "
        insert into report
            (data, errors, repository_id, linter_id)
        values
            ($1::jsonb, $2::text, $3::uuid, $4::integer)
        on conflict (repository_id, linter_id) do update
        set
            data = excluded.data,
            errors = excluded.errors,
            updated_at = current_timestamp;
        ",
        &[&Json(report), &errors, &repository_id, &(linter as i32)],
    )
    .await?;
    Ok(())
}

/// Update repository's score based on the repository's linters reports.
async fn update_repository_score(tx: &Transaction<'_>, repository_id: &Uuid) -> Result<(), Error> {
    // Lock repository's row
    tx.query_one(
        "select 1 from repository where repository_id = $1::uuid for update;",
        &[&repository_id],
    )
    .await?;

    // Calculate repository's score from the linters reports available
    let mut scores = Vec::<Score>::new();
    let rows = tx
        .query(
            "select linter_id, data from report where repository_id = $1::uuid;",
            &[&repository_id],
        )
        .await?;
    for row in rows {
        let linter_id: i32 = row.get("linter_id");
        let linter = Linter::try_from(linter_id)?;
        let report: Json<Report> = row.get("data");
        scores.push(score::calculate(linter, &report.0));
    }
    let repository_score = score::merge(scores);

    // Update repository's score
    tx.execute(
        "update repository set score = $1::jsonb where repository_id = $2::uuid;",
        &[&Json(&repository_score), &repository_id],
    )
    .await?;

    Ok(())
}

/// Update project's score based on the project's repositories scores.
async fn update_project_score(tx: &Transaction<'_>, repository_id: &Uuid) -> Result<(), Error> {
    // Get project's id and lock project's row
    let row = tx
        .query_one(
            "
            select project_id from project
            where project_id in (
                select project_id from repository where repository_id = $1::uuid
            ) for update;
            ",
            &[&repository_id],
        )
        .await?;
    let project_id: Uuid = row.get("project_id");

    // Calculate project's score from the repositories' scores
    let mut repositories_scores = Vec::<Score>::new();
    let rows = tx
        .query(
            "
            select score from repository
            where repository_id in (
                select repository_id from repository where project_id = $1::uuid
            );
            ",
            &[&project_id],
        )
        .await?;
    for row in rows {
        let score: Option<Json<Score>> = row.get("score");
        if let Some(Json(score)) = score {
            repositories_scores.push(score);
        }
    }
    let project_score = score::merge(repositories_scores);

    // Update project's score and rating
    tx.execute(
        "
        update project set
            score = $1::jsonb,
            rating = $2::text,
            updated_at = current_timestamp
        where project_id = $3::uuid;
        ",
        &[
            &Json(&project_score),
            &score::rating(&project_score),
            &project_id,
        ],
    )
    .await?;

    Ok(())
}

/// Update repository's digest.
async fn update_repository_digest(
    tx: &Transaction<'_>,
    repository_id: &Uuid,
    digest: &str,
) -> Result<(), Error> {
    tx.execute(
        "update repository set digest = $1::text where repository_id = $2::uuid;",
        &[&digest, &repository_id],
    )
    .await?;
    Ok(())
}
