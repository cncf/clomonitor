use anyhow::Result;
use async_trait::async_trait;
use config::Config;
#[cfg(test)]
use mockall::automock;
use octorust::{
    auth::Credentials,
    types::{IssuesCreateRequest, PullsUpdateReviewRequest, TitleOneOf},
    Client,
};

/// Trait that defines some operations a GH implementation must support.
#[async_trait]
#[cfg_attr(test, automock)]
pub(crate) trait GH {
    /// Create an issue comment.
    async fn create_comment(
        &self,
        owner: &str,
        repo: &str,
        issue_number: IssueNumber,
        body: &str,
    ) -> Result<CommentId>;

    /// Create an issue.
    async fn create_issue(
        &self,
        owner: &str,
        repo: &str,
        title: &str,
        body: &str,
    ) -> Result<IssueNumber>;

    /// Check if the issue provided is closed.
    async fn is_issue_closed(
        &self,
        owner: &str,
        repo: &str,
        issue_number: IssueNumber,
    ) -> Result<bool>;
}

/// Type alias to represent a GH trait object.
pub(crate) type DynGH = Box<dyn GH + Send + Sync>;

/// Type alias to represent an issue number.
type IssueNumber = i64;

/// Type alias to represent a comment id.
type CommentId = i64;

/// GH implementation backed by the GitHub API.
pub(crate) struct GHApi {
    client: Client,
}

impl GHApi {
    /// Create a new GHApi instance.
    pub(crate) fn new(cfg: &Config) -> Result<Self> {
        let token = cfg.get_string("creds.githubToken")?;
        let client = Client::new(
            format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            Credentials::Token(token),
        )?;
        Ok(Self { client })
    }
}

#[async_trait]
impl GH for GHApi {
    /// [GH::create_comment]
    async fn create_comment(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &str,
    ) -> Result<CommentId> {
        let body = &PullsUpdateReviewRequest {
            body: body.to_string(),
        };
        let comment = self
            .client
            .issues()
            .create_comment(owner, repo, issue_number, body)
            .await?;
        Ok(comment.id)
    }

    /// [GH::create_issue]
    async fn create_issue(
        &self,
        owner: &str,
        repo: &str,
        title: &str,
        body: &str,
    ) -> Result<IssueNumber> {
        let body = IssuesCreateRequest {
            assignee: String::new(),
            assignees: vec![],
            body: body.to_string(),
            labels: vec![],
            milestone: None,
            title: TitleOneOf::String(title.to_string()),
        };
        let issue = self.client.issues().create(owner, repo, &body).await?;
        Ok(issue.number)
    }

    /// [GH::is_issue_closed]
    async fn is_issue_closed(
        &self,
        owner: &str,
        repo: &str,
        issue_number: IssueNumber,
    ) -> Result<bool> {
        let issue = self.client.issues().get(owner, repo, issue_number).await?;
        Ok(issue.closed_at.is_some())
    }
}
