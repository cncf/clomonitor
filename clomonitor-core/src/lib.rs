#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::doc_markdown, clippy::wildcard_imports)]

#[allow(clippy::module_name_repetitions)]
pub mod linter;

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub mod score;
