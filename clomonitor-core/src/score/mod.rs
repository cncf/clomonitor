use crate::linter::{Linter, Report};
use serde::{Deserialize, Serialize};

/// Score information.
#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    pub global: usize,
    pub documentation: usize,
    pub license: usize,
    pub best_practices: usize,
    pub security: usize,
}

impl Score {
    /// Create a new score with all values set to zero.
    pub fn new() -> Self {
        Score {
            global: 0,
            documentation: 0,
            license: 0,
            best_practices: 0,
            security: 0,
        }
    }

    /// Return the score's rating (a, b, c or d).
    pub fn rating(&self) -> char {
        match self.global {
            75..=100 => 'a',
            50..=74 => 'b',
            25..=49 => 'c',
            0..=24 => 'd',
            _ => '?',
        }
    }
}

impl Default for Score {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate score for the given linter report.
pub fn calculate(linter: Linter, report: &Report) -> Score {
    match linter {
        Linter::Core => calculate_core_linter_score(report),
    }
}

/// Calculate score for the given report produced by the core linter.
fn calculate_core_linter_score(report: &Report) -> Score {
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
    if report.documentation.maintainers {
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
        score.license += 20;
    }
    if let Some(approved) = report.license.approved {
        if approved {
            score.license += 60;
        }
    }
    if report.license.fossa_badge {
        score.license += 20;
    }

    // Best practices
    if report.best_practices.openssf_badge {
        score.best_practices += 100;
    }

    // Security
    if report.security.security_policy {
        score.security = 100;
    }

    // Global
    score.global =
        (score.documentation + score.license + score.best_practices + score.security) / 4;

    score
}

/// Merge the scores provided into a single score.
pub fn merge(scores: Vec<Score>) -> Score {
    let mut score = Score::new();
    for entry in &scores {
        score.global += entry.global;
        score.documentation += entry.documentation;
        score.license += entry.license;
        score.best_practices += entry.best_practices;
        score.security += entry.security;
    }
    let n = scores.len();
    score.global /= n;
    score.documentation /= n;
    score.license /= n;
    score.best_practices /= n;
    score.security /= n;
    score
}
