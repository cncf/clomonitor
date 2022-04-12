use anyhow::{format_err, Result};
use clap::Parser;
use clomonitor_core::{
    linter::{lint, CheckSet, LintCredentials, LintOptions, LintServices},
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

    /// Repository url [https://github.com/org/repo] (required for some GitHub remote checks)
    #[clap(long)]
    url: String,

    /// Sets of checks to run
    #[clap(arg_enum, long, default_values = &["code", "community"])]
    check_set: Vec<CheckSet>,

    /// Linter pass score
    #[clap(long, default_value = "80")]
    pass_score: f64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Lint repository provided and display results
    println!("\nRunning CLOMonitor linter...\n");
    let opts = LintOptions {
        root: args.path,
        url: args.url,
        check_sets: args.check_set,
    };
    let svc = LintServices::new(LintCredentials::default())?;
    let report = lint(&opts, &svc).await?;
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
