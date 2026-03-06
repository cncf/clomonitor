#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::inline_always,
    clippy::ref_option,
    clippy::trivially_copy_pass_by_ref,
    clippy::unnecessary_wraps,
    clippy::unused_self
)]

/// Template filter that returns the rating letter corresponding to the score
/// value provided.
#[askama::filter_fn]
pub(crate) fn rating(score: &f64, _: &dyn askama::Values) -> askama::Result<char> {
    Ok(clomonitor_core::score::rating(*score))
}

/// Template filter that returns the rating letter corresponding to the score
/// value provided.
#[askama::filter_fn]
pub(crate) fn rating_opt(score: &Option<f64>, _: &dyn askama::Values) -> askama::Result<String> {
    Ok(match score {
        Some(v) => clomonitor_core::score::rating(*v).to_string(),
        None => "na".to_string(),
    })
}

/// Template filter that rounds the f64 value provided and returns its integer
/// part.
#[askama::filter_fn]
pub(crate) fn round(v: &f64, _: &dyn askama::Values) -> askama::Result<usize> {
    Ok(v.round() as usize)
}

/// Template filter that returns the width of the section score bar.
#[askama::filter_fn]
pub(crate) fn rs_section_score_width(
    score: &Option<f64>,
    _: &dyn askama::Values,
) -> askama::Result<f64> {
    Ok(match score {
        Some(v) => {
            let width = (v * 1.06).round();
            if width < 2.0 { 2.0 } else { width }
        }
        None => 0.0,
    })
}

/// Template filter that return the stroke-dasharray for the global score.
#[askama::filter_fn]
pub(crate) fn stroke(v: &f64, _: &dyn askama::Values) -> askama::Result<f64> {
    Ok(251.42 + (251.42 * v / 100.0))
}

/// Template filter that returns the integer part of the rounded score value
/// provided as a string. "n/a" is returned when the value is none.
#[askama::filter_fn]
pub(crate) fn to_string(score: &Option<f64>, _: &dyn askama::Values) -> askama::Result<String> {
    Ok(match score {
        Some(v) => (v.round() as usize).to_string(),
        None => "n/a".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use askama::NO_VALUES;

    use super::*;

    #[test]
    fn rating_works() {
        assert_eq!(rating::default().execute(&80.0, NO_VALUES).unwrap(), 'a');
        assert_eq!(rating::default().execute(&74.0, NO_VALUES).unwrap(), 'b');
    }

    #[test]
    fn rating_opt_some() {
        assert_eq!(
            rating_opt::default()
                .execute(&Some(80.0), NO_VALUES)
                .unwrap(),
            "a".to_string()
        );
        assert_eq!(
            rating_opt::default()
                .execute(&Some(74.0), NO_VALUES)
                .unwrap(),
            "b".to_string()
        );
    }

    #[test]
    fn rating_opt_none() {
        assert_eq!(
            rating_opt::default().execute(&None, NO_VALUES).unwrap(),
            "na".to_string()
        );
    }

    #[test]
    fn round_works() {
        assert_eq!(round::default().execute(&7.9, NO_VALUES).unwrap(), 8);
    }

    #[test]
    fn rs_section_score_width_some() {
        assert!(
            (rs_section_score_width::default()
                .execute(&Some(1.0), NO_VALUES)
                .unwrap()
                - 2.0)
                .abs()
                < f64::EPSILON
        );
        assert!(
            (rs_section_score_width::default()
                .execute(&Some(80.0), NO_VALUES)
                .unwrap()
                - 85.0)
                .abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn rs_section_score_width_none() {
        assert!(
            (rs_section_score_width::default()
                .execute(&None, NO_VALUES)
                .unwrap()
                - 0.0)
                .abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn to_string_some() {
        assert_eq!(
            to_string::default()
                .execute(&Some(79.9), NO_VALUES)
                .unwrap(),
            "80".to_string()
        );
    }

    #[test]
    fn to_string_none() {
        assert_eq!(
            to_string::default().execute(&None, NO_VALUES).unwrap(),
            "n/a".to_string()
        );
    }
}
