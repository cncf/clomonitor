use crate::linter::primary::Report;
use serde::{Deserialize, Serialize};

/// Score information for a repository of kind primary.
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Score {
    pub global: usize,
    pub documentation: usize,
    pub license: usize,
    pub best_practices: usize,
    pub security: usize,
}

impl Score {
    /// Create a new score with all values set to zero.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Score {
            global: 0,
            documentation: 0,
            license: 0,
            best_practices: 0,
            security: 0,
        }
    }
}

/// Calculate score for the given report produced by the core linter for a
/// repository of kind primary.
pub(crate) fn calculate_score(report: &Report) -> Score {
    let mut score = Score::new();

    // Documentation
    if report.documentation.adopters {
        score.documentation += 5;
    }
    if report.documentation.code_of_conduct {
        score.documentation += 5;
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
    if report.documentation.website {
        score.documentation += 5;
    }

    // License
    if let Some(approved) = report.license.approved {
        if approved {
            score.license += 60;
        }
    }
    if report.license.fossa_badge {
        score.license += 20;
    }
    if report.license.spdx_id.is_some() {
        score.license += 20;
    }

    // Best practices
    if report.best_practices.artifacthub_badge {
        score.best_practices += 5;
    }
    if report.best_practices.community_meeting {
        score.best_practices += 25;
    }
    if report.best_practices.openssf_badge {
        score.best_practices += 60;
    }
    if report.best_practices.recent_release {
        score.best_practices += 10;
    }

    // Security
    if report.security.security_policy {
        score.security = 100;
    }

    // Global
    let global = (score.documentation as f64
        + score.license as f64
        + score.best_practices as f64
        + score.security as f64)
        / 4.0;
    score.global = global.round() as usize;

    score
}
