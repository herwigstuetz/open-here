//! Command to run.
use crate::cli;

use std::error::Error;
use std::fmt;
use std::process::Command;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum OpenError {
    CouldNotRun(String),
}

impl fmt::Display for OpenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenError::CouldNotRun(msg) => write!(f, "Could not run: {}", msg),
        }
    }
}

impl Error for OpenError {}

type Result<T> = std::result::Result<T, OpenError>;

pub trait Runner {
    fn run(&self, open: &cli::OpenTarget) -> Result<()>;
}

pub struct LinuxOpen {}
impl Runner for LinuxOpen {
    fn run(&self, open: &cli::OpenTarget) -> Result<()> {
        tracing::info!("xdg-open {}", &open.target);
        let mut cmd = Command::new("xdg-open");
        cmd.arg(&open.target);

        let output = cmd
            .spawn()
            .map_err(|e| OpenError::CouldNotRun(e.to_string()))?;

        tracing::debug!("xdg-open output: {:?}", output);

        Ok(())
    }
}

pub struct MacOSOpen {}
impl Runner for MacOSOpen {
    fn run(&self, open: &cli::OpenTarget) -> Result<()> {
        tracing::info!("open {}", &open.target);

        let mut cmd = Command::new("open");
        cmd.arg(&open.target);

        let output = cmd
            .spawn()
            .map_err(|e| OpenError::CouldNotRun(e.to_string()))?;

        tracing::debug!("open output: {:?}", output);

        Ok(())
    }
}

pub struct WindowsOpen {}
impl Runner for WindowsOpen {
    fn run(&self, open: &cli::OpenTarget) -> Result<()> {
        tracing::info!("start {}", &open.target);

        let mut cmd = Command::new("cmd");
        cmd.args(&["/c", "start", &open.target]);

        let output = cmd
            .spawn()
            .map_err(|e| OpenError::CouldNotRun(e.to_string()))?;

        tracing::debug!("start output: {:?}", output);

        Ok(())
    }
}

#[cfg(target_os = "linux")]
pub fn get_system_runner() -> Box<dyn Runner> {
    Box::new(LinuxOpen {})
}

#[cfg(target_os = "macos")]
pub fn get_system_runner() -> Box<dyn Runner> {
    Box::new(MacOSOpen {})
}

#[cfg(target_os = "windows")]
pub fn get_system_runner() -> Box<dyn Runner> {
    Box::new(WindowsOpen {})
}
