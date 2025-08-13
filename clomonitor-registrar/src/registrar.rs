use std::{collections::HashMap, time::Duration};

use anyhow::{format_err, Context, Error, Result};
use config::Config;
use futures::stream::{self, StreamExt};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::time::{timeout, Instant};
use tracing::{debug, error, info, instrument};

use crate::db::DynDB;

/// Maximum time that can take processing a foundation data file.
const FOUNDATION_TIMEOUT: u64 = 300;

/// Represents a foundation registered in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Foundation {
    pub foundation_id: String,
    pub data_url: String,
}

/// Represents a project to be registered or updated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Project {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_dark_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub devstats_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,

    pub repositories: Vec<Repository>,
}

impl Project {
    fn set_digest(&mut self) -> Result<()> {
        let data = bincode::serde::encode_to_vec(&self, bincode::config::legacy())?;
        let digest = hex::encode(Sha256::digest(data));
        self.digest = Some(digest);
        Ok(())
    }
}

/// Represents a project's repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Repository {
    pub name: String,
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_sets: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
}

/// Process foundations registered in the database.
#[instrument(skip_all, err)]
pub(crate) async fn run(cfg: &Config, db: DynDB) -> Result<()> {
    info!("started");

    // Process foundations
    let http_client = reqwest::Client::new();
    let foundations = db.foundations().await?;
    #[allow(clippy::manual_try_fold)]
    let result = stream::iter(foundations)
        .map(|foundation| async {
            let foundation_id = foundation.foundation_id.clone();
            match timeout(
                Duration::from_secs(FOUNDATION_TIMEOUT),
                process_foundation(db.clone(), http_client.clone(), foundation),
            )
            .await
            {
                Ok(result) => result,
                Err(err) => Err(format_err!("{err}")),
            }
            .context(format!(
                "error processing foundation {foundation_id} data file",
            ))
        })
        .buffer_unordered(cfg.get("registrar.concurrency")?)
        .collect::<Vec<Result<()>>>()
        .await
        .into_iter()
        .fold(
            Ok::<(), Error>(()),
            |final_result, task_result| match task_result {
                Ok(()) => final_result,
                Err(task_err) => match final_result {
                    Ok(()) => Err(Into::into(task_err)),
                    Err(final_err) => Err(format_err!("{:#}\n{:#}", final_err, task_err)),
                },
            },
        );

    info!("finished");
    result
}

/// Process foundation's data file. New projects available will be registered
/// in the database and existing ones which have changed will be updated. When
/// a project is removed from the data file, it'll be removed from the database
/// as well.
#[instrument(fields(foundation = foundation.foundation_id), skip_all, err)]
async fn process_foundation(
    db: DynDB,
    http_client: reqwest::Client,
    foundation: Foundation,
) -> Result<()> {
    let start = Instant::now();
    debug!("started");

    // Fetch foundation data file
    let resp = http_client.get(foundation.data_url).send().await?;
    if resp.status() != StatusCode::OK {
        return Err(format_err!(
            "unexpected status code getting data file: {}",
            resp.status()
        ));
    }
    let data = resp.text().await?;

    // Get projects available in the data file
    let tmp: Vec<Project> = serde_yaml::from_str(&data)?;
    let mut projects_available: HashMap<String, Project> = HashMap::with_capacity(tmp.len());
    for mut project in tmp {
        // Do not include repositories that have been excluded for this service
        project.repositories.retain(|r| {
            if let Some(exclude) = &r.exclude {
                return !exclude.contains(&"clomonitor".to_string());
            }
            true
        });

        project.set_digest()?;
        projects_available.insert(project.name.clone(), project);
    }

    // Get projects registered in the database
    let foundation_id = &foundation.foundation_id;
    let projects_registered = db.foundation_projects(foundation_id).await?;

    // Register or update available projects as needed
    for (name, project) in &projects_available {
        // Check if the project is already registered
        if let Some(registered_digest) = projects_registered.get(name)
            && registered_digest == &project.digest
        {
            continue;
        }

        // Register project
        debug!(project = project.name, "registering");
        if let Err(err) = db.register_project(foundation_id, project).await {
            error!(?err, project = project.name, "error registering");
        }
    }

    // Unregister projects no longer available in the data file
    if !projects_available.is_empty() {
        for name in projects_registered.keys() {
            if !projects_available.contains_key(name) {
                debug!(project = name, "unregistering");
                if let Err(err) = db.unregister_project(foundation_id, name).await {
                    error!(?err, project = name, "error unregistering");
                }
            }
        }
    }

    debug!(duration_secs = start.elapsed().as_secs(), "completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use futures::future;
    use mockall::predicate::eq;
    use std::sync::Arc;

    use crate::db::MockDB;

    use super::*;

    const TESTDATA_PATH: &str = "src/testdata";
    const FOUNDATION: &str = "cncf";
    const FAKE_ERROR: &str = "fake error";

    #[tokio::test]
    async fn error_getting_foundations() {
        let cfg = setup_test_config();

        let mut db = MockDB::new();
        db.expect_foundations()
            .times(1)
            .returning(|| Box::pin(future::ready(Err(format_err!(FAKE_ERROR)))));

        let result = run(&cfg, Arc::new(db)).await;
        assert_eq!(result.unwrap_err().root_cause().to_string(), FAKE_ERROR);
    }

    #[tokio::test]
    async fn no_foundations_found() {
        let cfg = setup_test_config();

        let mut db = MockDB::new();
        db.expect_foundations()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));

        run(&cfg, Arc::new(db)).await.unwrap();
    }

    #[tokio::test]
    async fn error_fetching_foundation_data_file() {
        let cfg = setup_test_config();

        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mut db = MockDB::new();
        db.expect_foundations().times(1).returning(move || {
            Box::pin(future::ready(Ok(vec![Foundation {
                foundation_id: FOUNDATION.to_string(),
                data_url: url.clone(),
            }])))
        });

        let data_file_req = server
            .mock("GET", "/")
            .with_status(404)
            .create_async()
            .await;

        let result = run(&cfg, Arc::new(db)).await;
        assert_eq!(
            result.unwrap_err().root_cause().to_string(),
            "unexpected status code getting data file: 404 Not Found"
        );
        data_file_req.assert_async().await;
    }

    #[tokio::test]
    async fn invalid_foundation_data_file() {
        let cfg = setup_test_config();

        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mut db = MockDB::new();
        db.expect_foundations().times(1).returning(move || {
            Box::pin(future::ready(Ok(vec![Foundation {
                foundation_id: FOUNDATION.to_string(),
                data_url: url.clone(),
            }])))
        });

        let data_file_req = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("{invalid")
            .create_async()
            .await;

        let result = run(&cfg, Arc::new(db)).await;
        assert_eq!(
            result.unwrap_err().root_cause().to_string(),
            "invalid type: map, expected a sequence"
        );
        data_file_req.assert_async().await;
    }

    #[tokio::test]
    async fn error_getting_projects_registered_in_database() {
        let cfg = setup_test_config();

        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mut db = MockDB::new();
        db.expect_foundations().times(1).returning(move || {
            Box::pin(future::ready(Ok(vec![Foundation {
                foundation_id: FOUNDATION.to_string(),
                data_url: url.clone(),
            }])))
        });
        db.expect_foundation_projects()
            .with(eq(FOUNDATION))
            .times(1)
            .returning(|_| Box::pin(future::ready(Err(format_err!(FAKE_ERROR)))));

        let data_file_req = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("")
            .create_async()
            .await;

        let result = run(&cfg, Arc::new(db)).await;
        assert_eq!(result.unwrap_err().root_cause().to_string(), FAKE_ERROR);
        data_file_req.assert_async().await;
    }

    #[tokio::test]
    async fn no_need_to_register_registered_project_same_digest() {
        let cfg = setup_test_config();

        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mut db = MockDB::new();
        db.expect_foundations().times(1).returning(move || {
            Box::pin(future::ready(Ok(vec![Foundation {
                foundation_id: FOUNDATION.to_string(),
                data_url: url.clone(),
            }])))
        });
        db.expect_foundation_projects()
            .with(eq(FOUNDATION))
            .times(1)
            .returning(|_| {
                let mut projects_registered = HashMap::new();
                projects_registered.insert(
                    "artifact-hub".to_string(),
                    Some(
                        "c5ad3114e4e2c11afa4d981180954c63b71e5282890007d0d475d38278082dd1"
                            .to_string(),
                    ),
                );
                Box::pin(future::ready(Ok(projects_registered)))
            });

        let data_file_req = server
            .mock("GET", "/")
            .with_status(200)
            .with_body_from_file(format!("{TESTDATA_PATH}/cncf.yaml"))
            .create_async()
            .await;

        run(&cfg, Arc::new(db)).await.unwrap();
        data_file_req.assert_async().await;
    }

    #[tokio::test]
    async fn register_project_not_registered_yet() {
        let cfg = setup_test_config();

        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mut db = MockDB::new();
        db.expect_foundations().times(1).returning(move || {
            Box::pin(future::ready(Ok(vec![Foundation {
                foundation_id: FOUNDATION.to_string(),
                data_url: url.clone(),
            }])))
        });
        db.expect_foundation_projects()
            .with(eq(FOUNDATION))
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(HashMap::new()))));
        db.expect_register_project()
            .with(
                eq(FOUNDATION),
                eq(Project {
                    name: "artifact-hub".to_string(),
                    display_name: Some("Artifact Hub".to_string()),
                    description: "Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects".to_string(),
                    category: Some("app definition".to_string()),
                    home_url: None,
                    logo_url: Some("https://raw.githubusercontent.com/cncf/artwork/master/projects/artifacthub/icon/color/artifacthub-icon-color.svg".to_string()),
                    logo_dark_url: None,
                    devstats_url: Some("https://artifacthub.devstats.cncf.io/".to_string()),
                    accepted_at: Some("2020-06-23".to_string()),
                    maturity: Some("sandbox".to_string()),
                    digest: Some("c5ad3114e4e2c11afa4d981180954c63b71e5282890007d0d475d38278082dd1".to_string()),
                    repositories: vec![Repository{
                        name: "artifact-hub".to_string(),
                        url: "https://github.com/artifacthub/hub".to_string(),
                        check_sets: Some(vec!["community".to_string(), "code".to_string()]),
                        exclude: None,
                    }]
                }),
            )
            .times(1)
            .returning(|_, _| Box::pin(future::ready(Ok(()))));

        let data_file_req = server
            .mock("GET", "/")
            .with_status(200)
            .with_body_from_file(format!("{TESTDATA_PATH}/cncf.yaml"))
            .create_async()
            .await;

        run(&cfg, Arc::new(db)).await.unwrap();
        data_file_req.assert_async().await;
    }

    #[tokio::test]
    async fn unregister_registered_project() {
        let cfg = setup_test_config();

        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mut db = MockDB::new();
        db.expect_foundations().times(1).returning(move || {
            Box::pin(future::ready(Ok(vec![Foundation {
                foundation_id: FOUNDATION.to_string(),
                data_url: url.clone(),
            }])))
        });
        db.expect_foundation_projects()
            .with(eq(FOUNDATION))
            .times(1)
            .returning(|_| {
                let mut projects_registered = HashMap::new();
                projects_registered.insert(
                    "artifact-hub".to_string(),
                    Some(
                        "c5ad3114e4e2c11afa4d981180954c63b71e5282890007d0d475d38278082dd1"
                            .to_string(),
                    ),
                );
                projects_registered.insert("project-name".to_string(), Some("digest".to_string()));
                Box::pin(future::ready(Ok(projects_registered)))
            });
        db.expect_unregister_project()
            .with(eq(FOUNDATION), eq("project-name"))
            .times(1)
            .returning(|_, _| Box::pin(future::ready(Ok(()))));

        let data_file_req = server
            .mock("GET", "/")
            .with_status(200)
            .with_body_from_file(format!("{TESTDATA_PATH}/cncf.yaml"))
            .create_async()
            .await;

        run(&cfg, Arc::new(db)).await.unwrap();
        data_file_req.assert_async().await;
    }

    fn setup_test_config() -> Config {
        Config::builder()
            .set_default("registrar.concurrency", 1)
            .unwrap()
            .build()
            .unwrap()
    }
}
