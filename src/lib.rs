pub mod cli;
pub mod client;
pub mod cmd;
pub mod server;

use env_logger::Env;
pub use structopt::StructOpt;

use std::fs;
use std::path::Path;
use std::{error, vec::Vec};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UrlTarget {
    pub target: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PathTarget {
    pub filename: String,
    #[serde(skip)] // is handled via body
    pub content: Vec<u8>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
/// Commands to be executed
pub enum OpenTarget {
    Url(UrlTarget),
    Path(PathTarget),
}

#[derive(Debug)]
pub enum Error {
    UnknownTarget(String),
    File(Box<dyn std::error::Error>),
}

impl error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnknownTarget(target) => {
                write!(f, "Unknown target to open: {}", target)
            }
            Error::File(e) => {
                write!(f, "File error: {}", e)
            }
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

impl OpenTarget {
    pub fn new(s: &str) -> Result<OpenTarget> {
        if s.starts_with("http://") || s.starts_with("https://") {
            Ok(OpenTarget::Url(UrlTarget {
                target: s.to_string(),
            }))
        } else if Path::new(s).exists() {
            Ok(OpenTarget::Path(PathTarget {
                filename: s.to_string(),
                content: fs::read(s).map_err(|e| Error::File(Box::new(e)))?,
            }))
        } else {
            Err(Error::UnknownTarget(s.to_string()))
        }
    }
}

impl std::fmt::Display for OpenTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenTarget::Url(UrlTarget { target }) => write!(f, "{}", target.clone()),
            OpenTarget::Path(PathTarget { filename, content }) => {
                write!(f, "{}, len: {}", filename.clone(), content.len())
            }
        }
    }
}

pub type Response = std::result::Result<String, cmd::Error>;

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
