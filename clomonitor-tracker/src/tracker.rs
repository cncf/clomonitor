use std::time::{Duration, Instant};

use anyhow::{Error, Result, format_err};
#[cfg(not(test))]
use clomonitor_core::linter::setup_github_http_client;
use clomonitor_core::linter::{CheckSet, DynLinter, LinterInput, Project, ToolsConfig};
use config::Config;
use deadpool::unmanaged::{Object, Pool};
use futures::stream::{self, StreamExt};
#[cfg(not(test))]
use serde_json::Value;
use tempfile::Builder;
use time::{self, OffsetDateTime};
use tokio::{task::JoinError, time::timeout};
use tracing::{debug, error, info, instrument, warn};
use uuid::Uuid;

use crate::{db::DynDB, git::DynGit};

/// Maximum time that can take tracking a single repository.
const REPOSITORY_TRACK_TIMEOUT: u64 = 600;

/// A project's repository.
#[derive(Debug, Clone)]
#[allow(clippy::struct_field_names)]
pub(crate) struct Repository {
    pub repository_id: Uuid,
    pub url: String,
    pub check_sets: Vec<CheckSet>,
    pub digest: Option<String>,
    pub updated_at: OffsetDateTime,
    pub project: Project,
}

/// Track all repositories registered in the database.
#[instrument(skip_all, err)]
pub(crate) async fn run(
    cfg: &Config,
    db: DynDB,
    git: DynGit,
    linter: DynLinter,
    tools: ToolsConfig,
) -> Result<()> {
    info!("started");

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
        info!("no repositories found, finished");
        return Ok(());
    }

    // Track repositories
    info!("tracking repositories");
    #[allow(clippy::manual_try_fold)]
    let result = stream::iter(repositories)
        .map(|repository| async {
            let db = db.clone();
            let git = git.clone();
            let linter = linter.clone();
            let github_token = gh_tokens_pool.get().await.expect("token -when available-");
            let url = repository.url.clone();
            let tools = tools.clone();

            tokio::spawn(async move {
                match timeout(
                    Duration::from_secs(REPOSITORY_TRACK_TIMEOUT),
                    track_repository(db, git, linter, github_token, tools, repository),
                )
                .await
                {
                    Ok(result) => match result {
                        Ok(()) => {}
                        Err(err) => {
                            error!(?err, url, "error tracking");
                        }
                    },
                    Err(err) => {
                        warn!(?err, url, "timeout tracking");
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
                    Ok(()) => Err(Into::into(task_err)),
                    Err(final_err) => Err(format_err!("{final_err:#}\n{task_err:#}")),
                },
            },
        );

    // Check Github API rate limit status for each token
    #[cfg(not(test))]
    for (i, token) in gh_tokens.into_iter().enumerate() {
        let gh_client = setup_github_http_client(&token)?;
        let response: Value = gh_client
            .get("https://api.github.com/rate_limit")
            .send()
            .await?
            .json()
            .await?;
        debug!(
            token = i,
            rate = %response["rate"],
            graphql = %response["resources"]["graphql"],
            "token github rate limit info"
        );
    }

    info!("finished");
    result
}

/// Track repository if it has changed since the last time it was tracked.
/// This involves cloning the repository, linting it and storing the results.
#[instrument(fields(url = repository.url), skip_all, err)]
async fn track_repository(
    db: DynDB,
    git: DynGit,
    linter: DynLinter,
    github_token: Object<String>,
    tools: ToolsConfig,
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

    // Clone repository (the temporary directory is removed when the guard is
    // dropped at the end of this function)
    let tmp_dir = Builder::new().prefix("clomonitor").tempdir()?;
    git.clone_repository(&repository.url, tmp_dir.path())
        .await?;

    // Lint repository
    let mut errors: Option<String> = None;
    let input = LinterInput {
        project: Some(repository.project),
        root: tmp_dir.path().to_path_buf(),
        url: repository.url.clone(),
        check_sets: repository.check_sets.clone(),
        github_token: github_token.to_owned(),
        tools,
    };
    let report = match linter.lint(&input).await {
        Ok(report) => Some(report),
        Err(err) => {
            warn!(?err, "error linting repository");
            errors = Some(format!("error linting repository: {err:#}"));
            None
        }
    };

    // Store tracking results in database
    db.store_results(
        &repository.repository_id,
        &repository.check_sets,
        report.as_ref(),
        errors.as_ref(),
        &remote_digest,
    )
    .await?;

    // Remove cloned repository
    drop(github_token);
    tmp_dir.close()?;

    debug!(duration_secs = start.elapsed().as_secs(), "completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        path::{Path, PathBuf},
        sync::{Arc, LazyLock, Mutex},
    };

    use clomonitor_core::linter::{MockLinter, Report, ToolMode};
    use futures::future;
    use predicates::prelude::{predicate::*, *};

    use crate::{db::MockDB, git::MockGit};

    use super::*;

    const TOKEN1: &str = "0001";
    const TOKEN2: &str = "0002";
    const REPOSITORY1_URL: &str = "https://repo1.url";
    const REPOSITORY2_URL: &str = "https://repo2.url";
    const REPOSITORY1_DIGEST: &str = "repo1_digest";
    const REPOSITORY2_DIGEST: &str = "repo2_digest";
    const FAKE_ERROR: &str = "fake error";

    static REPOSITORY1_ID: LazyLock<Uuid> =
        LazyLock::new(|| Uuid::parse_str("00000000-0001-0000-0000-000000000000").unwrap());
    static REPOSITORY2_ID: LazyLock<Uuid> =
        LazyLock::new(|| Uuid::parse_str("00000000-0002-0000-0000-000000000000").unwrap());

    #[tokio::test]
    async fn error_getting_github_tokens() {
        let cfg = Config::builder().build().unwrap();
        let db = MockDB::new();
        let git = MockGit::new();
        let linter = MockLinter::new();

        let result = run(
            &cfg,
            Arc::new(db),
            Arc::new(git),
            Arc::new(linter),
            ToolsConfig::default(),
        )
        .await;
        assert_eq!(
            result.unwrap_err().root_cause().to_string(),
            r#"missing configuration field "creds.githubTokens""#
        );
    }

    #[tokio::test]
    async fn empty_list_of_github_tokens_provided() {
        let cfg = setup_test_config(1, &[]);
        let db = MockDB::new();
        let git = MockGit::new();
        let linter = MockLinter::new();

        let result = run(
            &cfg,
            Arc::new(db),
            Arc::new(git),
            Arc::new(linter),
            ToolsConfig::default(),
        )
        .await;
        assert_eq!(
            result.unwrap_err().root_cause().to_string(),
            "GitHub tokens not found in config file (creds.githubTokens)"
        );
    }

    #[tokio::test]
    async fn error_getting_repositories() {
        let cfg = setup_test_config(1, &[TOKEN1]);
        let mut db = MockDB::new();
        let git = MockGit::new();
        let linter = MockLinter::new();

        db.expect_repositories()
            .times(1)
            .returning(|| Box::pin(future::ready(Err(format_err!(FAKE_ERROR)))));

        let result = run(
            &cfg,
            Arc::new(db),
            Arc::new(git),
            Arc::new(linter),
            ToolsConfig::default(),
        )
        .await;
        assert_eq!(result.unwrap_err().root_cause().to_string(), FAKE_ERROR);
    }

    #[tokio::test]
    async fn no_repositories_found() {
        let cfg = setup_test_config(1, &[TOKEN1]);
        let mut db = MockDB::new();
        let git = MockGit::new();
        let linter = MockLinter::new();

        db.expect_repositories()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));

        run(
            &cfg,
            Arc::new(db),
            Arc::new(git),
            Arc::new(linter),
            ToolsConfig::default(),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn error_getting_repository_digest() {
        let cfg = setup_test_config(1, &[TOKEN1]);
        let mut db = MockDB::new();
        let mut git = MockGit::new();
        let linter = MockLinter::new();

        db.expect_repositories().times(1).returning(|| {
            Box::pin(future::ready(Ok(vec![Repository {
                repository_id: *REPOSITORY1_ID,
                url: REPOSITORY1_URL.to_string(),
                check_sets: vec![CheckSet::Code],
                digest: None,
                updated_at: OffsetDateTime::now_utc() - time::Duration::hours(6),
                project: Project::default(),
            }])))
        });
        git.expect_remote_digest()
            .with(eq(REPOSITORY1_URL))
            .times(1)
            .returning(|_: &str| Box::pin(future::ready(Err(format_err!(FAKE_ERROR)))));

        run(
            &cfg,
            Arc::new(db),
            Arc::new(git),
            Arc::new(linter),
            ToolsConfig::default(),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn repository_has_not_changed_and_was_tracked_within_last_day() {
        let cfg = setup_test_config(1, &[TOKEN1]);
        let mut db = MockDB::new();
        let mut git = MockGit::new();
        let linter = MockLinter::new();

        db.expect_repositories().times(1).returning(|| {
            Box::pin(future::ready(Ok(vec![Repository {
                repository_id: *REPOSITORY1_ID,
                url: REPOSITORY1_URL.to_string(),
                check_sets: vec![CheckSet::Code],
                digest: Some(REPOSITORY1_DIGEST.to_string()),
                updated_at: OffsetDateTime::now_utc() - time::Duration::hours(6),
                project: Project::default(),
            }])))
        });
        git.expect_remote_digest()
            .with(eq(REPOSITORY1_URL))
            .times(1)
            .returning(|_: &str| Box::pin(future::ready(Ok(REPOSITORY1_DIGEST.to_string()))));

        run(
            &cfg,
            Arc::new(db),
            Arc::new(git),
            Arc::new(linter),
            ToolsConfig::default(),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn error_cloning_repository() {
        let cfg = setup_test_config(1, &[TOKEN1]);
        let mut db = MockDB::new();
        let mut git = MockGit::new();
        let linter = MockLinter::new();

        db.expect_repositories().times(1).returning(|| {
            Box::pin(future::ready(Ok(vec![Repository {
                repository_id: *REPOSITORY1_ID,
                url: REPOSITORY1_URL.to_string(),
                check_sets: vec![CheckSet::Code],
                digest: None,
                updated_at: OffsetDateTime::now_utc() - time::Duration::hours(6),
                project: Project::default(),
            }])))
        });
        git.expect_remote_digest()
            .with(eq(REPOSITORY1_URL))
            .times(1)
            .returning(|_: &str| Box::pin(future::ready(Ok(REPOSITORY1_DIGEST.to_string()))));
        git.expect_clone_repository()
            .with(eq(REPOSITORY1_URL), path::exists().and(path::is_dir()))
            .times(1)
            .returning(|_: &str, _: &Path| Box::pin(future::ready(Err(format_err!(FAKE_ERROR)))));

        run(
            &cfg,
            Arc::new(db),
            Arc::new(git),
            Arc::new(linter),
            ToolsConfig::default(),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn panic_linting_repository() {
        let cfg = setup_test_config(1, &[TOKEN1]);
        let mut db = MockDB::new();
        let mut git = MockGit::new();
        let mut linter = MockLinter::new();

        db.expect_repositories().times(1).returning(|| {
            Box::pin(future::ready(Ok(vec![Repository {
                repository_id: *REPOSITORY1_ID,
                url: REPOSITORY1_URL.to_string(),
                check_sets: vec![CheckSet::Code],
                digest: None,
                updated_at: OffsetDateTime::now_utc() - time::Duration::hours(6),
                project: Project::default(),
            }])))
        });
        git.expect_remote_digest()
            .with(eq(REPOSITORY1_URL))
            .times(1)
            .returning(|_: &str| Box::pin(future::ready(Ok(REPOSITORY1_DIGEST.to_string()))));
        git.expect_clone_repository()
            .with(eq(REPOSITORY1_URL), path::exists().and(path::is_dir()))
            .times(1)
            .returning(|_: &str, _: &Path| Box::pin(future::ready(Ok(()))));
        linter
            .expect_lint()
            .withf(move |input: &LinterInput| {
                path::exists().and(path::is_dir()).eval(&input.root)
                    && input.url == REPOSITORY1_URL
                    && input.check_sets == vec![CheckSet::Code]
                    && input.github_token == TOKEN1
            })
            .times(1)
            .returning(|_: &LinterInput| panic!("fake panic"));

        run(
            &cfg,
            Arc::new(db),
            Arc::new(git),
            Arc::new(linter),
            ToolsConfig::default(),
        )
        .await
        .unwrap_err();
    }

    #[tokio::test]
    #[allow(clippy::too_many_lines)]
    async fn two_repos_tracked_successfully() {
        // Setup config
        let cfg = setup_test_config(2, &[TOKEN1, TOKEN2]);
        let tools = ToolsConfig {
            afdocs: ToolMode::Runner {
                url: "http://runner-afdocs:8080".to_string(),
            },
            scorecard: ToolMode::Runner {
                url: "http://runner-scorecard:8080".to_string(),
            },
        };

        // Setup mocks and expectations
        let mut db = MockDB::new();
        let mut git = MockGit::new();
        let mut linter = MockLinter::new();

        // Cloned repositories paths (to check they are removed afterwards)
        let roots: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(vec![]));

        // Get repositories
        db.expect_repositories().times(1).returning(|| {
            Box::pin(future::ready(Ok(vec![
                Repository {
                    repository_id: *REPOSITORY1_ID,
                    url: REPOSITORY1_URL.to_string(),
                    check_sets: vec![CheckSet::Code],
                    digest: None,
                    updated_at: OffsetDateTime::now_utc() - time::Duration::days(7),
                    project: Project::default(),
                },
                Repository {
                    repository_id: *REPOSITORY2_ID,
                    url: REPOSITORY2_URL.to_string(),
                    check_sets: vec![CheckSet::Code],
                    digest: None,
                    updated_at: OffsetDateTime::now_utc() - time::Duration::days(7),
                    project: Project::default(),
                },
            ])))
        });

        // Track repository 1
        git.expect_remote_digest()
            .with(eq(REPOSITORY1_URL))
            .times(1)
            .returning(|_: &str| Box::pin(future::ready(Ok(REPOSITORY1_DIGEST.to_string()))));
        git.expect_clone_repository()
            .with(eq(REPOSITORY1_URL), path::exists().and(path::is_dir()))
            .times(1)
            .returning(|_: &str, _: &Path| Box::pin(future::ready(Ok(()))));
        let roots1 = roots.clone();
        let tools1 = tools.clone();
        linter
            .expect_lint()
            .withf(move |input: &LinterInput| {
                if input.url == REPOSITORY1_URL {
                    roots1.lock().unwrap().push(input.root.clone());
                }
                path::exists().and(path::is_dir()).eval(&input.root)
                    && input.url == REPOSITORY1_URL
                    && input.check_sets == vec![CheckSet::Code]
                    && [TOKEN1, TOKEN2].contains(&&input.github_token[..])
                    && input.tools == tools1
            })
            .times(1)
            .returning(|_: &LinterInput| Box::pin(future::ready(Ok(Report::default()))));
        db.expect_store_results()
            .withf(|repository_id, check_sets, report, errors, digest| {
                *repository_id == *REPOSITORY1_ID
                    && check_sets == [CheckSet::Code]
                    && *report == Some(&Report::default())
                    && errors.is_none()
                    && digest == REPOSITORY1_DIGEST
            })
            .times(1)
            .returning(
                |_: &Uuid, _: &[CheckSet], _: Option<&Report>, _: Option<&String>, _: &str| {
                    Box::pin(future::ready(Ok(())))
                },
            );

        // Track repository 2
        git.expect_remote_digest()
            .with(eq(REPOSITORY2_URL))
            .times(1)
            .returning(|_: &str| Box::pin(future::ready(Ok(REPOSITORY2_DIGEST.to_string()))));
        git.expect_clone_repository()
            .with(eq(REPOSITORY2_URL), path::exists().and(path::is_dir()))
            .times(1)
            .returning(|_: &str, _: &Path| Box::pin(future::ready(Ok(()))));
        let roots2 = roots.clone();
        let tools2 = tools.clone();
        linter
            .expect_lint()
            .withf(move |input: &LinterInput| {
                if input.url == REPOSITORY2_URL {
                    roots2.lock().unwrap().push(input.root.clone());
                }
                path::exists().and(path::is_dir()).eval(&input.root)
                    && input.url == REPOSITORY2_URL
                    && input.check_sets == vec![CheckSet::Code]
                    && [TOKEN1, TOKEN2].contains(&&input.github_token[..])
                    && input.tools == tools2
            })
            .times(1)
            .returning(|_: &LinterInput| Box::pin(future::ready(Err(format_err!(FAKE_ERROR)))));
        db.expect_store_results()
            .withf(|repository_id, check_sets, report, errors, digest| {
                *repository_id == *REPOSITORY2_ID
                    && check_sets == [CheckSet::Code]
                    && report.is_none()
                    && *errors == Some(&format!("error linting repository: {FAKE_ERROR}"))
                    && digest == REPOSITORY2_DIGEST
            })
            .times(1)
            .returning(
                |_: &Uuid, _: &[CheckSet], _: Option<&Report>, _: Option<&String>, _: &str| {
                    Box::pin(future::ready(Ok(())))
                },
            );

        // Run tracker
        run(&cfg, Arc::new(db), Arc::new(git), Arc::new(linter), tools)
            .await
            .unwrap();

        // Cloned repositories must have been removed (after a successful lint
        // and after a lint error)
        let roots = roots.lock().unwrap();
        assert_eq!(roots.len(), 2);
        for root in roots.iter() {
            assert!(!root.exists(), "{} was not removed", root.display());
        }
    }

    fn setup_test_config(concurrency: u8, tokens: &[&str]) -> Config {
        Config::builder()
            .set_default("tracker.concurrency", concurrency)
            .unwrap()
            .set_default(
                "creds.githubTokens",
                tokens
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>(),
            )
            .unwrap()
            .build()
            .unwrap()
    }
}
