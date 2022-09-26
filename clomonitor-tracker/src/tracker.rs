use crate::{db::DynDB, git::DynGit};
use anyhow::{format_err, Error, Result};
use clomonitor_core::linter::{CheckSet, DynLinter, LinterInput};
use config::Config;
use deadpool::unmanaged::{Object, Pool};
use futures::stream::{self, StreamExt};
use std::time::{Duration, Instant};
use tempfile::Builder;
use time::{self, OffsetDateTime};
use tokio::{task::JoinError, time::timeout};
use tracing::{debug, error, info, instrument, warn};
use uuid::Uuid;

/// Maximum time that can take tracking a single repository.
const REPOSITORY_TRACK_TIMEOUT: u64 = 600;

/// A project's repository.
#[derive(Debug, Clone)]
pub(crate) struct Repository {
    pub repository_id: Uuid,
    pub url: String,
    pub check_sets: Vec<CheckSet>,
    pub digest: Option<String>,
    pub updated_at: OffsetDateTime,
}

/// Track all repositories registered in the database.
pub(crate) async fn run(cfg: &Config, db: DynDB, git: DynGit, linter: DynLinter) -> Result<()> {
    info!("tracker started");

    // Setup GitHub tokens pool
    let gh_tokens = cfg.get::<Vec<String>>("creds.githubTokens")?;
    if gh_tokens.is_empty() {
        return Err(format_err!(
            "GitHub tokens not found in config file (creds.githubTokens)"
        ));
    }
    let gh_tokens_pool = Pool::from(gh_tokens.clone());

    // Get repositories to process
    debug!("getting repositories");
    let repositories = db.repositories().await?;
    if repositories.is_empty() {
        info!("no repositories found");
        info!("tracker finished");
        return Ok(());
    }

    // Track repositories
    info!("tracking repositories");
    let result = stream::iter(repositories)
        .map(|repository| async {
            let db = db.clone();
            let git = git.clone();
            let linter = linter.clone();
            let github_token = gh_tokens_pool.get().await.expect("token -when available-");
            let repository_id = repository.repository_id;

            tokio::spawn(async move {
                match timeout(
                    Duration::from_secs(REPOSITORY_TRACK_TIMEOUT),
                    track_repository(db, git, linter, github_token, repository),
                )
                .await
                {
                    Ok(result) => match result {
                        Ok(()) => {}
                        Err(err) => {
                            error!("error tracking repository {}: {:#}", repository_id, err)
                        }
                    },
                    Err(err) => {
                        warn!("timeout tracking repository {}: {}", repository_id, err)
                    }
                }
            })
            .await
        })
        .buffer_unordered(cfg.get("tracker.concurrency")?)
        .collect::<Vec<Result<(), JoinError>>>()
        .await
        .into_iter()
        .fold(
            Ok::<(), Error>(()),
            |final_result, task_result| match task_result {
                Ok(()) => final_result,
                Err(task_err) => match final_result {
                    Ok(()) => Err(task_err).map_err(Into::into),
                    Err(final_err) => Err(format_err!("{:#}\n{:#}", final_err, task_err)),
                },
            },
        );

    info!("tracker finished");
    result
}

/// Track repository if it has changed since the last time it was tracked.
/// This involves cloning the repository, linting it and storing the results.
#[instrument(fields(repository_id = %repository.repository_id), skip_all, err)]
async fn track_repository(
    db: DynDB,
    git: DynGit,
    linter: DynLinter,
    github_token: Object<String>,
    repository: Repository,
) -> Result<()> {
    let start = Instant::now();

    // Process only if the repository has changed since the last time it
    // was tracked or if it hasn't been tracked in more than 1 day
    let remote_digest = git.remote_digest(&repository.url).await?;
    if let Some(digest) = &repository.digest {
        let one_day_ago = OffsetDateTime::now_utc() - time::Duration::days(1);
        if &remote_digest == digest && repository.updated_at > one_day_ago {
            return Ok(());
        }
    }

    debug!("started");

    // Clone repository
    let tmp_dir = Builder::new().prefix("clomonitor").tempdir()?;
    git.clone_repository(&repository.url, tmp_dir.path())
        .await?;

    // Lint repository
    let mut errors: Option<String> = None;
    let input = LinterInput {
        root: tmp_dir.into_path(),
        url: repository.url.clone(),
        check_sets: repository.check_sets.clone(),
        github_token: github_token.to_owned(),
    };
    let report = match linter.lint(&input).await {
        Ok(report) => Some(report),
        Err(err) => {
            warn!("error linting repository: {:#}", err);
            errors = Some(format!("error linting repository: {:#}", err));
            None
        }
    };

    // Store tracking results in database
    db.store_results(
        &repository.repository_id,
        report.as_ref(),
        errors.as_ref(),
        &remote_digest,
    )
    .await?;

    debug!("completed in {}s", start.elapsed().as_secs());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::MockDB, git::MockGit};
    use clomonitor_core::linter::{MockLinter, Report};
    use futures::future;
    use predicates::prelude::{predicate::*, *};
    use std::{path::Path, sync::Arc};

    #[tokio::test]
    async fn error_getting_github_tokens() {
        let cfg = Config::builder().build().unwrap();
        let db = MockDB::new();
        let git = MockGit::new();
        let linter = MockLinter::new();

        let result = run(&cfg, Arc::new(db), Arc::new(git), Arc::new(linter)).await;
        assert_eq!(
            result.unwrap_err().to_string(),
            r#"configuration property "creds.githubTokens" not found"#
        );
    }

    #[tokio::test]
    async fn empty_list_of_github_tokens_provided() {
        let cfg = Config::builder()
            .set_default("creds.githubTokens", Vec::<String>::new())
            .unwrap()
            .build()
            .unwrap();
        let db = MockDB::new();
        let git = MockGit::new();
        let linter = MockLinter::new();

        let result = run(&cfg, Arc::new(db), Arc::new(git), Arc::new(linter)).await;
        assert_eq!(
            result.unwrap_err().to_string(),
            "GitHub tokens not found in config file (creds.githubTokens)"
        );
    }

    #[tokio::test]
    async fn error_getting_repositories() {
        let cfg = Config::builder()
            .set_default("creds.githubTokens", vec!["0000".to_string()])
            .unwrap()
            .build()
            .unwrap();
        let mut db = MockDB::new();
        let git = MockGit::new();
        let linter = MockLinter::new();

        db.expect_repositories()
            .times(1)
            .returning(|| Box::pin(future::ready(Err(format_err!("fake error")))));

        let result = run(&cfg, Arc::new(db), Arc::new(git), Arc::new(linter)).await;
        assert_eq!(result.unwrap_err().to_string(), "fake error");
    }

    #[tokio::test]
    async fn no_repositories_found() {
        let cfg = Config::builder()
            .set_default("creds.githubTokens", vec!["0000".to_string()])
            .unwrap()
            .build()
            .unwrap();
        let mut db = MockDB::new();
        let git = MockGit::new();
        let linter = MockLinter::new();

        db.expect_repositories()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));

        run(&cfg, Arc::new(db), Arc::new(git), Arc::new(linter))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn error_getting_repository_digest() {
        let cfg = Config::builder()
            .set_default("tracker.concurrency", 1)
            .unwrap()
            .set_default("creds.githubTokens", vec!["0000".to_string()])
            .unwrap()
            .build()
            .unwrap();
        let mut db = MockDB::new();
        let mut git = MockGit::new();
        let linter = MockLinter::new();

        let r1_id = "00000000-0000-0000-0000-000000000001";
        let r1_url = "url1";
        db.expect_repositories().times(1).returning(|| {
            Box::pin(future::ready(Ok(vec![Repository {
                repository_id: Uuid::parse_str(r1_id).unwrap(),
                url: r1_url.to_string(),
                check_sets: vec![CheckSet::Code],
                digest: None,
                updated_at: OffsetDateTime::now_utc() - time::Duration::hours(6),
            }])))
        });
        git.expect_remote_digest()
            .with(eq(r1_url))
            .times(1)
            .returning(|_: &str| Box::pin(future::ready(Err(format_err!("fake error")))));

        run(&cfg, Arc::new(db), Arc::new(git), Arc::new(linter))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn repository_has_not_changed_and_was_tracked_within_last_day() {
        let cfg = Config::builder()
            .set_default("tracker.concurrency", 1)
            .unwrap()
            .set_default("creds.githubTokens", vec!["0000".to_string()])
            .unwrap()
            .build()
            .unwrap();
        let mut db = MockDB::new();
        let mut git = MockGit::new();
        let linter = MockLinter::new();

        let r1_id = "00000000-0000-0000-0000-000000000001";
        let r1_url = "url1";
        db.expect_repositories().times(1).returning(|| {
            Box::pin(future::ready(Ok(vec![Repository {
                repository_id: Uuid::parse_str(r1_id).unwrap(),
                url: r1_url.to_string(),
                check_sets: vec![CheckSet::Code],
                digest: Some("r1_digest".to_string()),
                updated_at: OffsetDateTime::now_utc() - time::Duration::hours(6),
            }])))
        });
        git.expect_remote_digest()
            .with(eq(r1_url))
            .times(1)
            .returning(|_: &str| Box::pin(future::ready(Ok("r1_digest".to_string()))));

        run(&cfg, Arc::new(db), Arc::new(git), Arc::new(linter))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn error_cloning_repository() {
        let cfg = Config::builder()
            .set_default("tracker.concurrency", 1)
            .unwrap()
            .set_default("creds.githubTokens", vec!["0000".to_string()])
            .unwrap()
            .build()
            .unwrap();
        let mut db = MockDB::new();
        let mut git = MockGit::new();
        let linter = MockLinter::new();

        let r1_id = "00000000-0000-0000-0000-000000000001";
        let r1_url = "url1";
        db.expect_repositories().times(1).returning(|| {
            Box::pin(future::ready(Ok(vec![Repository {
                repository_id: Uuid::parse_str(r1_id).unwrap(),
                url: r1_url.to_string(),
                check_sets: vec![CheckSet::Code],
                digest: None,
                updated_at: OffsetDateTime::now_utc() - time::Duration::hours(6),
            }])))
        });
        git.expect_remote_digest()
            .with(eq(r1_url))
            .times(1)
            .returning(|_: &str| Box::pin(future::ready(Ok("r1_digest".to_string()))));
        git.expect_clone_repository()
            .with(eq(r1_url), path::exists().and(path::is_dir()))
            .times(1)
            .returning(|_: &str, _: &Path| Box::pin(future::ready(Err(format_err!("fake error")))));

        run(&cfg, Arc::new(db), Arc::new(git), Arc::new(linter))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn panic_linting_repository() {
        let cfg = Config::builder()
            .set_default("tracker.concurrency", 1)
            .unwrap()
            .set_default("creds.githubTokens", vec!["0000".to_string()])
            .unwrap()
            .build()
            .unwrap();
        let mut db = MockDB::new();
        let mut git = MockGit::new();
        let mut linter = MockLinter::new();

        let r1_id = "00000000-0000-0000-0000-000000000001";
        let r1_url = "url1";
        db.expect_repositories().times(1).returning(|| {
            Box::pin(future::ready(Ok(vec![Repository {
                repository_id: Uuid::parse_str(r1_id).unwrap(),
                url: r1_url.to_string(),
                check_sets: vec![CheckSet::Code],
                digest: None,
                updated_at: OffsetDateTime::now_utc() - time::Duration::hours(6),
            }])))
        });
        git.expect_remote_digest()
            .with(eq(r1_url))
            .times(1)
            .returning(|_: &str| Box::pin(future::ready(Ok("r1_digest".to_string()))));
        git.expect_clone_repository()
            .with(eq(r1_url), path::exists().and(path::is_dir()))
            .times(1)
            .returning(|_: &str, _: &Path| Box::pin(future::ready(Ok(()))));
        linter
            .expect_lint()
            .withf(move |input: &LinterInput| {
                path::exists().and(path::is_dir()).eval(&input.root)
                    && input.url == r1_url
                    && input.check_sets == vec![CheckSet::Code]
                    && input.github_token == "0000"
            })
            .times(1)
            .returning(|_: &LinterInput| panic!("fake panic"));

        run(&cfg, Arc::new(db), Arc::new(git), Arc::new(linter))
            .await
            .unwrap_err();
    }

    #[tokio::test]
    async fn two_repos_tracked_successfully() {
        // Setup config
        let github_tokens = vec!["0000".to_string(), "1111".to_string()];
        let cfg = Config::builder()
            .set_default("tracker.concurrency", 2)
            .unwrap()
            .set_default("creds.githubTokens", github_tokens.clone())
            .unwrap()
            .build()
            .unwrap();

        // Setup mocks and expectations
        let mut db = MockDB::new();
        let mut git = MockGit::new();
        let mut linter = MockLinter::new();

        // Get repositories
        let r1_id = "00000000-0000-0000-0000-000000000001";
        let r1_url = "url1";
        let r2_id = "00000000-0000-0000-0000-000000000002";
        let r2_url = "url2";
        db.expect_repositories().times(1).returning(|| {
            Box::pin(future::ready(Ok(vec![
                Repository {
                    repository_id: Uuid::parse_str(r1_id).unwrap(),
                    url: r1_url.to_string(),
                    check_sets: vec![CheckSet::Code],
                    digest: None,
                    updated_at: OffsetDateTime::now_utc() - time::Duration::days(7),
                },
                Repository {
                    repository_id: Uuid::parse_str(r2_id).unwrap(),
                    url: r2_url.to_string(),
                    check_sets: vec![CheckSet::Code],
                    digest: None,
                    updated_at: OffsetDateTime::now_utc() - time::Duration::days(7),
                },
            ])))
        });

        // Track repository 1
        let github_tokens_copy = github_tokens.clone();
        git.expect_remote_digest()
            .with(eq(r1_url))
            .times(1)
            .returning(|_: &str| Box::pin(future::ready(Ok("r1_digest".to_string()))));
        git.expect_clone_repository()
            .with(eq(r1_url), path::exists().and(path::is_dir()))
            .times(1)
            .returning(|_: &str, _: &Path| Box::pin(future::ready(Ok(()))));
        linter
            .expect_lint()
            .withf(move |input: &LinterInput| {
                path::exists().and(path::is_dir()).eval(&input.root)
                    && input.url == r1_url
                    && input.check_sets == vec![CheckSet::Code]
                    && github_tokens_copy.contains(&input.github_token)
            })
            .times(1)
            .returning(|_: &LinterInput| Box::pin(future::ready(Ok(Report::default()))));
        db.expect_store_results()
            .withf(|repository_id, report, errors, digest| {
                *repository_id == Uuid::parse_str(r1_id).unwrap()
                    && *report == Some(&Report::default())
                    && errors.is_none()
                    && digest == "r1_digest"
            })
            .times(1)
            .returning(
                |_: &Uuid, _: Option<&Report>, _: Option<&String>, _: &str| {
                    Box::pin(future::ready(Ok(())))
                },
            );

        // Track repository 2
        git.expect_remote_digest()
            .with(eq(r2_url))
            .times(1)
            .returning(|_: &str| Box::pin(future::ready(Ok("r2_digest".to_string()))));
        git.expect_clone_repository()
            .with(eq(r2_url), path::exists().and(path::is_dir()))
            .times(1)
            .returning(|_: &str, _: &Path| Box::pin(future::ready(Ok(()))));
        linter
            .expect_lint()
            .withf(move |input: &LinterInput| {
                path::exists().and(path::is_dir()).eval(&input.root)
                    && input.url == r2_url
                    && input.check_sets == vec![CheckSet::Code]
                    && github_tokens.contains(&input.github_token)
            })
            .times(1)
            .returning(|_: &LinterInput| Box::pin(future::ready(Err(format_err!("fake error")))));
        db.expect_store_results()
            .withf(|repository_id, report, errors, digest| {
                *repository_id == Uuid::parse_str(r2_id).unwrap()
                    && report.is_none()
                    && *errors == Some(&"error linting repository: fake error".to_string())
                    && digest == "r2_digest"
            })
            .times(1)
            .returning(
                |_: &Uuid, _: Option<&Report>, _: Option<&String>, _: &str| {
                    Box::pin(future::ready(Ok(())))
                },
            );

        // Run tracker
        run(&cfg, Arc::new(db), Arc::new(git), Arc::new(linter))
            .await
            .unwrap();
    }
}
