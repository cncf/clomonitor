use crate::Args;
use anyhow::{format_err, Result};
use clomonitor_core::{
    linter::{CheckOutput, Report},
    score::Score,
};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table, *};
use std::fs;

const SUCCESS_SYMBOL: char = '✓';
const FAILURE_SYMBOL: char = '✗';
const WARNING_SYMBOL: char = '!';
const NOT_APPLICABLE_MSG: &str = "n/a";
const EXEMPT_MSG: &str = "Exempt";

/// Print the linter results provided.
pub(crate) fn display(report: &Report, score: &Score, args: &Args) -> Result<()> {
    println!("CLOMonitor linter results\n");

    // Repository information
    println!("Repository information\n");
    let mut repo_info = Table::new();
    repo_info
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .add_row(vec![
            cell_entry("Local path"),
            cell_entry(&fs::canonicalize(&args.path)?.to_string_lossy()),
        ])
        .add_row(vec![cell_entry("Remote url"), cell_entry(&args.url)])
        .add_row(vec![
            cell_entry("Check sets"),
            cell_entry(&format!("{:?}", args.check_set)),
        ]);
    println!("{}\n", repo_info);

    // Summary table
    println!("Score summary\n");
    let mut score_summary = Table::new();
    score_summary
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![cell_header("Section"), cell_header("Score")])
        .add_row(vec![cell_entry("Global"), cell_score(Some(score.global))])
        .add_row(vec![
            cell_entry("Documentation"),
            cell_score(score.documentation),
        ])
        .add_row(vec![cell_entry("License"), cell_score(score.license)])
        .add_row(vec![
            cell_entry("Best practices"),
            cell_score(score.best_practices),
        ])
        .add_row(vec![cell_entry("Security"), cell_score(score.security)])
        .add_row(vec![cell_entry("Legal"), cell_score(score.legal)]);
    println!("{}\n", score_summary);

    // Checks table
    println!("Checks summary\n");
    let mut checks_summary = Table::new();
    checks_summary
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![cell_header("Check"), cell_header("Passed")])
        .add_row(vec![
            cell_entry("Documentation / Adopters"),
            cell_check(&report.documentation.adopters),
        ])
        .add_row(vec![
            cell_entry("Documentation / Changelog"),
            cell_check(&report.documentation.changelog),
        ])
        .add_row(vec![
            cell_entry("Documentation / Code of conduct"),
            cell_check(&report.documentation.code_of_conduct),
        ])
        .add_row(vec![
            cell_entry("Documentation / Contributing"),
            cell_check(&report.documentation.contributing),
        ])
        .add_row(vec![
            cell_entry("Documentation / Governance"),
            cell_check(&report.documentation.governance),
        ])
        .add_row(vec![
            cell_entry("Documentation / Maintainers"),
            cell_check(&report.documentation.maintainers),
        ])
        .add_row(vec![
            cell_entry("Documentation / Readme"),
            cell_check(&report.documentation.readme),
        ])
        .add_row(vec![
            cell_entry("Documentation / Roadmap"),
            cell_check(&report.documentation.roadmap),
        ])
        .add_row(vec![
            cell_entry("Documentation / Website"),
            cell_check(&report.documentation.website),
        ])
        .add_row(vec![
            cell_entry("License"),
            Cell::new(match &report.license.spdx_id {
                None => NOT_APPLICABLE_MSG.to_string(),
                Some(r) => r
                    .value
                    .clone()
                    .unwrap_or_else(|| "Not detected".to_string()),
            })
            .set_alignment(CellAlignment::Center)
            .add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            cell_entry("License / Approved"),
            cell_check(&report.license.approved),
        ])
        .add_row(vec![
            cell_entry("License / Scanning"),
            cell_check(&report.license.scanning),
        ])
        .add_row(vec![
            cell_entry("Best practices / Artifact Hub badge"),
            cell_check(&report.best_practices.artifacthub_badge),
        ])
        .add_row(vec![
            cell_entry("Best practices / CLA"),
            cell_check(&report.best_practices.cla),
        ])
        .add_row(vec![
            cell_entry("Best practices / Community meeting"),
            cell_check(&report.best_practices.community_meeting),
        ])
        .add_row(vec![
            cell_entry("Best practices / DCO"),
            cell_check(&report.best_practices.dco),
        ])
        .add_row(vec![
            cell_entry("Best practices / OpenSSF (CII) badge"),
            cell_check(&report.best_practices.openssf_badge),
        ])
        .add_row(vec![
            cell_entry("Best practices / Recent release"),
            cell_check(&report.best_practices.recent_release),
        ])
        .add_row(vec![
            cell_entry("Best practices / Slack presence"),
            cell_check(&report.best_practices.slack_presence),
        ])
        .add_row(vec![
            cell_entry("Security / SBOM"),
            cell_check(&report.security.sbom),
        ])
        .add_row(vec![
            cell_entry("Security / Security policy"),
            cell_check(&report.security.security_policy),
        ])
        .add_row(vec![
            cell_entry("Legal / Trademark disclaimer"),
            cell_check(&report.legal.trademark_disclaimer),
        ]);
    println!("{}\n", checks_summary);

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
fn cell_score(score: Option<f64>) -> Cell {
    let (content, color) = match score {
        Some(v) => match v as usize {
            75..=100 => (v.round().to_string(), Color::Green),
            50..=74 => (v.round().to_string(), Color::Yellow),
            25..=49 => (v.round().to_string(), Color::DarkYellow),
            0..=24 => (v.round().to_string(), Color::Red),
            _ => ("?".to_string(), Color::Grey),
        },
        None => (NOT_APPLICABLE_MSG.to_string(), Color::Grey),
    };
    Cell::new(content)
        .set_alignment(CellAlignment::Center)
        .add_attribute(Attribute::Bold)
        .fg(color)
}

/// Build a cell used for checks output.
fn cell_check<T>(output: &Option<CheckOutput<T>>) -> Cell {
    let (content, color) = match output {
        Some(r) => match (r.passed, r.exempt, r.failed) {
            (true, _, _) => (SUCCESS_SYMBOL.to_string(), Color::Green),
            (false, true, _) => (EXEMPT_MSG.to_string(), Color::Grey),
            (false, _, false) => (FAILURE_SYMBOL.to_string(), Color::Red),
            (false, _, true) => (WARNING_SYMBOL.to_string(), Color::Yellow),
        },
        None => (NOT_APPLICABLE_MSG.to_string(), Color::Grey),
    };
    Cell::new(content)
        .set_alignment(CellAlignment::Center)
        .add_attribute(Attribute::Bold)
        .fg(color)
}
