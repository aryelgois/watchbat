use std::fs;

use crate::ensure;
use crate::error::*;

/// The amount of charge in the battery.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Percentage(u8);

impl Percentage {
    const MAX: u8 = 100;

    pub fn new(val: u8) -> Result<Self> {
        ensure!(val <= Self::MAX, ValidationError::Max(Self::MAX));
        Ok(Self(val))
    }

    /// Gets the current battery charge from a system file.
    pub fn open_and_parse_file(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        Self::try_from(contents)
    }
}

impl TryFrom<String> for Percentage {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        let val: u8 = s.trim().parse()?;
        Self::new(val)
    }
}

#[cfg(test)]
mod tests {
    use super::Percentage;

    /// Should create new instance.
    #[test]
    fn new_ok() {
        let vals = [0, 1, 50, 99, 100];
        for val in vals {
            assert!(Percentage::new(val).is_ok());
        }
    }

    /// Should fail to create new instance.
    #[test]
    fn new_err() {
        let vals = [101, 127, 128, 255];
        for val in vals {
            assert!(Percentage::new(val).is_err());
        }
    }

    /// Should parse `String` into new instance.
    #[test]
    fn try_from_string_ok() {
        let vals = ["0", " 1", "50 ", " 99 ", "\t100\n"];
        for val in vals {
            assert!(Percentage::try_from(val.to_string()).is_ok());
        }
    }

    /// Should fail to parse `String` into new instance.
    #[test]
    fn try_from_string_err() {
        let vals = ["foo", "-0", "-1", "0xFF", "1.23"];
        for val in vals {
            assert!(Percentage::try_from(val.to_string()).is_err());
        }
    }
}
