use open_here::cli;
use open_here::{run, setup_logger};

pub use structopt::StructOpt;

fn main() -> Result<(), ()> {
    let args = cli::Args::from_args();

    setup_logger(args.verbosity);

    run(args).map_err(|_| ())
}
