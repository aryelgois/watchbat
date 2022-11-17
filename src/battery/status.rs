use std::fmt;

use notify_rust::{Notification, Urgency};

/// Each `BatteryStatus` produces a `Notification`.
#[derive(Debug, PartialEq)]
pub enum BatteryStatus {
    Unknown(Option<String>),
    Critical,
    Low,
    High,
    Full,
}

impl fmt::Display for BatteryStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown(_) => write!(f, "Battery is Unknown"),
            Self::Critical => write!(f, "Battery is Critical"),
            Self::Low => write!(f, "Battery is almost Empty"),
            Self::High => write!(f, "Battery is almost Full"),
            Self::Full => write!(f, "Battery is Full"),
        }
    }
}

impl From<BatteryStatus> for Notification {
    fn from(status: BatteryStatus) -> Self {
        const TIMEOUT: i32 = 5000;

        let mut notification = Self::new();

        notification.summary(&status.to_string());

        match status {
            BatteryStatus::Unknown(Some(e)) => {
                notification
                    .body(e.as_str())
                    .urgency(Urgency::Critical)
                    .timeout(0);
            }
            BatteryStatus::Unknown(None) | BatteryStatus::Critical => {
                notification.urgency(Urgency::Critical).timeout(0);
            }
            BatteryStatus::Low | BatteryStatus::Full => {
                notification.urgency(Urgency::Critical).timeout(TIMEOUT);
            }
            _ => {}
        }

        notification
    }
}
