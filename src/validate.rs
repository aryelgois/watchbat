use std::fmt;

use crate::ensure;

/// Convenient wrapper around `std::Result`.
type ValidationResult = ::std::result::Result<(), ValidationError>;

/// An error when validating data.
#[derive(Debug)]
pub enum ValidationError {
    Max(u8),
    Order(u8, u8),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Max(m) => write!(f, "cannot be greater than {m}"),
            Self::Order(r, l) => write!(f, "right value ({r}) must be less than left value ({l})"),
        }
    }
}

impl std::error::Error for ValidationError {}

pub fn max(limit: u8, actual: u8) -> ValidationResult {
    ensure!(actual <= limit, ValidationError::Max(limit));
    Ok(())
}

#[cfg(test)]
mod max_tests {
    use super::max;

    #[test]
    fn is_ok() {
        let entries = [
            (0, 0),
            (1, 0),
            (10, 9),
            (10, 10),
            (100, 99),
            (100, 100),
            (u8::MAX, 0),
            (u8::MAX, u8::MAX - 1),
            (u8::MAX, u8::MAX),
        ];

        for (a, b) in entries {
            assert!(max(a, b).is_ok());
        }
    }

    #[test]
    fn is_err() {
        let entries = [
            (0, 1),
            (1, 2),
            (10, 20),
            (100, 200),
            (0, u8::MAX),
            (u8::MAX - 1, u8::MAX),
        ];

        for (a, b) in entries {
            assert!(max(a, b).is_err());
        }
    }
}

pub fn order(right: u8, left: u8) -> ValidationResult {
    ensure!(right < left, ValidationError::Order(right, left));
    Ok(())
}

#[cfg(test)]
mod order_tests {
    use super::order;

    #[test]
    fn is_ok() {
        let entries = [
            (0, 1),
            (1, 2),
            (10, 20),
            (100, 200),
            (0, u8::MAX),
            (u8::MAX - 1, u8::MAX),
        ];

        for (a, b) in entries {
            assert!(order(a, b).is_ok());
        }
    }

    #[test]
    fn is_err() {
        let entries = [
            (0, 0),
            (1, 0),
            (10, 9),
            (10, 10),
            (100, 99),
            (100, 100),
            (u8::MAX, 0),
            (u8::MAX, u8::MAX - 1),
            (u8::MAX, u8::MAX),
        ];

        for (a, b) in entries {
            assert!(order(a, b).is_err());
        }
    }
}
