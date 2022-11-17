mod battery;
mod error;
mod utils;
mod validate;

use std::time::Duration;

use battery::percentage::Breakpoints;
use battery::watcher::{Config, Watcher};

impl Default for Config {
    fn default() -> Self {
        Config::new(
            String::from("/sys/class/power_supply/BAT0/capacity"),
            Breakpoints::new(10, 13, 94, 97).unwrap(),
            Duration::from_secs(45),
        )
        .unwrap()
    }
}

fn main() {
    let watcher = Watcher::new(Config::default());

    for notification in watcher.run() {
        notification.show().unwrap();
    }
}
