use anyhow::{format_err, Error};

pub mod linter;
pub mod score;

/// Supported linters.
#[derive(Debug, PartialEq)]
pub enum Linter {
    Core = 0,
}

impl std::convert::TryFrom<i32> for Linter {
    type Error = Error;

    fn try_from(linter_id: i32) -> Result<Self, Self::Error> {
        match linter_id {
            0 => Ok(Linter::Core),
            _ => Err(format_err!("invalid linter id")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linter_from_i32() {
        assert!(matches!(Linter::try_from(0), Ok(Linter::Core)));
        assert!(matches!(Linter::try_from(1), Err(_)));
    }
}
