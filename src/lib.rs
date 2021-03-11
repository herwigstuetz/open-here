pub mod cli;
pub mod client;
pub mod cmd;
pub mod server;

use env_logger::Env;

fn clamp(x: usize, min: usize, max: usize) -> usize {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn setup_logger(log_level: u8) {
    let log_levels = vec!["error", "warn", "info", "debug", "trace"];
    let level = clamp(log_level as usize, 0, log_levels.len() - 1);

    env_logger::Builder::from_env(Env::default().default_filter_or(log_levels[level])).init();
}
