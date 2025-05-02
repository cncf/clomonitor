use anyhow::{Context, Result};
use time::{ext::NumericalDuration, Date, OffsetDateTime};
use tracing::{debug, info, instrument};
use uuid::Uuid;

use crate::db::DynDB;

/// Process projects and stats, generating snapshots when needed and removing
/// the ones that are no longer needed.
#[instrument(skip_all, err)]
pub(crate) async fn run(db: DynDB) -> Result<()> {
    info!("started");

    debug!("processing projects");
    for project_id in &db.projects_ids().await? {
        process_project(db.clone(), project_id).await?;
    }

    debug!("processing stats");
    for foundation in &db.foundations().await? {
        process_stats(db.clone(), Some(foundation)).await?;
    }
    process_stats(db.clone(), None).await?; // All foundations

    info!("finished");
    Ok(())
}

/// Process project provided, generating a snapshot for the current day when
/// needed and cleaning up the ones no longer needed.
#[instrument(fields(project_id = %project_id), skip_all, err)]
async fn process_project(db: DynDB, project_id: &Uuid) -> Result<()> {
    // Get project's snapshots
    let snapshots = db
        .project_snapshots(project_id)
        .await
        .context("error getting snapshots")?;
    let latest_snapshot_date = snapshots.first().map(ToOwned::to_owned);

    // Store new project's snapshot if needed
    let today = OffsetDateTime::now_utc().date();
    if latest_snapshot_date.unwrap_or(Date::MIN) < today {
        let data = db
            .project_data(project_id)
            .await
            .context("error getting project data")?;
        if let Some(data) = data {
            db.store_project_snapshot(project_id, data)
                .await
                .context("error storing snapshot")?;
            debug!(date = %today, "snapshot stored");
        }
    }

    // Delete snapshots no longer needed
    let snapshots_to_keep = get_snapshots_to_keep(today, snapshots.as_slice());
    for snapshot in &snapshots {
        if !snapshots_to_keep.contains(snapshot) {
            db.delete_project_snapshot(project_id, snapshot)
                .await
                .context(format!("error deleting snapshot {snapshot}"))?;
            debug!(date = %snapshot, "snapshot deleted");
        }
    }

    Ok(())
}

/// Process stats, generating a snapshot for the current day when needed and
/// cleaning up the ones no longer needed.
#[instrument(fields(foundation = foundation.unwrap_or_default()), skip_all, err)]
async fn process_stats(db: DynDB, foundation: Option<&str>) -> Result<()> {
    // Get stats's snapshots
    let snapshots = db
        .stats_snapshots(foundation)
        .await
        .context("error getting snapshots")?;
    let latest_snapshot_date = snapshots.first().map(ToOwned::to_owned);

    // Store new stats snapshot if needed
    let today = OffsetDateTime::now_utc().date();
    if latest_snapshot_date.unwrap_or(Date::MIN) < today {
        let data = db
            .stats_data(foundation)
            .await
            .context("error getting stats data")?;
        if let Some(data) = data {
            db.store_stats_snapshot(foundation, data)
                .await
                .context("error storing snapshot")?;
            debug!(date = %today, "snapshot stored");
        }
    }

    // Delete snapshots no longer needed
    let snapshots_to_keep = get_snapshots_to_keep(today, snapshots.as_slice());
    for snapshot in &snapshots {
        if !snapshots_to_keep.contains(snapshot) {
            db.delete_stats_snapshot(foundation, snapshot)
                .await
                .context(format!("error deleting snapshot {snapshot}"))?;
            debug!(date = %snapshot, "snapshot deleted");
        }
    }

    Ok(())
}

/// Return a list of snapshots that we'd like to keep.
fn get_snapshots_to_keep(ref_date: Date, snapshots: &[Date]) -> Vec<Date> {
    let mut snapshots_to_keep = Vec::new();

    for snapshot in snapshots.iter().copied() {
        // Include snapshots for the previous 2 days
        if ref_date - snapshot <= 2.days() {
            snapshots_to_keep.push(snapshot);
            continue;
        }

        // Include latest snapshot for each week in the last month
        if ref_date - snapshot <= 30.days()
            && snapshots_to_keep.last().is_none_or(|last_snapshot_kept| {
                last_snapshot_kept.iso_week() > snapshot.iso_week()
            })
        {
            snapshots_to_keep.push(snapshot);
            continue;
        }

        // Include latest snapshot for each month in the last 2 years
        if ref_date - snapshot <= (2 * 365).days()
            && snapshots_to_keep.last().is_none_or(|last_snapshot_kept| {
                last_snapshot_kept.month() as u8 > snapshot.month() as u8
            })
        {
            snapshots_to_keep.push(snapshot);
            continue;
        }

        // Include latest snapshot of the year for the rest of the years
        if snapshots_to_keep
            .last()
            .is_none_or(|last_snapshot_kept| last_snapshot_kept.year() > snapshot.year())
        {
            snapshots_to_keep.push(snapshot);
        }
    }

    snapshots_to_keep
}

#[cfg(test)]
#[allow(clippy::redundant_closure_for_method_calls)]
mod tests {
    use std::sync::{Arc, LazyLock};

    use anyhow::format_err;
    use futures::future;
    use mockall::predicate::eq;
    use serde_json::{json, Value};
    use time::{macros::date, Month};

    use crate::db::MockDB;

    use super::*;

    const FAKE_ERROR: &str = "fake error";

    static PROJECT_ID: LazyLock<Uuid> =
        LazyLock::new(|| Uuid::parse_str("00000000-0001-0000-0000-000000000000").unwrap());
    static SNAPSHOT_DATA: LazyLock<Value> = LazyLock::new(|| json!({"some": "data"}));
    static SNAPSHOT_1980_12: LazyLock<Date> =
        LazyLock::new(|| Date::from_calendar_date(1980, Month::December, 1).unwrap());
    static SNAPSHOT_1980_11: LazyLock<Date> =
        LazyLock::new(|| Date::from_calendar_date(1980, Month::November, 1).unwrap());

    #[tokio::test]
    async fn error_getting_projects() {
        let mut db = MockDB::new();
        db.expect_projects_ids()
            .times(1)
            .returning(|| Box::pin(future::ready(Err(format_err!(FAKE_ERROR)))));

        let result = run(Arc::new(db)).await;
        assert_eq!(result.unwrap_err().root_cause().to_string(), FAKE_ERROR);
    }

    #[tokio::test]
    async fn error_getting_foundations() {
        let mut db = MockDB::new();
        db.expect_projects_ids()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));
        db.expect_foundations()
            .times(1)
            .returning(|| Box::pin(future::ready(Err(format_err!(FAKE_ERROR)))));

        let result = run(Arc::new(db)).await;
        assert_eq!(result.unwrap_err().root_cause().to_string(), FAKE_ERROR);
    }

    #[tokio::test]
    async fn project_error_getting_snapshots() {
        let mut db = MockDB::new();
        db.expect_projects_ids()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![*PROJECT_ID]))));
        db.expect_project_snapshots()
            .with(eq(*PROJECT_ID))
            .times(1)
            .returning(|_| Box::pin(future::ready(Err(format_err!(FAKE_ERROR)))));

        let result = run(Arc::new(db)).await;
        assert_eq!(result.unwrap_err().root_cause().to_string(), FAKE_ERROR);
    }

    #[tokio::test]
    async fn project_store_new_snapshot() {
        let mut db = MockDB::new();
        db.expect_projects_ids()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![*PROJECT_ID]))));
        db.expect_project_snapshots()
            .with(eq(*PROJECT_ID))
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(vec![]))));
        db.expect_project_data()
            .with(eq(*PROJECT_ID))
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(Some(SNAPSHOT_DATA.clone())))));
        db.expect_store_project_snapshot()
            .with(eq(*PROJECT_ID), eq(SNAPSHOT_DATA.clone()))
            .times(1)
            .returning(|_, _| Box::pin(future::ready(Ok(()))));
        db.expect_foundations()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));
        db.expect_stats_snapshots()
            .withf(|foundation| foundation.is_none())
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(vec![OffsetDateTime::now_utc().date()]))));

        run(Arc::new(db)).await.unwrap();
    }

    #[tokio::test]
    async fn project_no_need_to_store_new_snapshot() {
        let mut db = MockDB::new();
        db.expect_projects_ids()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![*PROJECT_ID]))));
        db.expect_project_snapshots()
            .with(eq(*PROJECT_ID))
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(vec![OffsetDateTime::now_utc().date()]))));
        db.expect_foundations()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));
        db.expect_stats_snapshots()
            .withf(|foundation| foundation.is_none())
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(vec![OffsetDateTime::now_utc().date()]))));

        run(Arc::new(db)).await.unwrap();
    }

    #[tokio::test]
    async fn project_delete_old_snapshot() {
        let mut db = MockDB::new();
        db.expect_projects_ids()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![*PROJECT_ID]))));
        db.expect_project_snapshots()
            .with(eq(*PROJECT_ID))
            .times(1)
            .returning(|_| {
                Box::pin(future::ready(Ok(vec![
                    OffsetDateTime::now_utc().date(),
                    *SNAPSHOT_1980_12,
                    *SNAPSHOT_1980_11,
                ])))
            });
        db.expect_delete_project_snapshot()
            .with(eq(*PROJECT_ID), eq(*SNAPSHOT_1980_11))
            .times(1)
            .returning(|_, _| Box::pin(future::ready(Ok(()))));
        db.expect_foundations()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));
        db.expect_stats_snapshots()
            .withf(|foundation| foundation.is_none())
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(vec![OffsetDateTime::now_utc().date()]))));

        run(Arc::new(db)).await.unwrap();
    }

    #[tokio::test]
    async fn stats_all_foundations_error_getting_snapshots() {
        let mut db = MockDB::new();
        db.expect_projects_ids()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));
        db.expect_foundations()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));
        db.expect_stats_snapshots()
            .withf(|foundation| foundation.is_none())
            .times(1)
            .returning(|_| Box::pin(future::ready(Err(format_err!(FAKE_ERROR)))));

        let result = run(Arc::new(db)).await;
        assert_eq!(result.unwrap_err().root_cause().to_string(), FAKE_ERROR);
    }

    #[tokio::test]
    async fn stats_all_foundations_store_new_snapshot() {
        let mut db = MockDB::new();
        db.expect_projects_ids()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));
        db.expect_foundations()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));
        db.expect_stats_snapshots()
            .withf(|foundation| foundation.is_none())
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(vec![]))));
        db.expect_stats_data()
            .withf(|foundation| foundation.is_none())
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(Some(SNAPSHOT_DATA.clone())))));
        db.expect_store_stats_snapshot()
            .withf(|foundation, data| foundation.is_none() && data == &SNAPSHOT_DATA.clone())
            .times(1)
            .returning(|_, _| Box::pin(future::ready(Ok(()))));

        run(Arc::new(db)).await.unwrap();
    }

    #[tokio::test]
    async fn stats_single_foundation_store_new_snapshot() {
        let mut db = MockDB::new();
        db.expect_projects_ids()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));
        db.expect_foundations()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec!["cncf".to_string()]))));
        db.expect_stats_snapshots()
            .withf(|foundation| foundation == &Some("cncf"))
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(vec![]))));
        db.expect_stats_data()
            .withf(|foundation| foundation == &Some("cncf"))
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(Some(SNAPSHOT_DATA.clone())))));
        db.expect_store_stats_snapshot()
            .withf(|foundation, data| foundation == &Some("cncf") && data == &SNAPSHOT_DATA.clone())
            .times(1)
            .returning(|_, _| Box::pin(future::ready(Ok(()))));
        db.expect_stats_snapshots()
            .withf(|foundation| foundation.is_none())
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(vec![OffsetDateTime::now_utc().date()]))));

        run(Arc::new(db)).await.unwrap();
    }

    #[tokio::test]
    async fn stats_all_foundations_no_need_to_store_new_snapshot() {
        let mut db = MockDB::new();
        db.expect_projects_ids()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));
        db.expect_foundations()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));
        db.expect_stats_snapshots()
            .withf(|foundation| foundation.is_none())
            .times(1)
            .returning(|_| Box::pin(future::ready(Ok(vec![OffsetDateTime::now_utc().date()]))));

        run(Arc::new(db)).await.unwrap();
    }

    #[tokio::test]
    async fn stats_all_foundations_delete_old_snapshot() {
        let mut db = MockDB::new();
        db.expect_projects_ids()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));
        db.expect_foundations()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));
        db.expect_stats_snapshots()
            .withf(|foundation| foundation.is_none())
            .times(1)
            .returning(|_| {
                Box::pin(future::ready(Ok(vec![
                    OffsetDateTime::now_utc().date(),
                    *SNAPSHOT_1980_12,
                    *SNAPSHOT_1980_11,
                ])))
            });
        db.expect_delete_stats_snapshot()
            .withf(|foundation, date| foundation.is_none() && date == &*SNAPSHOT_1980_11)
            .times(1)
            .returning(|_, _| Box::pin(future::ready(Ok(()))));

        run(Arc::new(db)).await.unwrap();
    }

    #[test]
    fn get_snapshots_to_keep_1() {
        assert_eq!(
            get_snapshots_to_keep(
                date!(2022 - 10 - 28),
                &[
                    date!(2022 - 10 - 28),
                    date!(2022 - 10 - 27),
                    date!(2022 - 10 - 26),
                ]
            ),
            vec![
                date!(2022 - 10 - 28),
                date!(2022 - 10 - 27),
                date!(2022 - 10 - 26),
            ]
        );
    }

    #[test]
    fn get_snapshots_to_keep_2() {
        assert_eq!(
            get_snapshots_to_keep(
                date!(2022 - 10 - 28),
                &[
                    date!(2022 - 10 - 28),
                    date!(2022 - 10 - 25),
                    date!(2022 - 10 - 24),
                ]
            ),
            vec![date!(2022 - 10 - 28)]
        );
    }

    #[test]
    fn get_snapshots_to_keep_3() {
        assert_eq!(
            get_snapshots_to_keep(
                date!(2022 - 10 - 24),
                &[
                    date!(2022 - 10 - 24),
                    date!(2022 - 10 - 20),
                    date!(2022 - 10 - 19),
                ]
            ),
            vec![date!(2022 - 10 - 24), date!(2022 - 10 - 20),]
        );
    }

    #[test]
    fn get_snapshots_to_keep_4() {
        assert_eq!(
            get_snapshots_to_keep(
                date!(2022 - 10 - 25),
                &[
                    date!(2022 - 10 - 25),
                    date!(2022 - 10 - 24),
                    date!(2022 - 10 - 20),
                    date!(2022 - 10 - 19),
                    date!(2022 - 10 - 13),
                    date!(2022 - 10 - 10),
                ]
            ),
            vec![
                date!(2022 - 10 - 25),
                date!(2022 - 10 - 24),
                date!(2022 - 10 - 20),
                date!(2022 - 10 - 13),
            ]
        );
    }

    #[test]
    fn get_snapshots_to_keep_5() {
        assert_eq!(
            get_snapshots_to_keep(
                date!(2022 - 10 - 25),
                &[
                    date!(2022 - 10 - 25),
                    date!(2022 - 10 - 13),
                    date!(2022 - 10 - 10),
                ]
            ),
            vec![date!(2022 - 10 - 25), date!(2022 - 10 - 13),]
        );
    }

    #[test]
    fn get_snapshots_to_keep_6() {
        assert_eq!(
            get_snapshots_to_keep(
                date!(2022 - 10 - 25),
                &[
                    date!(2022 - 10 - 18),
                    date!(2022 - 10 - 17),
                    date!(2022 - 9 - 29),
                    date!(2022 - 9 - 11),
                    date!(2022 - 8 - 1),
                ]
            ),
            vec![
                date!(2022 - 10 - 18),
                date!(2022 - 9 - 29),
                date!(2022 - 8 - 1),
            ]
        );
    }

    #[test]
    fn get_snapshots_to_keep_7() {
        assert_eq!(
            get_snapshots_to_keep(
                date!(2022 - 10 - 25),
                &[
                    date!(2022 - 10 - 18),
                    date!(2021 - 10 - 17),
                    date!(2021 - 9 - 29),
                    date!(2020 - 9 - 11),
                    date!(2020 - 8 - 1),
                ]
            ),
            vec![
                date!(2022 - 10 - 18),
                date!(2021 - 10 - 17),
                date!(2021 - 9 - 29),
                date!(2020 - 9 - 11),
            ]
        );
    }

    #[test]
    fn get_snapshots_to_keep_8() {
        assert_eq!(
            get_snapshots_to_keep(
                date!(2022 - 10 - 25),
                &[
                    date!(2022 - 10 - 25),
                    date!(2022 - 10 - 24),
                    date!(2022 - 10 - 20),
                    date!(2022 - 10 - 19),
                    date!(2022 - 10 - 13),
                    date!(2022 - 10 - 10),
                    date!(2022 - 7 - 1),
                    date!(2022 - 6 - 2),
                    date!(2022 - 6 - 1),
                    date!(2021 - 12 - 9),
                    date!(2021 - 9 - 29),
                    date!(2020 - 9 - 11),
                    date!(2020 - 8 - 23),
                ]
            ),
            vec![
                date!(2022 - 10 - 25),
                date!(2022 - 10 - 24),
                date!(2022 - 10 - 20),
                date!(2022 - 10 - 13),
                date!(2022 - 7 - 1),
                date!(2022 - 6 - 2),
                date!(2021 - 12 - 9),
                date!(2021 - 9 - 29),
                date!(2020 - 9 - 11),
            ]
        );
    }
}
