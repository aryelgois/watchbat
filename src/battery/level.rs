use super::status::BatteryStatus;

/// Describes roughly the amount of charge in a battery.
#[derive(Debug, PartialEq)]
pub enum BatteryLevel {
    Unknown,
    Critical,
    Low,
    Regular,
    High,
    Full,
}

impl Default for BatteryLevel {
    fn default() -> Self {
        Self::Unknown
    }
}

impl BatteryLevel {
    /// Checks the state transition from one `BatteryLevel` to another.
    ///
    /// Since the real battery is the source of truth about its charge,
    /// it is **allowed** to transition from and to any `BatteryLevel`,
    /// and this function just gives a possible `BatteryStatus` to
    /// describe the transition.
    pub fn transition(&self, to: &Self) -> Option<BatteryStatus> {
        if self == to {
            return None;
        }

        match (self, to) {
            (_, Self::Unknown) => Some(BatteryStatus::Unknown),

            (_, Self::Critical) => Some(BatteryStatus::Critical),

            (Self::Critical, Self::Low) => None,
            (_, Self::Low) => Some(BatteryStatus::Low),

            (Self::Full, Self::High) => None,
            (_, Self::High) => Some(BatteryStatus::High),

            (_, Self::Full) => Some(BatteryStatus::Full),

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use BatteryLevel::*;

    fn test_transition(from: &BatteryLevel, to: &BatteryLevel, expected: Option<BatteryStatus>) {
        assert_eq!(from.transition(to), expected);
    }

    /// Should always give `None`.
    #[test]
    fn self_transition() {
        fn run(level: &BatteryLevel) {
            test_transition(level, level, None);
        }

        run(&Unknown);
        run(&Critical);
        run(&Low);
        run(&Regular);
        run(&High);
        run(&Full);
    }

    /// Should give `None` when charging to `Low` or `Regular`.
    #[test]
    fn charging_transition() {
        test_transition(&Unknown, &Critical, Some(BatteryStatus::Critical));
        test_transition(&Critical, &Low, None);
        test_transition(&Low, &Regular, None);
        test_transition(&Regular, &High, Some(BatteryStatus::High));
        test_transition(&High, &Full, Some(BatteryStatus::Full));
    }

    /// Should give `None` when discharging to `High` or `Regular`.
    #[test]
    fn discharging_transition() {
        test_transition(&Unknown, &Full, Some(BatteryStatus::Full));
        test_transition(&Full, &High, None);
        test_transition(&High, &Regular, None);
        test_transition(&Regular, &Low, Some(BatteryStatus::Low));
        test_transition(&Low, &Critical, Some(BatteryStatus::Critical));
    }
}
