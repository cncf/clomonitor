use anyhow::{format_err, Error};
use clap::{ArgEnum, Parser};
use clomonitor_core::{
    linter::{lint, Report},
    score::{self, Score},
    Linter,
};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table, *};
use std::path::PathBuf;

const SUCCESS_SYMBOL: char = '✓';
const FAILURE_SYMBOL: char = '✗';

#[derive(Debug, Clone, ArgEnum)]
enum Format {
    Table,
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    /// Output format
    #[clap(arg_enum, short, long, default_value = "table")]
    format: Format,

    /// Repository root path
    #[clap(short, long, parse(from_os_str), default_value = ".")]
    path: PathBuf,

    /// Pass score
    #[clap(long, default_value = "80")]
    pass_score: usize,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    // Lint repository provided and display results
    let report = lint(&args.path)?;
    let score = score::calculate(Linter::Core, &report);
    display(&report, &score);

    // Check if the linter succeeded acording to the provided pass score
    if score.global >= args.pass_score {
        println!(
            "{SUCCESS_SYMBOL} Succeeded with a global score of {}\n",
            score.global
        );
        Ok(())
    } else {
        Err(format_err!(
            "{FAILURE_SYMBOL} Failed with a global score of {} (pass score is {})\n",
            score.global,
            args.pass_score
        ))
    }
}

/// Print the linter results provided.
fn display(report: &Report, score: &Score) {
    println!("\nCloMonitor linter results\n");

    // Summary table
    println!("Score summary\n");
    let mut summary = Table::new();
    summary
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![cell_header("Section"), cell_header("Score")])
        .add_row(vec![cell_entry("Global"), cell_score(score.global)])
        .add_row(vec![
            cell_entry("Documentation"),
            cell_score(score.documentation),
        ])
        .add_row(vec![cell_entry("License"), cell_score(score.license)])
        .add_row(vec![cell_entry("Quality"), cell_score(score.quality)])
        .add_row(vec![cell_entry("Security"), cell_score(score.security)]);
    println!("{summary}\n");

    // Checks table
    println!("Checks summary\n");
    let mut checks = Table::new();
    checks
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![cell_header("Check"), cell_header("Passed")])
        .add_row(vec![
            cell_entry("Documentation / Adopters"),
            cell_check(report.documentation.adopters),
        ])
        .add_row(vec![
            cell_entry("Documentation / Code of conduct"),
            cell_check(report.documentation.code_of_conduct),
        ])
        .add_row(vec![
            cell_entry("Documentation / Contributing"),
            cell_check(report.documentation.contributing),
        ])
        .add_row(vec![
            cell_entry("Documentation / Changelog"),
            cell_check(report.documentation.changelog),
        ])
        .add_row(vec![
            cell_entry("Documentation / Governance"),
            cell_check(report.documentation.governance),
        ])
        .add_row(vec![
            cell_entry("Documentation / Maintainers"),
            cell_check(report.documentation.maintainers),
        ])
        .add_row(vec![
            cell_entry("Documentation / Readme"),
            cell_check(report.documentation.readme),
        ])
        .add_row(vec![
            cell_entry("Documentation / Roadmap"),
            cell_check(report.documentation.roadmap),
        ])
        .add_row(vec![
            cell_entry("License"),
            Cell::new(
                report
                    .license
                    .spdx_id
                    .clone()
                    .unwrap_or_else(|| "Not detected".to_string()),
            ),
        ])
        .add_row(vec![
            cell_entry("License / Approved"),
            cell_check(report.license.approved.unwrap_or(false)),
        ])
        .add_row(vec![
            cell_entry("Quality / Fossa badge"),
            cell_check(report.quality.fossa_badge),
        ])
        .add_row(vec![
            cell_entry("Quality / OpenSSF (CII) badge"),
            cell_check(report.quality.openssf_badge),
        ])
        .add_row(vec![
            cell_entry("Security / Security policy"),
            cell_check(report.security.security_policy),
        ]);
    println!("{checks}\n");
}

/// Build a cell used for headers text.
fn cell_header(title: &str) -> Cell {
    Cell::new(title)
        .set_alignment(CellAlignment::Center)
        .add_attribute(Attribute::Bold)
}

/// Build a cell used for regular entries text.
fn cell_entry(title: &str) -> Cell {
    Cell::new(title).set_alignment(CellAlignment::Left)
}

/// Build a cell used for scores.
fn cell_score(score: usize) -> Cell {
    let color = match score {
        75..=100 => Color::Green,
        50..=74 => Color::Yellow,
        25..=49 => Color::DarkYellow,
        0..=24 => Color::Red,
        _ => Color::Grey,
    };
    Cell::new(score.to_string())
        .set_alignment(CellAlignment::Center)
        .add_attribute(Attribute::Bold)
        .fg(color)
}

/// Build a cell used for checks symbols.
fn cell_check(passed: bool) -> Cell {
    let symbol: char;
    let color: Color;
    match passed {
        true => {
            symbol = SUCCESS_SYMBOL;
            color = Color::Green;
        }
        false => {
            symbol = FAILURE_SYMBOL;
            color = Color::Red;
        }
    };
    Cell::new(symbol)
        .set_alignment(CellAlignment::Center)
        .add_attribute(Attribute::Bold)
        .fg(color)
}
