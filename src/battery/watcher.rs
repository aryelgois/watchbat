use std::iter;
use std::time::Duration;

use notify_rust::Notification;

use crate::error::Result;
use crate::utils;
use crate::validate;

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
        let battery_file = validate::required("battery_file", battery_file)?;
        validate::greater_than_zero(interval)?;

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
        eprintln!("config = {:#?}", config);

        Self {
            config,
            state: BatteryLevel::default(),
        }
    }

    /// Starts the `Watcher`.
    ///
    /// It uses the `Watcher` as an `Iterator` that yields `Notification`
    /// when the internal state changes.
    pub fn run(self) -> impl iter::Iterator<Item = Notification> {
        utils::on_interval(self.config.interval, true)
            .zip(self.into_iter())
            .filter_map(|(_, n)| n)
    }

    /// Gets a new `BatteryLevel` to update the internal state and produce a `BatteryStatus`.
    fn update(&mut self) -> Result<Option<BatteryStatus>> {
        match self.config.read_percentage() {
            Ok(percentage) => {
                let level = self.config.breakpoints.get_level(&percentage);
                let status = self.state.transition(&level);

                eprintln!(
                    "percentage: {:?}, level: {:?}, status: {:?}",
                    percentage, level, status
                );

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

impl iter::Iterator for Watcher {
    type Item = Option<Notification>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.update() {
            Ok(status) => status.and_then(|s| Some(s.into())),
            Err(e) => Some(e.into()),
        })
    }
}
