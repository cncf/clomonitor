use anyhow::Result;
use clap::Parser;
use clomonitor_core::{
    linter::{lint, CheckSet, GithubOptions, LintOptions, LintServices},
    score,
};
use display::display;
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
        root: args.path.clone(),
        url: args.url.clone(),
        check_sets: args.check_set.clone(),
    };
    let svc = LintServices::new(GithubOptions::default())?;
    let report = lint(&opts, &svc).await?;
    let score = score::calculate(&report);
    display(&report, &score, &args)
}
