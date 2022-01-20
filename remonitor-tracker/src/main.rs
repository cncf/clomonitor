use anyhow::{format_err, Error};
use config::{Config, File};
use deadpool_postgres::{Client as DbClient, Config as DbConfig, Runtime, Transaction};
use futures::future;
use futures::stream::{FuturesUnordered, StreamExt};
use remonitor_linter::{lint, Report};
use serde::Serialize;
use std::path::Path;
use std::time::Instant;
use tempdir::TempDir;
use tokio::process::Command;
use tokio_postgres::types::Json;
use tokio_postgres::{Error as DbError, NoTls};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Supported linters.
#[derive(Debug)]
enum Linter {
    Core = 0,
}

impl std::convert::TryFrom<i32> for Linter {
    type Error = Error;

    fn try_from(linter_id: i32) -> Result<Self, Self::Error> {
        match linter_id {
            0 => Ok(Linter::Core),
            _ => Err(format_err!("invalid linter id")),
        }
    }
}

/// Repository information.
#[derive(Debug)]
struct Repository {
    repository_id: Uuid,
    url: String,
    digest: Option<String>,
}

/// Project's score information.
#[derive(Debug, Serialize)]
struct Score {
    global: usize,
    documentation: usize,
    license: usize,
    quality: usize,
    security: usize,
}

impl Score {
    fn new() -> Self {
        Score {
            global: 0,
            documentation: 0,
            license: 0,
            quality: 0,
            security: 0,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set RUST_LOG if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "remonitor_tracker=debug")
    }
    tracing_subscriber::fmt::init();
    info!("tracker started");

    // Check if required external tools are available
    // TODO

    // Setup configuration
    let cfg_dir = dirs::config_dir()
        .expect("config dir not found")
        .join("remonitor");
    let mut cfg = Config::new();
    cfg.set_default("db.dbname", "remonitor")?;
    cfg.set_default("tracker.concurrency", 10)?;
    cfg.merge(File::from(cfg_dir.join("tracker")))?;

    // Setup database
    let db_cfg: DbConfig = cfg.get("db").unwrap();
    let db_pool = db_cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;

    // Get repositories to process
    let repositories = get_repositories(db_pool.get().await?).await?;
    if repositories.is_empty() {
        info!("no repositories found");
        info!("tracker finished");
        return Ok(());
    }

    // Process repositories
    info!("processing repositories");
    let mut futs = FuturesUnordered::new();
    for repo in repositories {
        let db = db_pool.get().await?;
        futs.push(tokio::spawn(async move {
            let repository_id = repo.repository_id.to_string();
            if let Err(err) = process_repository(db, repo).await {
                error!("error processing repository {}: {}", repository_id, err);
            }
        }));
        if futs.len() == cfg.get::<usize>("tracker.concurrency").unwrap() {
            futs.next().await;
        }
    }
    future::join_all(futs).await;

    info!("tracker finished");
    Ok(())
}

/// Get all repositories available in the database.
async fn get_repositories(db: DbClient) -> Result<Vec<Repository>, DbError> {
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
async fn process_repository(mut db: DbClient, repo: Repository) -> Result<(), Error> {
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
    let tmp_dir = TempDir::new("remonitor")?;
    clone_repository(&repo.url, tmp_dir.path()).await?;

    // Lint repository
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
    update_repository_digest(&tx, &repo.repository_id, &remote_digest).await?;
    store_linter_report(&tx, &repo.repository_id, Linter::Core, report, errors).await?;
    update_project_score(&tx, &repo.repository_id).await?;
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

/// Store the provided linter report.
async fn store_linter_report(
    tx: &Transaction<'_>,
    repository_id: &Uuid,
    linter: Linter,
    report: Option<Report>,
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

/// Update project's score based on the linter reports available for each of
/// the repositories in the project.
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

    // Calculate project's score from the project's repositories linters reports
    let mut scores = Vec::<Score>::new();
    let rows = tx
        .query(
            "
            select linter_id, data from report
            where repository_id in (
                select repository_id from repository where project_id = $1::uuid
            );
            ",
            &[&project_id],
        )
        .await?;
    for row in rows {
        let linter_id: i32 = row.get("linter_id");
        let linter = Linter::try_from(linter_id)?;
        let report: Json<Report> = row.get("data");
        scores.push(calculate_score(linter, report.0));
    }
    let score = merge_scores(scores);

    // Update project's score
    tx.execute(
        "
        update project set
            score = $1::jsonb,
            updated_at = current_timestamp
        where project_id = $2::uuid;
        ",
        &[&Json(score), &project_id],
    )
    .await?;

    Ok(())
}

/// Calculate score for the given linter report.
fn calculate_score(linter: Linter, report: Report) -> Score {
    match linter {
        Linter::Core => calculate_score_core_linter(report),
    }
}

/// Calculate score for the given report produced by the core linter.
fn calculate_score_core_linter(report: Report) -> Score {
    let mut score = Score::new();

    // Documentation
    if report.documentation.adopters {
        score.documentation += 5;
    }
    if report.documentation.code_of_conduct {
        score.documentation += 10;
    }
    if report.documentation.contributing {
        score.documentation += 10;
    }
    if report.documentation.changelog {
        score.documentation += 5;
    }
    if report.documentation.governance {
        score.documentation += 10;
    }
    if report.documentation.maintainers || report.documentation.owners {
        score.documentation += 5;
    }
    if report.documentation.readme {
        score.documentation += 50;
    }
    if report.documentation.roadmap {
        score.documentation += 5;
    }

    // License
    if report.license.spdx_id.is_some() {
        score.license += 25;
    }
    if let Some(approved) = report.license.approved {
        if approved {
            score.license += 75;
        }
    }

    // Quality
    if report.quality.fossa {
        score.quality += 25;
    }
    if report.quality.openssf_badge {
        score.quality += 75;
    }

    // Security
    if report.security.security_policy {
        score.security = 100;
    }

    // Global
    score.global = (score.documentation + score.license + score.quality + score.security) / 4;

    score
}

/// Merge the scores provided into a single score.
fn merge_scores(scores: Vec<Score>) -> Score {
    let mut score = Score::new();
    for entry in &scores {
        score.global += entry.global;
        score.documentation += entry.documentation;
        score.license += entry.license;
        score.quality += entry.quality;
        score.security += entry.security;
    }
    let n = scores.len();
    score.global /= n;
    score.documentation /= n;
    score.license /= n;
    score.quality /= n;
    score.security /= n;
    score
}
