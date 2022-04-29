use anyhow::{format_err, Result};
use clap::Parser;
use clomonitor_core::{
    linter::{lint, CheckSet, GithubOptions, LintOptions, LintServices},
    score,
};
use display::display;
use std::{env, io, path::PathBuf};

mod display;

/// Environment variable containing Github token.
const GITHUB_TOKEN: &str = "GITHUB_TOKEN";

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

    // Check if required Github token is present in environment
    let github_token = match env::var(GITHUB_TOKEN) {
        Err(_) => return Err(format_err!("{} not found in environment", GITHUB_TOKEN)),
        Ok(token) => token,
    };

    // Lint repository provided and display results
    println!("\nRunning CLOMonitor linter...\n");
    let opts = LintOptions {
        root: args.path.clone(),
        url: args.url.clone(),
        check_sets: args.check_set.clone(),
        github_token: github_token.clone(),
    };
    let svc = LintServices::new(GithubOptions {
        token: github_token,
        ..GithubOptions::default()
    })?;
    let report = lint(&opts, &svc).await?;
    let score = score::calculate(&report);
    display(&report, &score, &args, &mut io::stdout())
}
