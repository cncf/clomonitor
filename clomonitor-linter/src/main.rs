#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::doc_markdown, clippy::wildcard_imports)]

use std::{env, io, path::PathBuf};

use anyhow::{Result, format_err};
use clap::{Parser, ValueEnum};
use clomonitor_core::{
    linter::{CheckSet, CoreLinter, Linter, LinterInput, ToolMode, ToolsConfig},
    score,
};
use serde_json::json;
use tokio::signal;

mod table;

/// Environment variable containing Github token.
const GITHUB_TOKEN: &str = "GITHUB_TOKEN";

/// CLI output format options.
#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Json,
    Table,
}

/// External tool execution options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ToolExecution {
    /// Run the tool binary available in PATH
    Local,
    /// Do not run the tool (checks relying on it are reported as failed)
    Disabled,
}

impl From<ToolExecution> for ToolMode {
    fn from(execution: ToolExecution) -> Self {
        match execution {
            ToolExecution::Local => ToolMode::Local,
            ToolExecution::Disabled => ToolMode::Disabled,
        }
    }
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
scope) by setting the GITHUB_TOKEN environment variable.

Some checks rely on external tools: OpenSSF Scorecard (security checks) and
AFDocs (agent readiness checks, https://afdocs.dev). The scorecard and afdocs
binaries available in PATH are used, but a tool can be disabled (--scorecard
disabled / --afdocs disabled). AFDocs never receives the GitHub token."
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

    /// AFDocs execution mode (agent readiness checks, requires afdocs 0.20.x)
    #[clap(value_enum, long, default_value = "local")]
    afdocs: ToolExecution,

    /// OpenSSF Scorecard execution mode (scorecard backed security checks)
    #[clap(value_enum, long, default_value = "local")]
    scorecard: ToolExecution,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Check if required Github token is present in environment
    let Ok(github_token) = env::var(GITHUB_TOKEN) else {
        return Err(format_err!("{GITHUB_TOKEN} not found in environment"));
    };

    // Setup external tools execution modes
    let tools = ToolsConfig {
        afdocs: args.afdocs.into(),
        scorecard: args.scorecard.into(),
    };
    if tools.afdocs == ToolMode::Disabled {
        eprintln!("warning: afdocs disabled, agent readiness checks will be reported as failed");
    }
    if tools.scorecard == ToolMode::Disabled {
        eprintln!(
            "warning: scorecard disabled, scorecard backed checks will be reported as failed"
        );
    }

    // Lint repository provided. Tools run in their own process group, so the
    // run must be dropped explicitly on Ctrl-C for them to be killed
    let input = LinterInput {
        project: None,
        root: args.path.clone(),
        url: args.url.clone(),
        check_sets: args.check_set.clone(),
        github_token,
        tools,
    };
    let linter = CoreLinter::new();
    let report = tokio::select! {
        report = linter.lint(&input) => report?,
        _ = signal::ctrl_c() => return Err(format_err!("interrupted")),
    };
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
