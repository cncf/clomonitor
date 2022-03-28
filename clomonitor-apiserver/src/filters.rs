/// Template filter that returns the rating letter corresponding to the score
/// value provided.
pub fn rating(score: &f64) -> ::askama::Result<char> {
    Ok(clomonitor_core::score::rating(*score))
}

/// Template filter that returns the rating letter corresponding to the score
/// value provided.
pub fn rating_opt(score: &Option<f64>) -> ::askama::Result<String> {
    Ok(match score {
        Some(v) => clomonitor_core::score::rating(*v).to_string(),
        None => "na".to_string(),
    })
}

/// Template filter that rounds the f64 value provided and returns its integer
/// part.
pub fn round(v: &f64) -> ::askama::Result<usize> {
    Ok(v.round() as usize)
}

/// Template filter that returns the width of the section score bar.
pub fn rs_section_score_width(score: &Option<f64>) -> ::askama::Result<f64> {
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

/// Template filter that returns the integer part of the rounded score value
/// provided as a string. "n/a" is returned when the value is none.
pub fn to_string(score: &Option<f64>) -> ::askama::Result<String> {
    Ok(match score {
        Some(v) => (v.round() as usize).to_string(),
        None => "n/a".to_string(),
    })
}
