//! CLI for open-here.

use crate::server;

pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "open-here", about = "xdg-open/open/start via ssh.")]
pub struct Args {
    // Verbosity
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: u8,

    /// Sub-command to execute
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
/// Commands that open-here can execute
pub enum Command {
    /// Start open-here server
    #[structopt(name = "server")]
    Server(server::Config),

    /// Open target
    #[structopt(name = "open")]
    Open {target: String},
}
