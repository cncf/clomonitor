/// Template filter that returns the section score x-axis translate value for
/// the score bar.
pub fn rs_section_score_arrow_t_x(score: &usize) -> ::askama::Result<usize> {
    let v = score + 19;
    if v < 23 {
        return Ok(23);
    }
    Ok(v)
}

/// Template filter that returns the width of the section score bar.
pub fn rs_section_score_width(score: &usize) -> ::askama::Result<usize> {
    if *score < 100 {
        return Ok(score.saturating_sub(4));
    }
    Ok(*score)
}

/// Template filter that returns the rating letter corresponding to the score
/// value provided.
pub fn rating(score: &usize) -> ::askama::Result<char> {
    Ok(clomonitor_core::score::rating(*score))
}
