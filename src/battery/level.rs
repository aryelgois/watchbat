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
        let levels = [&Unknown, &Critical, &Low, &Regular, &High, &Full];
        for level in levels {
            test_transition(level, level, None);
        }
    }

    /// Should give `None` when charging to `Low` or `Regular`.
    #[test]
    fn charging_transition() {
        let entries = [
            (&Unknown, &Critical, Some(BatteryStatus::Critical)),
            (&Critical, &Low, None),
            (&Low, &Regular, None),
            (&Regular, &High, Some(BatteryStatus::High)),
            (&High, &Full, Some(BatteryStatus::Full)),
        ];

        for (from, to, expected) in entries {
            test_transition(from, to, expected);
        }
    }

    /// Should give `None` when discharging to `High` or `Regular`.
    #[test]
    fn discharging_transition() {
        let entries = [
            (&Unknown, &Full, Some(BatteryStatus::Full)),
            (&Full, &High, None),
            (&High, &Regular, None),
            (&Regular, &Low, Some(BatteryStatus::Low)),
            (&Low, &Critical, Some(BatteryStatus::Critical)),
        ];

        for (from, to, expected) in entries {
            test_transition(from, to, expected);
        }
    }
}
