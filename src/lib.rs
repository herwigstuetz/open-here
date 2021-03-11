pub mod cli;
pub mod client;
pub mod cmd;
pub mod server;

use env_logger::Env;
pub use structopt::StructOpt;

use std::fs;
use std::path::Path;
use std::vec::Vec;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
/// Commands to be executed
pub enum OpenTarget {
    Url { target: String },
    Path {
        filename: String,
        content: Vec<u8>,
    },
}

impl OpenTarget {
    pub fn new(s: &str) -> Option<OpenTarget> {
        if s.starts_with("http://") || s.starts_with("https://") {
            Some(
                OpenTarget::Url { target: s.to_string() }
            )
        } else if Path::new(s).exists() {
            Some (
                OpenTarget::Path {
                    filename: s.to_string(),
                    content: fs::read(s).ok()?,
                }
            )
        } else {
            None
        }
    }
}

impl std::fmt::Display for OpenTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenTarget::Url { target } => write!(f, "{}", target.clone()),
            OpenTarget::Path { filename, .. } => write!(f, "{}", filename.clone()),
        }
    }
}

pub type Response = Result<String, cmd::OpenError>;


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
