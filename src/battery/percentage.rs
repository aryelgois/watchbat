use std::{fmt, fs};

use crate::error::{Error, Result};
use crate::validate;

use super::level::BatteryLevel;

/// The amount of charge in the battery.
#[derive(PartialEq, PartialOrd)]
pub struct Percentage(u8);

impl Percentage {
    const MAX: u8 = 100;

    pub fn new(val: u8) -> Result<Self> {
        validate::max(Self::MAX, val)?;
        Ok(Self(val))
    }

    /// Gets the current battery charge from a system file.
    pub fn open_and_parse_file(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        Self::try_from(contents)
    }
}

impl fmt::Debug for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}%", self.0)
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

/// Groups `Percentage` marks to select a `BatteryLevel`.
#[derive(Debug)]
pub struct Breakpoints {
    critical: Percentage,
    low: Percentage,
    high: Percentage,
    full: Percentage,
}

impl Breakpoints {
    pub fn new(critical: u8, low: u8, high: u8, full: u8) -> Result<Self> {
        validate::order(critical, low)?;
        validate::order(low, high)?;
        validate::order(high, full)?;

        Ok(Self {
            critical: Percentage::new(critical)?,
            low: Percentage::new(low)?,
            high: Percentage::new(high)?,
            full: Percentage::new(full)?,
        })
    }

    /// Selects a `BatteryLevel` from a `Percentage`.
    pub fn get_level(&self, percentage: &Percentage) -> BatteryLevel {
        if percentage <= &self.critical {
            BatteryLevel::Critical
        } else if percentage <= &self.low {
            BatteryLevel::Low
        } else if percentage < &self.high {
            BatteryLevel::Regular
        } else if percentage < &self.full {
            BatteryLevel::High
        } else {
            BatteryLevel::Full
        }
    }
}

#[cfg(test)]
mod breakpoints_tests {
    use super::BatteryLevel;
    use super::Breakpoints;
    use super::Percentage;

    /// Should create new instance.
    #[test]
    fn new_ok() {
        let entries = [
            (0, 1, 2, 3),
            (0, 1, 99, 100),
            (97, 98, 99, 100),
            (10, 15, 90, 95),
        ];

        for p in entries {
            assert!(Breakpoints::new(p.0, p.1, p.2, p.3).is_ok());
        }
    }

    /// Should fail to create new instance.
    #[test]
    fn new_err() {
        let entries = [
            (0, 0, 0, 0),
            (1, 1, 3, 4),
            (2, 1, 3, 4),
            (1, 3, 2, 4),
            (1, 2, 4, 3),
            (100, 100, 100, 100),
            (101, 102, 103, 104),
        ];

        for p in entries {
            assert!(Breakpoints::new(p.0, p.1, p.2, p.3).is_err());
        }
    }

    /// Should select the correct `BatteryLevel`.
    #[test]
    fn get_level() {
        // The gap between `low` and `high` is expected, and
        // it is useful for aligning with the index in `levels`
        let breakpoints = Breakpoints::new(0, 1, 3, 4).unwrap();

        let levels = [
            BatteryLevel::Critical,
            BatteryLevel::Low,
            BatteryLevel::Regular,
            BatteryLevel::High,
            BatteryLevel::Full,
        ];

        for (val, expected) in levels.iter().enumerate() {
            let percentage = Percentage::new(val as u8).unwrap();
            let actual = breakpoints.get_level(&percentage);
            assert_eq!(&actual, expected);
        }
    }
}
