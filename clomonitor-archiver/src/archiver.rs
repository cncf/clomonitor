use crate::db::DynDB;
use anyhow::{Context, Result};
use time::{ext::NumericalDuration, Date, OffsetDateTime};
use tracing::{debug, info, instrument};
use uuid::Uuid;

/// Process projects and stats, generating snapshots when needed and removing
/// the ones that are no longer needed.
#[instrument(skip_all, err)]
pub(crate) async fn run(db: DynDB) -> Result<()> {
    info!("started");

    debug!("processing projects");
    for project_id in db.projects_ids().await?.iter() {
        process_project(db.clone(), project_id).await?;
    }

    debug!("processing stats");
    for foundation in db.foundations().await?.iter() {
        process_stats(db.clone(), Some(foundation)).await?;
    }
    process_stats(db.clone(), None).await?; // All foundations

    info!("finished");
    Ok(())
}

/// Process project provided, generating a snapshot for the current day when
/// needed and cleaning up the ones no longer needed.
#[instrument(fields(project_id = project_id.to_string()), skip_all, err)]
async fn process_project(db: DynDB, project_id: &Uuid) -> Result<()> {
    // Get project's snapshots
    let snapshots = db
        .project_snapshots(project_id)
        .await
        .context("error getting snapshots")?;
    let latest_snapshot_date = snapshots.first().map(|d| d.to_owned());

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
            debug!("snapshot [{}] stored", today);
        }
    }

    // Delete snapshots no longer needed
    let snapshots_to_keep = get_snapshots_to_keep(today, snapshots.as_slice());
    for snapshot in snapshots.iter() {
        if !snapshots_to_keep.contains(snapshot) {
            db.delete_project_snapshot(project_id, snapshot)
                .await
                .context(format!("error deleting snapshot {snapshot}"))?;
            debug!("snapshot [{}] deleted", snapshot);
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
    let latest_snapshot_date = snapshots.first().map(|d| d.to_owned());

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
            debug!("snapshot [{}] stored", today);
        }
    }

    // Delete snapshots no longer needed
    let snapshots_to_keep = get_snapshots_to_keep(today, snapshots.as_slice());
    for snapshot in snapshots.iter() {
        if !snapshots_to_keep.contains(snapshot) {
            db.delete_stats_snapshot(foundation, snapshot)
                .await
                .context(format!("error deleting snapshot {snapshot}"))?;
            debug!("snapshot [{}] deleted", snapshot);
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
            && snapshots_to_keep.last().map_or(true, |last_snapshot_kept| {
                last_snapshot_kept.iso_week() > snapshot.iso_week()
            })
        {
            snapshots_to_keep.push(snapshot);
            continue;
        }

        // Include latest snapshot for each month in the last 2 years
        if ref_date - snapshot <= (2 * 365).days()
            && snapshots_to_keep.last().map_or(true, |last_snapshot_kept| {
                last_snapshot_kept.month() as u8 > snapshot.month() as u8
            })
        {
            snapshots_to_keep.push(snapshot);
            continue;
        }

        // Include latest snapshot of the year for the rest of the years
        if snapshots_to_keep.last().map_or(true, |last_snapshot_kept| {
            last_snapshot_kept.year() > snapshot.year()
        }) {
            snapshots_to_keep.push(snapshot);
        }
    }

    snapshots_to_keep
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

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
