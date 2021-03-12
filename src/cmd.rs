//! Command to run.
use crate::OpenTarget;

use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::fmt;
use std::process::Command;

use tempfile::tempdir;


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

pub struct OpenCommand {
    program: String,
    /// TODO: use Vec<&str>
    args: std::vec::Vec<String>,
}

impl fmt::Display for OpenCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.program,
            self.args
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Into<Command> for OpenCommand {
    fn into(self) -> Command {
        let mut cmd = Command::new(self.program);
        cmd.args(self.args);

        cmd
    }
}

pub trait Runner {
    fn cmd(&self, open: &str) -> Result<OpenCommand>;

    fn run(&self, open: &OpenTarget) -> Result<String> {
        let span = tracing::debug_span!("run", open = %format!("{:?}", open));
        let _guard = span.enter();

        match open {
            OpenTarget::Url { target } => {
                let mut cmd: Command = self.cmd(&target)?.into();

                cmd.spawn()
                    .map_err(|e| OpenError::CouldNotRun(e.to_string()))?;

                Ok(String::from(""))
            },

            OpenTarget::Path { filename, content } => {
                let span = tracing::debug_span!("run path");
                let _guard = span.enter();

                let dir = std::env::temp_dir().join("open-here");

                std::fs::create_dir_all(&dir).map_err(|e| OpenError::CouldNotRun(e.to_string()))?;

                let file_path = dir.join(filename);

                let mut file = File::create(&file_path).map_err(|e| OpenError::CouldNotRun(e.to_string()))?;

                tracing::debug!("file: {}", &file_path.as_path().display());

                file.write_all(content).map_err(|e| OpenError::CouldNotRun(e.to_string()))?;

                let mut cmd: Command =
                    self.cmd(&file_path
                             .as_path().display().to_string())?.into();

                cmd.spawn()
                    .map_err(|e| OpenError::CouldNotRun(e.to_string()))?;

                Ok(String::from(""))
            }
        }
    }

    fn dry_run(&self, open: &OpenTarget) -> Result<String> {
        let span = tracing::debug_span!("run", open = %format!("{:?}", open));
        let _guard = span.enter();

        match open {
            OpenTarget::Url { target } => {
                let cmd = self.cmd(&target)?;

                let res = format!("Would run: {}", cmd);
                tracing::info!("{}", &res);

                Ok(res)
            },

            OpenTarget::Path { filename, content } => {
                Ok(String::from("")) // TODO
            }
        }
    }
}

pub struct LinuxOpen {}
impl Runner for LinuxOpen {
    fn cmd(&self, open: &str) -> Result<OpenCommand> {
        Ok(OpenCommand {
            program: String::from("xdg-open"),
            args: vec![open.to_string()],
        })
    }
}

pub struct MacOSOpen {}
impl Runner for MacOSOpen {
    fn cmd(&self, open: &str) -> Result<OpenCommand> {
        Ok(OpenCommand {
            program: String::from("open"),
            args: vec![open.to_string()],
        })
    }
}

pub struct WindowsOpen {}
impl Runner for WindowsOpen {
    fn cmd(&self, open: &str) -> Result<OpenCommand> {
        Ok(OpenCommand {
            program: String::from("cmd"),
            args: vec!["/c", "start", &open.to_string()]
                .iter()
                .map(|arg| arg.to_string())
                .collect(),
        })
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
