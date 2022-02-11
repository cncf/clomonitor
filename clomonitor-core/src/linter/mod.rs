mod check;
mod patterns;
pub mod primary;
pub mod secondary;

use anyhow::{format_err, Error};
use clap::ArgEnum;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::str::FromStr;

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
pub enum Report {
    Primary(primary::Report),
    Secondary(secondary::Report),
}

/// Linter configuration options.
pub struct LintOptions<'a> {
    pub root: &'a Path,
    pub kind: &'a RepositoryKind,
}

/// Lint the path provided and return a report.
pub fn lint(options: LintOptions) -> Result<Report, Error> {
    Ok(match options.kind {
        RepositoryKind::Primary => Report::Primary(primary::lint(options.root)?),
        RepositoryKind::Secondary => Report::Secondary(secondary::lint(options.root)?),
    })
}
