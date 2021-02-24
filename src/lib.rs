pub mod cli;
pub mod client;
pub mod cmd;
pub mod server;

use env_logger::Env;
use envconfig::Envconfig;

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

pub fn serve(config: server::Config) -> Result<(), String> {
    let server = server::Server::new(config)?;

    let res = server.run();

    if let Err(e) = res {
        tracing::error!("{}", e);
        Err(e.to_string())
    } else {
        Ok(())
    }
}

pub fn open(config: client::Config, target: cli::OpenTarget) -> Result<String, String> {
    let client = client::OpenClient::new(format!("http://{}", config.host));

    client.open(&target).map_err(|e| e.to_string())
}

pub fn run(args: cli::Args) -> Result<(), String> {
    tracing::debug!("{:?}", args);

    match args.command {
        cli::Command::Server(config) => {
            tracing::debug!("serving");

            serve(config)
        }
        cli::Command::Open(target) => {
            tracing::debug!("{:?}", target);

            let config = client::Config::init_from_env().unwrap();

            open(config, target).map(|_| ())
        }
    }
}
