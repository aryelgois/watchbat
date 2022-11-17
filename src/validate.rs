use std::fmt;
use std::time::Duration;

use crate::ensure;

/// Convenient wrapper around `std::Result`.
///
/// When `T` is specified, the validation function also transforms the input.
type ValidationResult<T = ()> = ::std::result::Result<T, ValidationError>;

/// An error when validating data.
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    GreaterThanZero,
    Max(u8),
    Order(u8, u8),
    Required(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::GreaterThanZero => write!(f, "must be greater than zero"),
            Self::Max(m) => write!(f, "cannot be greater than {m}"),
            Self::Order(l, r) => write!(f, "left value ({l}) must be less than right value ({r})"),
            Self::Required(name) => write!(f, "property '{name}' is required"),
        }
    }
}

impl std::error::Error for ValidationError {}

pub fn greater_than_zero<T>(val: T) -> ValidationResult
where
    T: HasZero + PartialOrd,
{
    ensure!(val > T::zero(), ValidationError::GreaterThanZero);
    Ok(())
}

/// Provides a zero value for comparison with `PartialOrd`.
pub trait HasZero {
    /// The zero value.
    fn zero() -> Self;
}

impl HasZero for Duration {
    fn zero() -> Self {
        Self::ZERO
    }
}

#[cfg(test)]
mod greater_than_zero_tests {
    use std::time::Duration;

    use super::greater_than_zero;

    #[test]
    fn is_ok() {
        let vals = [
            Duration::from_nanos(1),
            Duration::from_secs(1),
            Duration::MAX,
        ];

        for val in vals {
            assert!(greater_than_zero(val).is_ok());
        }
    }

    #[test]
    fn is_err() {
        let vals = [Duration::ZERO, Duration::from_secs(0)];
        for val in vals {
            assert!(greater_than_zero(val).is_err());
        }
    }
}

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

pub fn order(left: u8, right: u8) -> ValidationResult {
    ensure!(left < right, ValidationError::Order(left, right));
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

pub fn required(name: &str, val: String) -> ValidationResult<String> {
    let clean = val.trim();

    ensure!(
        !clean.is_empty(),
        ValidationError::Required(name.to_string())
    );

    Ok(clean.to_string())
}

#[cfg(test)]
mod required_tests {
    use super::required;

    #[test]
    fn is_ok() {
        let vals = ["foo", "foo ", "\tfoo", "foo\n"];
        let actual = Ok(String::from("foo"));

        for val in vals {
            let result = required("test_field", val.to_string());
            assert_eq!(result, actual);
        }
    }

    #[test]
    fn is_err() {
        let vals = ["", " ", "\t", "\n"];
        for val in vals {
            assert!(required("test_field", val.to_string()).is_err());
        }
    }
}
