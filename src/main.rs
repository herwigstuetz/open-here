use open_here::cli;
use open_here::client;
use open_here::server;

use env_logger::Env;
use structopt::StructOpt;

fn clamp(x: usize, min: usize, max: usize) -> usize {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn setup_logger(log_level: u8) {
    let log_levels = vec!["error", "warn", "info", "debug", "trace"];
    let level = clamp(log_level as usize, 0, log_levels.len() - 1);

    env_logger::Builder::from_env(Env::default().default_filter_or(log_levels[level])).init();
}

fn main() {
    let args = cli::Args::from_args();

    setup_logger(args.verbosity);

    tracing::debug!("{:?}", args);

    match args.command {
        cli::Command::Server => {
            tracing::debug!("serving");
            let res = server::serve();

            if let Err(e) = res {
                tracing::error!("{}", e);
            }
        }
        cli::Command::Open(target) => {
            tracing::debug!("{:?}", target);
            let res = client::open(target);

            if let Err(e) = res {
                tracing::error!("{}", e);
            }
        }
    }
}
