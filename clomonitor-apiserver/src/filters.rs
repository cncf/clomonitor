/// Template filter that returns the rating letter corresponding to the score
/// value provided.
#[allow(clippy::unnecessary_wraps, clippy::trivially_copy_pass_by_ref)]
pub(crate) fn rating(score: &f64) -> askama::Result<char> {
    Ok(clomonitor_core::score::rating(*score))
}

/// Template filter that returns the rating letter corresponding to the score
/// value provided.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn rating_opt(score: &Option<f64>) -> askama::Result<String> {
    Ok(match score {
        Some(v) => clomonitor_core::score::rating(*v).to_string(),
        None => "na".to_string(),
    })
}

/// Template filter that rounds the f64 value provided and returns its integer
/// part.
#[allow(
    clippy::unnecessary_wraps,
    clippy::trivially_copy_pass_by_ref,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub(crate) fn round(v: &f64) -> askama::Result<usize> {
    Ok(v.round() as usize)
}

/// Template filter that returns the width of the section score bar.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn rs_section_score_width(score: &Option<f64>) -> askama::Result<f64> {
    Ok(match score {
        Some(v) => {
            let width = (v * 1.06).round();
            if width < 2.0 {
                2.0
            } else {
                width
            }
        }
        None => 0.0,
    })
}

/// Template filter that return the stroke-dasharray for the global score.
#[allow(clippy::unnecessary_wraps, clippy::trivially_copy_pass_by_ref)]
pub(crate) fn stroke(v: &f64) -> askama::Result<f64> {
    Ok(251.42 + (251.42 * v / 100.0))
}

/// Template filter that returns the integer part of the rounded score value
/// provided as a string. "n/a" is returned when the value is none.
#[allow(
    clippy::unnecessary_wraps,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]
pub(crate) fn to_string(score: &Option<f64>) -> askama::Result<String> {
    Ok(match score {
        Some(v) => (v.round() as usize).to_string(),
        None => "n/a".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating_works() {
        assert_eq!(rating(&80.0).unwrap(), 'a');
        assert_eq!(rating(&74.0).unwrap(), 'b');
    }

    #[test]
    fn rating_opt_some() {
        assert_eq!(rating_opt(&Some(80.0)).unwrap(), "a".to_string());
        assert_eq!(rating_opt(&Some(74.0)).unwrap(), "b".to_string());
    }

    #[test]
    fn rating_opt_none() {
        assert_eq!(rating_opt(&None).unwrap(), "na".to_string());
    }

    #[test]
    fn round_works() {
        assert_eq!(round(&7.9).unwrap(), 8);
    }

    #[test]
    fn rs_section_score_width_some() {
        assert!((rs_section_score_width(&Some(1.0)).unwrap() - 2.0).abs() < f64::EPSILON);
        assert!((rs_section_score_width(&Some(80.0)).unwrap() - 85.0).abs() < f64::EPSILON);
    }

    #[test]
    fn rs_section_score_width_none() {
        assert!((rs_section_score_width(&None).unwrap() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn to_string_some() {
        assert_eq!(to_string(&Some(79.9)).unwrap(), "80".to_string());
    }

    #[test]
    fn to_string_none() {
        assert_eq!(to_string(&None).unwrap(), "n/a".to_string());
    }
}
