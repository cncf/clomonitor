#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::doc_markdown, clippy::wildcard_imports)]

use std::{env, io, path::PathBuf};

use anyhow::{format_err, Result};
use clap::{Parser, ValueEnum};
use clomonitor_core::{
    linter::{CheckSet, CoreLinter, Linter, LinterInput},
    score,
};
use serde_json::json;

mod table;

/// Environment variable containing Github token.
const GITHUB_TOKEN: &str = "GITHUB_TOKEN";

/// CLI output format options.
#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Json,
    Table,
}

#[derive(Debug, Parser)]
#[clap(
    author,
    version,
    about = "Checks repository to verify it meets certain project health best practices

The CLOMonitor linter runs some checks on the repository provided and produces
a report with the result. Some of the checks are done locally using the path
provided and some remotely as they rely on external APIs. Only GitHub repos
are supported at the moment. For more information about the checks, please see
https://clomonitor.io/docs/topics/checks/. The exit code will be 0 if the
linter runs successfully and the score is equal or higher than the pass score
provided, or non-zero otherwise.

This tool uses the Github GraphQL API for some checks, which requires
authentication. Please make sure you provide a Github token (with public_repo
scope) by setting the GITHUB_TOKEN environment variable."
)]
struct Args {
    /// Repository local path (used for checks that can be done locally)
    #[clap(long)]
    path: PathBuf,

    /// Repository url [https://github.com/org/repo] (used for some GitHub remote checks)
    #[clap(long)]
    url: String,

    /// Sets of checks to run
    #[clap(value_enum, long, default_values = &["code", "community"])]
    check_set: Vec<CheckSet>,

    /// Linter pass score
    #[clap(long, default_value = "75")]
    pass_score: f64,

    /// Output format
    #[clap(value_enum, long, default_value = "table")]
    format: Format,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Check if required Github token is present in environment
    let Ok(github_token) = env::var(GITHUB_TOKEN) else {
        return Err(format_err!("{} not found in environment", GITHUB_TOKEN));
    };

    // Lint repository provided
    let input = LinterInput {
        project: None,
        root: args.path.clone(),
        url: args.url.clone(),
        check_sets: args.check_set.clone(),
        github_token,
    };
    let report = CoreLinter::new().lint(&input).await?;
    let score = score::calculate(&report);

    // Display results using the requested format
    match args.format {
        Format::Table => table::display(&report, &score, &args, &mut io::stdout())?,
        Format::Json => {
            let output = json!({
                "report": report,
                "score": score,
            });
            println!("{output}");
        }
    }

    // Check if the linter succeeded according to the provided pass score
    if score.global() < args.pass_score {
        std::process::exit(1);
    }
    Ok(())
}
