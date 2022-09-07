use crate::db::DynDB;
use anyhow::{format_err, Result};
use config::Config;
use futures::{
    future,
    stream::{FuturesUnordered, StreamExt},
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, time::Duration};
use tokio::time::{timeout, Instant};
use tracing::{debug, error, info, instrument, warn};

/// Maximum time that can take processing a foundation data file.
const FOUNDATION_TIMEOUT: u64 = 300;

/// Represents a foundation registered in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Foundation {
    pub foundation_id: String,
    pub data_url: String,
}

/// Represents a project to be registered or updated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Project {
    pub name: String,
    pub display_name: Option<String>,
    pub description: String,
    pub category: String,
    pub home_url: Option<String>,
    pub logo_url: Option<String>,
    pub devstats_url: Option<String>,
    pub accepted_at: Option<String>,
    pub maturity: String,
    pub digest: Option<String>,
    pub repositories: Vec<Repository>,
}

impl Project {
    fn set_digest(&mut self) -> Result<()> {
        let data = bincode::serialize(&self)?;
        let digest = hex::encode(Sha256::digest(data));
        self.digest = Some(digest);
        Ok(())
    }
}

/// Represents a project's repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Repository {
    pub name: String,
    pub url: String,
    pub check_sets: Vec<String>,
}

/// Process foundations registered in the database.
#[instrument(skip_all, err)]
pub(crate) async fn run(cfg: &Config, db: DynDB) -> Result<()> {
    info!("started");

    // Process foundations
    let http_client = reqwest::Client::new();
    let mut futs = FuturesUnordered::new();
    for foundation in db.foundations().await? {
        let foundation_id = foundation.foundation_id.clone();

        // Spawn task to process next foundation
        let db = db.clone();
        let http_client = http_client.clone();
        futs.push(tokio::spawn(async move {
            if timeout(
                Duration::from_secs(FOUNDATION_TIMEOUT),
                process_foundation(db, http_client, foundation),
            )
            .await
            .is_err()
            {
                warn!("timeout processing foundation data file {}", foundation_id);
            }
        }));

        // Wait if needed to honor the concurrency limits
        if futs.len() == cfg.get::<usize>("registrar.concurrency")? {
            futs.next().await;
        }
    }
    future::join_all(futs).await;

    info!("finished");
    Ok(())
}

/// Process foundation's data file. New projects available will be registered
/// in the database and existing ones which have changed will be updated. When
/// a project is removed from the data file, it'll be removed from the database
/// as well.
#[instrument(fields(foundation_id = foundation.foundation_id), skip_all, err)]
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
        project.set_digest()?;
        projects_available.insert(project.name.clone(), project);
    }

    // Get projects registered in the database
    let foundation_id = &foundation.foundation_id;
    let projects_registered = db.foundation_projects(foundation_id).await?;

    // Unregister projects no longer available in the data file
    if !projects_available.is_empty() {
        for name in projects_registered.keys() {
            if !projects_available.contains_key(name) {
                debug!("unregistering project {}", name);
                if let Err(err) = db.unregister_project(foundation_id, name).await {
                    error!("error unregistering project {}: {}", name, err);
                };
            }
        }
    }

    // Register or update available projects as needed
    for (name, project) in &projects_available {
        // Check if the project is already registered
        if let Some(registered_digest) = projects_registered.get(name) {
            if registered_digest == &project.digest {
                continue;
            }
        }

        // Register project
        debug!("registering project {}", project.name);
        if let Err(err) = db.register_project(foundation_id, project).await {
            error!("error registering project {}: {}", project.name, err);
        }
    }

    debug!("completed in {}s", start.elapsed().as_secs());
    Ok(())
}
