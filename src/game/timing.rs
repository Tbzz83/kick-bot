use std::time::{Duration, Instant};
use rand::Rng;

pub const WAIT_TIME_MILLIS: u128 = 1000;
pub const SLEEP_SECONDS: u64 = 1;

pub fn random_wait_time() -> u128 {
    rand::rng().random_range(0..WAIT_TIME_MILLIS)
}

pub fn random_target() -> u16 {
    rand::rng().random_range(0..3)
}

pub fn wait_for(duration_ms: u128) {
    let start = Instant::now();
    while start.elapsed().as_millis() < duration_ms {}
}

pub fn sleep_short() {
    std::thread::sleep(Duration::from_secs(SLEEP_SECONDS));
}
