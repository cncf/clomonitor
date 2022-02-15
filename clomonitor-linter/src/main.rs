mod display;

use anyhow::{format_err, Error};
use clap::Parser;
use clomonitor_core::{
    linter::{lint, LintOptions, RepositoryKind},
    score, Linter,
};
use display::*;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Repository root path
    #[clap(long, parse(from_os_str), default_value = ".")]
    path: PathBuf,

    /// Repository kind
    #[clap(arg_enum, long, default_value = "primary")]
    kind: RepositoryKind,

    /// Linter pass score
    #[clap(long, default_value = "80")]
    pass_score: usize,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    // Lint repository provided and display results
    let options = LintOptions {
        root: &args.path,
        kind: &args.kind,
    };
    let report = lint(options)?;
    let score = score::calculate(Linter::Core, &report);
    display(&report, &score);

    // Check if the linter succeeded acording to the provided pass score
    if score.global() >= args.pass_score {
        println!(
            "{SUCCESS_SYMBOL} Succeeded with a global score of {}\n",
            score.global()
        );
        Ok(())
    } else {
        Err(format_err!(
            "{FAILURE_SYMBOL} Failed with a global score of {} (pass score is {})\n",
            score.global(),
            args.pass_score
        ))
    }
}
