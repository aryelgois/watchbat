use std::{fmt, io, num};

use notify_rust::{Notification, Urgency};

use crate::validate::ValidationError;

/// Convenient wrapper around `std::Result`.
pub type Result<T> = ::std::result::Result<T, Error>;

/// A type of error.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse(num::ParseIntError),
    Validation(ValidationError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O Error: {e}"),
            Self::Parse(e) => write!(f, "Parsing Error: {e}"),
            Self::Validation(e) => write!(f, "Validation Error: {e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Self::Parse(e)
    }
}

impl From<ValidationError> for Error {
    fn from(e: ValidationError) -> Self {
        Self::Validation(e)
    }
}

impl From<Error> for Notification {
    fn from(e: Error) -> Self {
        let mut notification = Self::new();

        notification
            .summary("watchbat Error")
            .body(e.to_string().as_str())
            .urgency(Urgency::Critical)
            .timeout(0);

        notification
    }
}

/// Exits a function early with an `Error`.
#[macro_export]
macro_rules! bail {
    ($e:expr) => {
        return Err($e.into());
    };
    ($fmt:expr, $($arg:tt)+) => {
        return Err(format!($fmt, $($arg)+).into());
    };
}

/// Exits a function early with an `Error` if the condition is not satisfied.
///
/// Similar to `assert!`, `ensure!` takes a condition and exits the function
/// if the condition fails. Unlike `assert!`, `ensure!` returns an `Error`,
/// it does not panic.
#[macro_export(local_inner_macros)]
macro_rules! ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) {
            bail!($e);
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !($cond) {
            bail!($fmt, $($arg)*);
        }
    };
}
