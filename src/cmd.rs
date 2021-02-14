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


pub struct XdgOpen {
}
impl Runner for XdgOpen {
    fn run(&self, open: &cli::OpenTarget) -> Result<()> {
        tracing::info!("xdg-open {}", &open.target);

        let mut cmd = Command::new("xdg-open");
        cmd.arg(&open.target);

        let output = cmd.spawn().map_err(|e| Error::CouldNotRun(e.to_string()))?;

        tracing::debug!("xdg-open output: {:?}", output);

        Ok(())
    }
}

pub fn get_system_runner() -> Box<dyn Runner> {
    Box::new(XdgOpen {})
}
