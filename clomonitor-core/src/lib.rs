pub mod linter;
pub mod score;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// The linter id provided is not valid.
    #[error("invalid linter id")]
    InvalidLinterId,
}

/// Supported linters.
#[derive(Debug)]
pub enum Linter {
    Core = 0,
}

impl std::convert::TryFrom<i32> for Linter {
    type Error = Error;

    fn try_from(linter_id: i32) -> Result<Self, Self::Error> {
        match linter_id {
            0 => Ok(Linter::Core),
            _ => Err(Error::InvalidLinterId),
        }
    }
}
