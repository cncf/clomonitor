pub mod primary;
pub mod secondary;

use crate::{linter::Report, Linter};
use serde::{Deserialize, Serialize};

/// Score information specific to a repository kind linter report.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "score_kind")]
pub enum Score {
    Primary(primary::Score),
    Secondary(secondary::Score),
}

impl Score {
    /// Return the score's global value.
    pub fn global(&self) -> usize {
        match self {
            Score::Primary(report) => report.global,
            Score::Secondary(report) => report.global,
        }
    }

    /// Return the score's rating (a, b, c or d).
    pub fn rating(&self) -> char {
        match self.global() {
            75..=100 => 'a',
            50..=74 => 'b',
            25..=49 => 'c',
            0..=24 => 'd',
            _ => '?',
        }
    }
}

/// Calculate score for the given linter report.
pub fn calculate(linter: Linter, report: &Report) -> Score {
    match linter {
        Linter::Core => match report {
            Report::Primary(report) => Score::Primary(primary::calculate_score(report)),
            Report::Secondary(report) => Score::Secondary(secondary::calculate_score(report)),
        },
    }
}

/// Merge the scores provided into a single score.
pub fn merge(scores: Vec<Score>) -> Score {
    // Count scores of each kind
    let mut n_pri: usize = 0;
    let mut n_sec: usize = 0;
    for entry in &scores {
        match entry {
            Score::Primary(_) => n_pri += 1,
            Score::Secondary(_) => n_sec += 1,
        }
    }

    // Call the corresponding merge function based on the scores kind
    match (n_pri, n_sec) {
        (_, 0) => {
            let k = 1.0 / n_pri as f64;
            Score::Primary(merge_primaries(scores, k))
        }
        (0, _) => {
            let k = 1.0 / n_sec as f64;
            Score::Secondary(merge_secondaries(scores, k))
        }
        (_, _) => {
            let k_pri_only = 1.0 / n_pri as f64;
            let k_pri = 0.8 / n_pri as f64;
            let k_sec = 0.2 / n_sec as f64;
            Score::Primary(merge_mixed(scores, k_pri_only, k_pri, k_sec))
        }
    }
}

/// Merge the primary scores provided into a single primary score.
fn merge_primaries(scores: Vec<Score>, k: f64) -> primary::Score {
    let mut score = primary::Score::new();
    for entry in &scores {
        if let Score::Primary(entry) = entry {
            score.global += (entry.global as f64 * k).round() as usize;
            score.documentation += (entry.documentation as f64 * k).round() as usize;
            score.license += (entry.license as f64 * k).round() as usize;
            score.best_practices += (entry.best_practices as f64 * k).round() as usize;
            score.security += (entry.security as f64 * k).round() as usize;
        }
    }
    score
}

/// Merge the secondary scores provided into a single secondary score.
fn merge_secondaries(scores: Vec<Score>, k: f64) -> secondary::Score {
    let mut score = secondary::Score::new();
    for entry in &scores {
        if let Score::Secondary(entry) = entry {
            score.global += (entry.global as f64 * k).round() as usize;
            score.documentation += (entry.documentation as f64 * k).round() as usize;
            score.license += (entry.license as f64 * k).round() as usize;
        }
    }
    score
}

/// Merge the scores provided (primaries and secondaries) into a single primary score.
///
/// - k_pri_only: used for sections that only exist in primary scores
/// - k_pri: used for primary scores sections
/// - k_sec: used for secondary scores sections
fn merge_mixed(scores: Vec<Score>, k_pri_only: f64, k_pri: f64, k_sec: f64) -> primary::Score {
    let mut score = primary::Score::new();
    for entry in &scores {
        match entry {
            Score::Primary(entry) => {
                score.global += (entry.global as f64 * k_pri).round() as usize;
                score.documentation += (entry.documentation as f64 * k_pri).round() as usize;
                score.license += (entry.license as f64 * k_pri).round() as usize;
                score.best_practices += (entry.best_practices as f64 * k_pri_only).round() as usize;
                score.security += (entry.security as f64 * k_pri_only).round() as usize;
            }
            Score::Secondary(entry) => {
                score.global += (entry.global as f64 * k_sec).round() as usize;
                score.documentation += (entry.documentation as f64 * k_sec).round() as usize;
                score.license += (entry.license as f64 * k_sec).round() as usize;
            }
        }
    }
    score
}
