use open_here::cli;
use open_here::client;
use open_here::server;
use open_here::setup_logger;
use open_here::OpenTarget;

use envconfig::Envconfig;
use structopt::StructOpt;

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

pub fn open(config: client::Config, target: OpenTarget) -> Result<String, String> {
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
        cli::Command::Open { target } => {
            tracing::debug!("{:?}", target);

            let config = client::Config::init_from_env().unwrap();
            let target = OpenTarget::new(&target).ok_or_else(|| "Could not create OpenTarget".to_string())?;

            tracing::debug!("run: new: {:}", target);

            open(config, target).map(|_| ())
        }
    }
}

fn main() {
    let args = cli::Args::from_args();

    setup_logger(args.verbosity);

    if let Err(err) = run(args) {
        tracing::error!("{}", err);
        std::process::exit(1);
    }
}
