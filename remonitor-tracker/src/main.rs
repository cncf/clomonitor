use anyhow::{format_err, Error};
use config::{Config, File};
use deadpool_postgres::{Client as DbClient, Config as DbConfig, Runtime};
use futures::future;
use futures::stream::{FuturesUnordered, StreamExt};
use remonitor_linter::lint;
use std::path::Path;
use std::time::Instant;
use tempdir::TempDir;
use tokio::process::Command;
use tokio_postgres::{Error as DbError, NoTls};
use tracing::{error, info, warn};
use uuid::Uuid;

/// Repository information.
#[derive(Debug)]
struct Repository {
    repository_id: Uuid,
    url: String,
    digest: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
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
    info!("getting repositories");
    let mut repositories: Vec<Repository> = Vec::new();
    let rows = db
        .query("select repository_id, url, digest from repository", &[])
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
async fn process_repository(db: DbClient, repo: Repository) -> Result<(), Error> {
    let start = Instant::now();

    // Skip if repository hasn't changed since the last time it was processed
    let remote_digest = get_remote_digest(&repo.url).await?;
    if let Some(digest) = repo.digest {
        if remote_digest == digest {
            return Ok(());
        }
    }

    info!("processing repository [id: {}]", repo.repository_id);

    // Clone repository
    let wd = TempDir::new("remonitor")?;
    clone_repository(&repo.url, wd.path()).await?;

    // Lint repository
    let mut report_json: Option<serde_json::Value> = None;
    let mut errors: Option<String> = None;
    match lint(wd.path()) {
        Ok(report) => report_json = Some(serde_json::to_value(&report)?),
        Err(err) => {
            warn!(
                "error linting repository [id: {}]: {}",
                repo.repository_id,
                err.to_string()
            );
            errors = Some(err.to_string())
        }
    }

    // Store linter report in database
    store_report(
        db,
        &repo.repository_id,
        0, // Core linter, hardcoded for now
        &remote_digest,
        report_json,
        errors,
    )
    .await?;

    info!(
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

/// Clone a repository in the destination path provided.
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

/// Store the provided linter report in the database.
async fn store_report(
    mut db: DbClient,
    repository_id: &Uuid,
    linter_id: i32,
    digest: &str,
    report: Option<serde_json::Value>,
    errors: Option<String>,
) -> Result<(), Error> {
    let tx = db.transaction().await?;

    // Update repository's digest
    tx.execute(
        "update repository set digest = $1 where repository_id = $2",
        &[&digest, &repository_id],
    )
    .await?;

    // Store (insert or update) linter report
    tx.execute(
        "
        insert into report
            (data, errors, repository_id, linter_id)
        values
            ($1, $2, $3, $4)
        on conflict (repository_id, linter_id) do update
        set
            data = excluded.data,
            errors = excluded.errors,
            updated_at = current_timestamp;
        ",
        &[&report, &errors, &repository_id, &linter_id],
    )
    .await?;

    tx.commit().await?;
    Ok(())
}
