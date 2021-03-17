//! Command to run.
use crate::{OpenTarget, PathTarget, UrlTarget};

use std::error;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Error {
    CouldNotRun(String),
    CreateDirectory(String),
    OpenFile(String),
    WriteFile(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CouldNotRun(msg) => write!(f, "Could not run: {}", msg),
            Error::CreateDirectory(msg) => write!(f, "Could not create directory: {}", msg),
            Error::OpenFile(msg) => write!(f, "Could open file: {}", msg),
            Error::WriteFile(msg) => write!(f, "Could write file: {}", msg),
        }
    }
}

impl error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

pub enum Opener {
    XdgOpen,
    Open,
    Start,
}

impl Opener {
    #[cfg(target_os = "linux")]
    fn get_system_opener() -> Opener {
        Opener::XdgOpen
    }
    #[cfg(target_os = "macos")]
    fn get_system_opener() -> Opener {
        Opener::Open
    }
    #[cfg(target_os = "windows")]
    fn get_system_opener() -> Opener {
        Opener::Start
    }

    fn cmd(&self, target: &str) -> OpenCommand {
        match self {
            Opener::XdgOpen => OpenCommand {
                program: String::from("xdg-open"),
                args: vec![target.to_string()],
            },
            Opener::Open => OpenCommand {
                program: String::from("open"),
                args: vec![target.to_string()],
            },
            Opener::Start => OpenCommand {
                program: String::from("cmd"),
                args: vec!["/c", "start", &target.to_string()]
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect(),
            },
        }
    }
}

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

pub struct Runner {
    opener: Opener,
}

impl Runner {
    pub fn from_system_runner() -> Runner {
        Self::new(Opener::get_system_opener())
    }

    pub fn new(opener: Opener) -> Runner {
        Runner { opener }
    }

    fn temp_dir(&self) -> std::path::PathBuf {
        std::env::temp_dir().join("open-here")
    }

    fn cmd(&self, target: &OpenTarget) -> Result<OpenCommand> {
        match target {
            OpenTarget::Url(UrlTarget { target }) => Ok(self.opener.cmd(target)),

            OpenTarget::Path(PathTarget { filename, content }) => {
                let dir = self.temp_dir();

                let file_path = if PathBuf::from(filename).is_absolute() {
                    dir.join(PathBuf::from(format!("./{}", filename)))
                } else {
                    dir.join(PathBuf::from(filename))
                };

                tracing::trace!("Writing file {}", &file_path.display());
                std::fs::create_dir_all(&file_path.parent().unwrap())
                    .map_err(|e| Error::CreateDirectory(e.to_string()))?;

                let mut file =
                    File::create(&file_path).map_err(|e| Error::OpenFile(e.to_string()))?;

                tracing::trace!("Writing file {}", file_path.display());

                file.write_all(content)
                    .map_err(|e| Error::WriteFile(e.to_string()))?;

                Ok(self.opener.cmd(&file_path.as_path().display().to_string()))
            }
        }
    }

    pub fn run(&self, target: &OpenTarget) -> Result<String> {
        let cmd = self.cmd(target)?;

        tracing::debug!("run: {}", &cmd);

        let mut cmd: Command = cmd.into();

        cmd.spawn().map_err(|e| Error::CouldNotRun(e.to_string()))?;

        Ok(String::from(""))
    }

    pub fn dry_run(&self, target: &OpenTarget) -> Result<String> {
        let cmd = self.cmd(target)?;

        tracing::debug!("dry_run: {}", &cmd);

        Ok(cmd.to_string())
    }
}
