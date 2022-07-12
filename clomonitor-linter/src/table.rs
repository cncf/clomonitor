use crate::Args;
use anyhow::Result;
use clomonitor_core::{
    linter::{CheckOutput, Report},
    score::Score,
};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table, *};
use std::{fs, io};

const SUCCESS_SYMBOL: char = '✓';
const FAILURE_SYMBOL: char = '✗';
const WARNING_SYMBOL: char = '!';
const NOT_APPLICABLE_MSG: &str = "n/a";
const EXEMPT_MSG: &str = "Exempt";

/// Print the linter results provided.
pub(crate) fn display(
    report: &Report,
    score: &Score,
    args: &Args,
    w: &mut impl io::Write,
) -> Result<()> {
    writeln!(w, "\nCLOMonitor linter results\n")?;

    // Repository information
    let local_path = match fs::canonicalize(&args.path) {
        Ok(cp) => cp.to_string_lossy().to_string(),
        Err(_) => args.path.to_string_lossy().to_string(),
    };
    writeln!(w, "Repository information\n")?;
    let mut repo_info = new_table();
    repo_info
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .add_row(vec![cell_entry("Local path"), cell_entry(&local_path)])
        .add_row(vec![cell_entry("Remote url"), cell_entry(&args.url)])
        .add_row(vec![
            cell_entry("Check sets"),
            cell_entry(&format!("{:?}", args.check_set)),
        ]);
    writeln!(w, "{}\n", repo_info)?;

    // Summary table
    writeln!(w, "Score summary\n")?;
    let mut score_summary = new_table();
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
    writeln!(w, "{}\n", score_summary)?;

    // Checks table
    writeln!(w, "Checks summary\n")?;
    let mut checks_summary = new_table();
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
            Cell::new(match &report.license.license_spdx_id {
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
            cell_check(&report.license.license_approved),
        ])
        .add_row(vec![
            cell_entry("License / Scanning"),
            cell_check(&report.license.license_scanning),
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
            cell_entry("Best practices / GitHub discussions"),
            cell_check(&report.best_practices.github_discussions),
        ])
        .add_row(vec![
            cell_entry("Best practices / Google Analytics 4"),
            cell_check(&report.best_practices.ga4),
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
            cell_entry("Security / Binary artifacts"),
            cell_check(&report.security.binary_artifacts),
        ])
        .add_row(vec![
            cell_entry("Security / Branch protection"),
            cell_check(&report.security.branch_protection),
        ])
        .add_row(vec![
            cell_entry("Security / Code review"),
            cell_check(&report.security.code_review),
        ])
        .add_row(vec![
            cell_entry("Security / Dangerous workflow"),
            cell_check(&report.security.dangerous_workflow),
        ])
        .add_row(vec![
            cell_entry("Security / Dependency update tool"),
            cell_check(&report.security.dependency_update_tool),
        ])
        .add_row(vec![
            cell_entry("Security / Maintained"),
            cell_check(&report.security.maintained),
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
            cell_entry("Security / Signed release"),
            cell_check(&report.security.signed_releases),
        ])
        .add_row(vec![
            cell_entry("Security / Token permissions"),
            cell_check(&report.security.token_permissions),
        ])
        .add_row(vec![
            cell_entry("Security / Vulnerabilities"),
            cell_check(&report.security.vulnerabilities),
        ])
        .add_row(vec![
            cell_entry("Legal / Trademark disclaimer"),
            cell_check(&report.legal.trademark_disclaimer),
        ]);
    writeln!(w, "{}\n", checks_summary)?;

    // Check if the linter succeeded according to the provided pass score
    if score.global() >= args.pass_score {
        writeln!(
            w,
            "{SUCCESS_SYMBOL} Succeeded with a global score of {}\n",
            score.global().round()
        )?;
    } else {
        writeln!(
            w,
            "{FAILURE_SYMBOL} Failed with a global score of {} (pass score is {})\n",
            score.global().round(),
            args.pass_score
        )?;
    }

    Ok(())
}

/// Helper function to create a new table that will be forced to use a non-tty
/// mode when running tests.
#[allow(clippy::let_and_return, unused_mut)]
fn new_table() -> Table {
    let mut table = Table::new();

    #[cfg(test)]
    table.force_no_tty();

    table
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

#[cfg(test)]
mod tests {
    use super::display;
    use crate::{Args, Format};
    use clomonitor_core::{
        linter::{
            BestPractices, CheckOutput, CheckSet, Documentation, Legal, License, Report, Security,
        },
        score::Score,
    };
    use std::{fs, path::PathBuf, str, str::FromStr};

    #[test]
    fn display_prints_results() {
        // Setup test linter results
        let report = Report {
            documentation: Documentation {
                adopters: Some(true.into()),
                code_of_conduct: Some(true.into()),
                contributing: Some(true.into()),
                changelog: Some(true.into()),
                governance: Some(true.into()),
                maintainers: Some(true.into()),
                readme: Some(true.into()),
                roadmap: Some(true.into()),
                website: Some(true.into()),
            },
            license: License {
                license_approved: Some(CheckOutput {
                    passed: true,
                    value: Some(true),
                    ..Default::default()
                }),
                license_scanning: Some(CheckOutput {
                    passed: true,
                    url: (Some("https://license-scanning.url".to_string())),
                    ..CheckOutput::default()
                }),
                license_spdx_id: Some(Some("Apache-2.0".to_string()).into()),
            },
            best_practices: BestPractices {
                artifacthub_badge: Some(CheckOutput {
                    exempt: true,
                    ..Default::default()
                }),
                cla: Some(true.into()),
                community_meeting: Some(true.into()),
                dco: Some(true.into()),
                ga4: Some(true.into()),
                github_discussions: Some(true.into()),
                openssf_badge: Some(true.into()),
                recent_release: Some(true.into()),
                slack_presence: Some(true.into()),
            },
            security: Security {
                binary_artifacts: Some(true.into()),
                branch_protection: Some(true.into()),
                code_review: Some(true.into()),
                dangerous_workflow: Some(true.into()),
                dependency_update_tool: Some(true.into()),
                maintained: Some(true.into()),
                sbom: Some(true.into()),
                security_policy: Some(true.into()),
                signed_releases: Some(true.into()),
                token_permissions: Some(true.into()),
                vulnerabilities: Some(true.into()),
            },
            legal: Legal {
                trademark_disclaimer: Some(true.into()),
            },
        };
        let score = Score {
            global: 99.99999999999999,
            global_weight: 90,
            documentation: Some(100.0),
            documentation_weight: Some(30),
            license: Some(100.0),
            license_weight: Some(20),
            best_practices: Some(100.0),
            best_practices_weight: Some(20),
            security: Some(100.0),
            security_weight: Some(15),
            legal: Some(100.0),
            legal_weight: Some(5),
        };
        let args = Args {
            path: PathBuf::from_str("test-repo-path").unwrap(),
            url: "https://github.com/test-org/test-repo".to_string(),
            check_set: vec![CheckSet::Code, CheckSet::Community],
            pass_score: 80.0,
            format: Format::Table,
        };

        // Display linter results using a vector as output
        let mut w = Vec::new();
        display(&report, &score, &args, &mut w).unwrap();

        let golden_path = "src/testdata/display.golden";

        // Write output to golden file (uncomment line below to update golden)
        // fs::write(golden_path, &w).unwrap();

        // Check output matches golden file content
        let output = str::from_utf8(w.as_slice()).unwrap();
        let golden = fs::read_to_string(golden_path).unwrap();
        assert_eq!(output, golden);
    }
}
