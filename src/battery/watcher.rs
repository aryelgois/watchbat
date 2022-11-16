use std::time::Duration;

use crate::error::Result;

use super::level::BatteryLevel;
use super::percentage::{Breakpoints, Percentage};
use super::status::BatteryStatus;

/// Contains settings for `Watcher`.
#[derive(Debug)]
pub struct Config {
    battery_file: String,
    breakpoints: Breakpoints,
    interval: Duration,
}

impl Config {
    pub fn new(battery_file: String, breakpoints: Breakpoints, interval: Duration) -> Result<Self> {
        Ok(Self {
            battery_file,
            breakpoints,
            interval,
        })
    }

    /// Gets the current battery charge.
    pub fn read_percentage(&self) -> Result<Percentage> {
        Percentage::open_and_parse_file(&self.battery_file)
    }
}

/// Keeps track of the `BatteryLevel`.
#[derive(Debug)]
pub struct Watcher {
    config: Config,
    state: BatteryLevel,
}

impl Watcher {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            state: BatteryLevel::default(),
        }
    }

    /// Gets a new `BatteryLevel` to update the internal state and produce a `BatteryStatus`.
    fn update(&mut self) -> Result<Option<BatteryStatus>> {
        match self.config.read_percentage() {
            Ok(percentage) => {
                let level = self.config.breakpoints.get_level(&percentage);
                let status = self.state.transition(&level);

                self.state = level;
                Ok(status)
            }
            Err(e) => {
                self.state = BatteryLevel::Unknown;
                Err(e)
            }
        }
    }
}
