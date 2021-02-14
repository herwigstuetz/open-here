//! Command to run.
use crate::cli;

use std::process::{Command, ExitStatus};

#[derive(Debug)]
pub enum Error {
    Exit(ExitStatus),
    CouldNotRun(String),
}

type Result<T> = std::result::Result<T, Error>;


pub trait Runner {
    fn run(&self, open: &cli::OpenTarget) -> Result<()>;
}


pub struct LinuxOpen {
}
impl Runner for LinuxOpen {
    fn run(&self, open: &cli::OpenTarget) -> Result<()> {
        tracing::info!("xdg-open {}", &open.target);

        let mut cmd = Command::new("xdg-open");
        cmd.arg(&open.target);

        let output = cmd.spawn().map_err(|e| Error::CouldNotRun(e.to_string()))?;

        tracing::debug!("xdg-open output: {:?}", output);

        Ok(())
    }
}

pub struct MacOSOpen {
}
impl Runner for MacOSOpen {
    fn run(&self, open: &cli::OpenTarget) -> Result<()> {
        tracing::info!("open {}", &open.target);

        let mut cmd = Command::new("open");
        cmd.arg(&open.target);

        let output = cmd.spawn().map_err(|e| Error::CouldNotRun(e.to_string()))?;

        tracing::debug!("open output: {:?}", output);

        Ok(())
    }
}

pub struct WindowsOpen {
}
impl Runner for WindowsOpen {
    fn run(&self, open: &cli::OpenTarget) -> Result<()> {
        tracing::info!("start {}", &open.target);

        let mut cmd = Command::new("start");
        cmd.arg(&open.target);

        let output = cmd.spawn().map_err(|e| Error::CouldNotRun(e.to_string()))?;

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
