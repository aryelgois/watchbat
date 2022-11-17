use std::iter;
use std::thread;
use std::time::Duration;

/// An infinite iterator that delays by `interval`.
///
/// If `immediate` is `true`, the first step does not delay.
pub fn on_interval(interval: Duration, immediate: bool) -> impl iter::Iterator<Item = ()> {
    let mut skip = immediate;

    iter::repeat_with(move || {
        if skip {
            skip = false;
        } else {
            thread::sleep(interval);
        }
    })
}
