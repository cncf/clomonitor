use anyhow::{format_err, Error};
use clap::ArgEnum;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

mod check;
pub mod primary;
pub mod secondary;
pub use check::CheckResult;

/// Supported repository kinds.
#[derive(Debug, Clone, ArgEnum)]
pub enum RepositoryKind {
    Primary,
    Secondary,
}

impl FromStr for RepositoryKind {
    type Err = Error;

    fn from_str(input: &str) -> Result<RepositoryKind, Self::Err> {
        match input {
            "primary" => Ok(RepositoryKind::Primary),
            "secondary" => Ok(RepositoryKind::Secondary),
            _ => Err(format_err!("invalid repository kind")),
        }
    }
}

/// A core linter report specific to a repository kind.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "report_kind")]
#[allow(clippy::large_enum_variant)]
pub enum Report {
    Primary(primary::Report),
    Secondary(secondary::Report),
}

/// Linter configuration options.
pub struct LintOptions {
    pub root: PathBuf,
    pub kind: RepositoryKind,
    pub url: String,
    pub github_token: Option<String>,
}

/// Lint the path provided and return a report.
pub async fn lint(options: LintOptions) -> Result<Report, Error> {
    // Initialize Github API client
    let mut builder = octocrab::Octocrab::builder();
    if let Some(token) = &options.github_token {
        builder = builder.personal_token(token.to_string());
    }
    octocrab::initialise(builder)?;

    // Run the linter corresponding to the repository kind provided
    Ok(match &options.kind {
        RepositoryKind::Primary => Report::Primary(primary::lint(options).await?),
        RepositoryKind::Secondary => Report::Secondary(secondary::lint(options).await?),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_kind_from_str_valid_kind() {
        assert!(matches!("primary".parse(), Ok(RepositoryKind::Primary)));
        assert!(matches!("secondary".parse(), Ok(RepositoryKind::Secondary)));
    }

    #[test]
    fn repository_kind_from_str_invalid_kind() {
        assert!(matches!("invalid".parse::<RepositoryKind>(), Err(_)));
    }
}
