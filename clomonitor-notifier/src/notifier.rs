use crate::{db::DynDB, github::DynGH, tmpl};
use anyhow::{format_err, Result};
use askama::Template;
use config::Config;
use lazy_static::lazy_static;
use regex::Regex;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, instrument};
use uuid::Uuid;

/// Process pending notifications.
#[instrument(skip_all, err)]
pub(crate) async fn run(cfg: &Config, db: DynDB, gh: DynGH) -> Result<()> {
    info!("started");

    process_annual_review_notifications(cfg, db, gh).await?;

    info!("finished");
    Ok(())
}

/// Title used in the issues created to notify that the annual review is due.
const ANNUAL_REVIEW_DUE_TITLE: &str = "CNCF TOC annual review due";

/// Information needed to send an annual review notification.
pub(crate) struct AnnualReviewNotification {
    pub project_id: Uuid,
    pub community_repo_url: String,
    pub issue_number: Option<i64>,
}

/// Process annual review notifications.
#[instrument(skip_all, err)]
async fn process_annual_review_notifications(cfg: &Config, db: DynDB, gh: DynGH) -> Result<()> {
    match db.get_pending_annual_review_notifications().await {
        Ok(mut notifications) => {
            // If a list of allowed repositories is provided, filter out
            // notifications whose repository isn't listed on it
            if let Ok(allowed_repos) = cfg.get::<Vec<String>>("notifier.allowedRepositories") {
                notifications.retain(|n| allowed_repos.contains(&n.community_repo_url))
            };

            // Process pending notifications
            for (i, n) in notifications.iter().enumerate() {
                info!(?n.project_id, ?n.community_repo_url, "processing pending annual review notification");

                // Extract owner and repo from url
                let Ok((owner, repo)) = get_owner_and_repo(&n.community_repo_url) else { continue };

                // Pre-register notification in database
                // (to avoid sending multiple notifications if registration failed after sending)
                let notification_id = db
                    .pre_register_annual_review_notification(&n.project_id, &n.community_repo_url)
                    .await?;

                // If the notification contains an issue number, check if it's closed
                let mut is_issue_closed = false;
                if let Some(issue_number) = n.issue_number {
                    is_issue_closed = gh.is_issue_closed(&owner, &repo, issue_number).await?;
                }

                // Send notification
                let issue_number;
                let mut comment_id = None;
                if n.issue_number.is_none() || is_issue_closed {
                    // Create new issue
                    let body = tmpl::AnnualReviewDue {}.render().unwrap();
                    match gh
                        .create_issue(&owner, &repo, ANNUAL_REVIEW_DUE_TITLE, &body)
                        .await
                    {
                        Ok(v) => {
                            issue_number = Some(v);
                            info!(
                                ?owner,
                                ?repo,
                                ?issue_number,
                                "annual review due notification sent"
                            );
                        }
                        Err(err) => {
                            error!(?err, ?owner, ?repo, "error creating issue");
                            continue;
                        }
                    }
                } else {
                    // Post comment in existing issue
                    issue_number = Some(n.issue_number.unwrap());
                    let body = tmpl::AnnualReviewDueReminder {}.render().unwrap();
                    match gh
                        .create_comment(&owner, &repo, issue_number.unwrap(), &body)
                        .await
                    {
                        Ok(v) => {
                            comment_id = Some(v);
                            info!(
                                ?owner,
                                ?repo,
                                ?issue_number,
                                ?comment_id,
                                "annual review due reminder notification sent"
                            );
                        }
                        Err(err) => {
                            error!(?err, ?owner, ?repo, ?issue_number, "error creating comment");
                            continue;
                        }
                    }
                }

                // Update notification details in database
                db.update_annual_review_notification(&notification_id, issue_number, comment_id)
                    .await?;

                // If there are more notifications to process, pause before the
                // next one to avoid hitting GitHub secondary rate limits
                // https://docs.github.com/en/rest/guides/best-practices-for-integrators?apiVersion=2022-11-28#dealing-with-secondary-rate-limits
                if i < notifications.len() - 1 {
                    sleep(Duration::from_secs(10)).await;
                }
            }

            Ok(())
        }
        Err(err) => Err(err),
    }
}

lazy_static! {
    static ref GITHUB_REPO_URL: Regex =
        Regex::new("^https://github.com/(?P<owner>[^/]+)/(?P<repo>[^/]+)/?$")
            .expect("exprs in GITHUB_REPO_URL to be valid");
}

/// Extract the owner and repository from the repository url provided.
fn get_owner_and_repo(repo_url: &str) -> Result<(String, String)> {
    let c = GITHUB_REPO_URL
        .captures(repo_url)
        .ok_or_else(|| format_err!("invalid repository url"))?;
    Ok((c["owner"].to_string(), c["repo"].to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::MockDB, github::MockGH};
    use futures::future;
    use mockall::predicate::eq;

    const FAKE_ERROR: &str = "fake error";
    const REPO1_URL: &str = "https://github.com/owner/repo1";
    const REPO2_URL: &str = "https://github.com/owner/repo2";
    const ISSUE_NUMBER: i64 = 1;
    const COMMENT_ID: i64 = 1234;

    lazy_static! {
        static ref PROJECT_ID: Uuid =
            Uuid::parse_str("00000000-0001-0000-0000-000000000000").unwrap();
        static ref NOTIFICATION_ID: Uuid =
            Uuid::parse_str("00000000-0001-0000-0000-000000000000").unwrap();
    }

    #[tokio::test]
    async fn error_getting_pending_annual_review_notifications() {
        let cfg = Config::builder().build().unwrap();

        let mut db = MockDB::new();
        db.expect_get_pending_annual_review_notifications()
            .times(1)
            .returning(|| Box::pin(future::ready(Err(format_err!(FAKE_ERROR)))));

        let gh = MockGH::new();

        let result = run(&cfg, Box::new(db), Box::new(gh)).await;
        assert_eq!(result.unwrap_err().root_cause().to_string(), FAKE_ERROR);
    }

    #[tokio::test]
    async fn no_pending_annual_review_notifications() {
        let cfg = Config::builder().build().unwrap();

        let mut db = MockDB::new();
        db.expect_get_pending_annual_review_notifications()
            .times(1)
            .returning(|| Box::pin(future::ready(Ok(vec![]))));

        let gh = MockGH::new();

        run(&cfg, Box::new(db), Box::new(gh)).await.unwrap();
    }

    #[tokio::test]
    async fn filter_out_repo_not_in_allowed_repositories() {
        let cfg = Config::builder()
            .set_default("notifier.allowedRepositories", vec![REPO2_URL])
            .unwrap()
            .build()
            .unwrap();

        let mut db = MockDB::new();
        db.expect_get_pending_annual_review_notifications()
            .times(1)
            .returning(|| {
                Box::pin(future::ready(Ok(vec![AnnualReviewNotification {
                    project_id: *PROJECT_ID,
                    community_repo_url: REPO1_URL.to_string(),
                    issue_number: None,
                }])))
            });

        let gh = MockGH::new();

        run(&cfg, Box::new(db), Box::new(gh)).await.unwrap();
    }

    #[tokio::test]
    async fn create_new_issue_because_none_was_provided() {
        let cfg = Config::builder().build().unwrap();

        let mut db = MockDB::new();
        db.expect_get_pending_annual_review_notifications()
            .times(1)
            .returning(|| {
                Box::pin(future::ready(Ok(vec![AnnualReviewNotification {
                    project_id: *PROJECT_ID,
                    community_repo_url: REPO1_URL.to_string(),
                    issue_number: None,
                }])))
            });
        db.expect_pre_register_annual_review_notification()
            .with(eq(*PROJECT_ID), eq(REPO1_URL))
            .times(1)
            .returning(|_, _| Box::pin(future::ready(Ok(*NOTIFICATION_ID))));
        db.expect_update_annual_review_notification()
            .with(eq(*NOTIFICATION_ID), eq(Some(ISSUE_NUMBER)), eq(None))
            .times(1)
            .returning(|_, _, _| Box::pin(future::ready(Ok(()))));

        let mut gh = MockGH::new();
        gh.expect_create_issue()
            .with(
                eq("owner"),
                eq("repo1"),
                eq(ANNUAL_REVIEW_DUE_TITLE),
                eq(tmpl::AnnualReviewDue {}.render().unwrap()),
            )
            .times(1)
            .returning(|_, _, _, _| Box::pin(future::ready(Ok(ISSUE_NUMBER))));

        run(&cfg, Box::new(db), Box::new(gh)).await.unwrap();
    }

    #[tokio::test]
    async fn create_new_issue_because_existing_one_was_closed() {
        let cfg = Config::builder().build().unwrap();

        let mut db = MockDB::new();
        db.expect_get_pending_annual_review_notifications()
            .times(1)
            .returning(|| {
                Box::pin(future::ready(Ok(vec![AnnualReviewNotification {
                    project_id: *PROJECT_ID,
                    community_repo_url: REPO1_URL.to_string(),
                    issue_number: Some(ISSUE_NUMBER),
                }])))
            });
        db.expect_pre_register_annual_review_notification()
            .with(eq(*PROJECT_ID), eq(REPO1_URL))
            .times(1)
            .returning(|_, _| Box::pin(future::ready(Ok(*NOTIFICATION_ID))));
        db.expect_update_annual_review_notification()
            .with(eq(*NOTIFICATION_ID), eq(Some(ISSUE_NUMBER)), eq(None))
            .times(1)
            .returning(|_, _, _| Box::pin(future::ready(Ok(()))));

        let mut gh = MockGH::new();
        gh.expect_is_issue_closed()
            .with(eq("owner"), eq("repo1"), eq(ISSUE_NUMBER))
            .times(1)
            .returning(|_, _, _| Box::pin(future::ready(Ok(true))));
        gh.expect_create_issue()
            .with(
                eq("owner"),
                eq("repo1"),
                eq(ANNUAL_REVIEW_DUE_TITLE),
                eq(tmpl::AnnualReviewDue {}.render().unwrap()),
            )
            .times(1)
            .returning(|_, _, _, _| Box::pin(future::ready(Ok(ISSUE_NUMBER))));

        run(&cfg, Box::new(db), Box::new(gh)).await.unwrap();
    }

    #[tokio::test]
    async fn create_new_comment_because_existing_one_was_open() {
        let cfg = Config::builder().build().unwrap();

        let mut db = MockDB::new();
        db.expect_get_pending_annual_review_notifications()
            .times(1)
            .returning(|| {
                Box::pin(future::ready(Ok(vec![AnnualReviewNotification {
                    project_id: *PROJECT_ID,
                    community_repo_url: REPO1_URL.to_string(),
                    issue_number: Some(ISSUE_NUMBER),
                }])))
            });
        db.expect_pre_register_annual_review_notification()
            .with(eq(*PROJECT_ID), eq(REPO1_URL))
            .times(1)
            .returning(|_, _| Box::pin(future::ready(Ok(*NOTIFICATION_ID))));
        db.expect_update_annual_review_notification()
            .with(
                eq(*NOTIFICATION_ID),
                eq(Some(ISSUE_NUMBER)),
                eq(Some(COMMENT_ID)),
            )
            .times(1)
            .returning(|_, _, _| Box::pin(future::ready(Ok(()))));

        let mut gh = MockGH::new();
        gh.expect_is_issue_closed()
            .with(eq("owner"), eq("repo1"), eq(ISSUE_NUMBER))
            .times(1)
            .returning(|_, _, _| Box::pin(future::ready(Ok(false))));
        gh.expect_create_comment()
            .with(
                eq("owner"),
                eq("repo1"),
                eq(ISSUE_NUMBER),
                eq(tmpl::AnnualReviewDueReminder {}.render().unwrap()),
            )
            .times(1)
            .returning(|_, _, _, _| Box::pin(future::ready(Ok(COMMENT_ID))));

        run(&cfg, Box::new(db), Box::new(gh)).await.unwrap();
    }
}
