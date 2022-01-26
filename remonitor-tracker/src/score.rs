use crate::tracker::Linter;
use remonitor_linter::Report;
use serde::{Deserialize, Serialize};

/// Project's score information.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Score {
    global: usize,
    documentation: usize,
    license: usize,
    quality: usize,
    security: usize,
}

impl Score {
    fn new() -> Self {
        Score {
            global: 0,
            documentation: 0,
            license: 0,
            quality: 0,
            security: 0,
        }
    }
}

/// Calculate score for the given linter report.
pub(crate) fn calculate(linter: Linter, report: Report) -> Score {
    match linter {
        Linter::Core => core_linter_score(report),
    }
}

/// Merge the scores provided into a single score.
pub(crate) fn merge(scores: Vec<Score>) -> Score {
    let mut score = Score::new();
    for entry in &scores {
        score.global += entry.global;
        score.documentation += entry.documentation;
        score.license += entry.license;
        score.quality += entry.quality;
        score.security += entry.security;
    }
    let n = scores.len();
    score.global /= n;
    score.documentation /= n;
    score.license /= n;
    score.quality /= n;
    score.security /= n;
    score
}

/// Returns the rating corresponding to the score provided.
pub(crate) fn rating(score: &Score) -> String {
    match score.global {
        75..=100 => "a",
        50..=74 => "b",
        25..=49 => "c",
        0..=24 => "d",
        _ => "?",
    }
    .to_string()
}

/// Calculate score for the given report produced by the core linter.
fn core_linter_score(report: Report) -> Score {
    let mut score = Score::new();

    // Documentation
    if report.documentation.adopters {
        score.documentation += 5;
    }
    if report.documentation.code_of_conduct {
        score.documentation += 10;
    }
    if report.documentation.contributing {
        score.documentation += 10;
    }
    if report.documentation.changelog {
        score.documentation += 5;
    }
    if report.documentation.governance {
        score.documentation += 10;
    }
    if report.documentation.maintainers || report.documentation.owners {
        score.documentation += 5;
    }
    if report.documentation.readme {
        score.documentation += 50;
    }
    if report.documentation.roadmap {
        score.documentation += 5;
    }

    // License
    if report.license.spdx_id.is_some() {
        score.license += 25;
    }
    if let Some(approved) = report.license.approved {
        if approved {
            score.license += 75;
        }
    }

    // Quality
    if report.quality.fossa {
        score.quality += 50;
    }
    if report.quality.openssf_badge {
        score.quality += 50;
    }

    // Security
    if report.security.security_policy {
        score.security = 100;
    }

    // Global
    score.global = (score.documentation + score.license + score.quality + score.security) / 4;

    score
}
