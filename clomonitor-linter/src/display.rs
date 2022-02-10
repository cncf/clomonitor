use clomonitor_core::{linter::Report, score::Score};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table, *};

pub(crate) const SUCCESS_SYMBOL: char = '✓';
pub(crate) const FAILURE_SYMBOL: char = '✗';

/// Print the linter results provided.
pub(crate) fn display(report: &Report, score: &Score) {
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
        .add_row(vec![
            cell_entry("Best practices"),
            cell_score(score.best_practices),
        ])
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
            cell_entry("License / FOSSA badge"),
            cell_check(report.license.fossa_badge),
        ])
        .add_row(vec![
            cell_entry("Best practices / OpenSSF (CII) badge"),
            cell_check(report.best_practices.openssf_badge),
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
