use anyhow::{format_err, Error};
use clap::Parser;
use clomonitor_core::{
    linter::{lint, CheckSet, LintOptions},
    score,
};
use display::*;
use std::path::PathBuf;

mod display;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Repository root path
    #[clap(long, parse(from_os_str), default_value = ".")]
    path: PathBuf,

    /// Sets of checks to run
    #[clap(arg_enum, long, default_values = &["code", "community"])]
    check_set: Vec<CheckSet>,

    /// Linter pass score
    #[clap(long, default_value = "80")]
    pass_score: f64,

    /// Repository url [https://github.com/org/repo] (required for some GitHub remote checks)
    #[clap(long)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    // Lint repository provided and display results
    println!("\nRunning CLOMonitor linter...\n");
    let options = LintOptions {
        check_sets: args.check_set,
        root: args.path,
        url: args.url,
        github_token: None,
    };
    let report = lint(options).await?;
    let score = score::calculate(&report);
    display(&report, &score);

    // Check if the linter succeeded acording to the provided pass score
    if score.global() >= args.pass_score {
        println!(
            "{SUCCESS_SYMBOL} Succeeded with a global score of {}\n",
            score.global().round()
        );
        Ok(())
    } else {
        Err(format_err!(
            "{FAILURE_SYMBOL} Failed with a global score of {} (pass score is {})\n",
            score.global().round(),
            args.pass_score
        ))
    }
}
