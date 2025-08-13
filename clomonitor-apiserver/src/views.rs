use std::{collections::HashMap, sync::Arc, sync::LazyLock, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use time::{
    OffsetDateTime,
    format_description::{self, FormatItem},
};
use tokio::{
    sync::{RwLock, broadcast, mpsc},
    task::JoinSet,
    time::Instant,
};
use tracing::error;
use uuid::Uuid;

use crate::db::DynDB;

/// How often projects views will be written to the database.
#[cfg(not(test))]
const FLUSH_FREQUENCY: Duration = Duration::from_secs(300);
#[cfg(test)]
const FLUSH_FREQUENCY: Duration = Duration::from_millis(100);

/// Type alias to represent a ViewsTracker trait object.
pub(crate) type DynVT = Arc<RwLock<dyn ViewsTracker + Send + Sync>>;

/// Type alias to represent a project id.
pub(crate) type ProjectId = Uuid;

/// Type alias to represent a day in DATE_FORMAT.
pub(crate) type Day = String;

/// Type alias to represent a views counter.
pub(crate) type Total = u32;

/// Type alias to represent a views batch.
type Batch = HashMap<String, Total>;

static DATE_FORMAT: LazyLock<Vec<FormatItem<'static>>> = LazyLock::new(|| {
    format_description::parse("[year]-[month]-[day]").expect("format to be valid")
});

/// Trait that defines some operations a ViewsTracker implementation must
/// support.
#[async_trait]
#[cfg_attr(test, automock)]
pub(crate) trait ViewsTracker {
    /// Track a view for the project provided.
    async fn track_view(&self, project_id: ProjectId) -> Result<()>;
}

/// ViewsTracker implementation backed by a DB instance.
pub(crate) struct ViewsTrackerDB {
    views_tx: mpsc::Sender<ProjectId>,
    stop_tx: Option<broadcast::Sender<()>>,
    workers: JoinSet<()>,
}

impl ViewsTrackerDB {
    /// Create a new ViewsTrackerDB instance.
    pub(crate) fn new(db: DynDB) -> Self {
        // Setup channels
        let (views_tx, views_rx) = mpsc::channel(100);
        let (batches_tx, batches_rx) = mpsc::channel(5);
        let (stop_tx, _) = broadcast::channel(1);

        // Setup workers
        let mut workers = JoinSet::new();
        workers.spawn(aggregator(views_rx, batches_tx, stop_tx.subscribe()));
        workers.spawn(flusher(db, batches_rx));

        Self {
            views_tx,
            stop_tx: Some(stop_tx),
            workers,
        }
    }

    /// Ask the workers to stop and wait for them to finish.
    pub(crate) async fn stop(&mut self) {
        self.stop_tx = None;
        while self.workers.join_next().await.is_some() {}
    }
}

#[async_trait]
impl ViewsTracker for ViewsTrackerDB {
    async fn track_view(&self, project_id: ProjectId) -> Result<()> {
        self.views_tx.send(project_id).await.map_err(Into::into)
    }
}

/// Worker that aggregates the views received on the views channel, passing the
/// resulting batches to the flusher periodically.
async fn aggregator(
    mut views_rx: mpsc::Receiver<ProjectId>,
    batches_tx: mpsc::Sender<Batch>,
    mut stop_rx: broadcast::Receiver<()>,
) {
    let first_flush = Instant::now() + FLUSH_FREQUENCY;
    let mut flush_interval = tokio::time::interval_at(first_flush, FLUSH_FREQUENCY);
    let mut batch: Batch = HashMap::new();
    loop {
        tokio::select! {
            biased;

            // Send batch to flusher every FLUSH_FREQUENCY
            _ = flush_interval.tick() => {
                if !batch.is_empty() {
                    _ = batches_tx.send(batch.clone()).await;
                    batch.clear();
                }
            }

            // Pick next view from queue and aggregate it
            Some(project_id) = views_rx.recv() => {
                *batch.entry(build_key(project_id)).or_default() += 1;
            }

            // Exit if the aggregator has been asked to stop
            _ = stop_rx.recv() => {
                if !batch.is_empty() {
                    _ = batches_tx.send(batch).await;
                }
                break
            }
        }
    }
}

/// Worker that stores the views batches received from the aggregator into
/// the database.
async fn flusher(db: DynDB, mut batches_rx: mpsc::Receiver<Batch>) {
    while let Some(batch) = batches_rx.recv().await {
        // Prepare batch data for database update
        let mut data: Vec<(ProjectId, Day, Total)> = batch
            .iter()
            .map(|(key, total)| {
                let (project_id, day) = parse_key(key);
                (project_id, day, *total)
            })
            .collect();
        data.sort();

        // Write data to database
        if let Err(err) = db.update_projects_views(data).await {
            error!(?err, "error writing projects views to database");
        }
    }
}

/// Build key used to track views for a given project.
fn build_key(project_id: ProjectId) -> String {
    let day = OffsetDateTime::now_utc().format(&DATE_FORMAT).unwrap();
    format!("{project_id}##{day}")
}

/// Parse project views key, returning the project id and the day.
fn parse_key(key: &str) -> (ProjectId, Day) {
    let mut parts = key.split("##");
    let project_id = Uuid::parse_str(parts.next().unwrap()).unwrap();
    let day = parts.next().unwrap().to_owned();
    (project_id, day)
}

#[cfg(test)]
mod tests {
    use futures::future;
    use mockall::predicate::eq;
    use tokio::time::{Duration, sleep};

    use crate::db::MockDB;

    use super::*;

    static PROJECT1_ID: LazyLock<Uuid> =
        LazyLock::new(|| Uuid::parse_str("00000000-0001-0000-0000-000000000000").unwrap());
    static PROJECT2_ID: LazyLock<Uuid> =
        LazyLock::new(|| Uuid::parse_str("00000000-0002-0000-0000-000000000000").unwrap());

    #[tokio::test]
    async fn no_views_tracked_nothing_to_flush() {
        let mut vt = ViewsTrackerDB::new(Arc::new(MockDB::new()));
        vt.stop().await;
    }

    #[tokio::test]
    async fn flush_periodically() {
        let vt = ViewsTrackerDB::new(setup_mock_db());
        vt.track_view(*PROJECT1_ID).await.unwrap();
        vt.track_view(*PROJECT1_ID).await.unwrap();
        vt.track_view(*PROJECT2_ID).await.unwrap();
        sleep(Duration::from_millis(500)).await;
    }

    #[tokio::test]
    async fn flush_on_stop() {
        let mut vt = ViewsTrackerDB::new(setup_mock_db());
        vt.track_view(*PROJECT1_ID).await.unwrap();
        vt.track_view(*PROJECT1_ID).await.unwrap();
        vt.track_view(*PROJECT2_ID).await.unwrap();
        vt.stop().await;
    }

    fn setup_mock_db() -> Arc<MockDB> {
        let day = OffsetDateTime::now_utc().format(&DATE_FORMAT).unwrap();

        let mut db = MockDB::new();
        db.expect_update_projects_views()
            .with(eq(vec![
                (*PROJECT1_ID, day.clone(), 2),
                (*PROJECT2_ID, day, 1),
            ]))
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(()))));

        Arc::new(db)
    }
}
