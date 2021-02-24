//! The server handling open requests.

use crate::cli;
use crate::cmd;

use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;
use structopt::StructOpt;

pub type Response = Result<String, cmd::OpenError>;

/// Configuration from the environment for the open-here server
#[derive(Debug, Clone, StructOpt)]
pub struct Config {
    /// Host and port which open-here server should listen on
    #[structopt(default_value = "127.0.0.1:9123")]
    pub host: String,

    /// If true will print the command instead of executing it
    #[structopt(short, long)]
    pub dry_run: bool,
}

/// Handle GET /open by opening the target with the system runner
fn open(cfg: web::Data<Config>, form: web::Query<cli::OpenTarget>) -> HttpResponse {
    // TODO: More consistent logging/tracing/spanning
    let span = tracing::debug_span!("open", target = %format!("{:?}", form.target));
    let _guard = span.enter();

    let open = cli::OpenTarget {
        target: form.target.to_string(),
    };

    let res: Response = if cfg.dry_run {
        cmd::get_system_runner().dry_run(&open)
    } else {
        cmd::get_system_runner().run(&open)
    };

    if let Err(err) = &res {
        tracing::warn!("{}", err);
    }

    HttpResponse::Ok().json(res)
}

pub struct Server {
    config: Config,
    listener: TcpListener,
}

impl Server {
    // Create new open-here server
    pub fn new(config: Config) -> Result<Server, String> {
        let listener = TcpListener::bind(&config.host).map_err(|err| err.to_string())?;

        Ok(Server { config, listener })
    }

    pub fn get_port(&self) -> Result<u16, String> {
        Ok(self
            .listener
            .local_addr()
            .map_err(|err| err.to_string())?
            .port())
    }

    /// Start open-here server
    pub fn run(self) -> std::io::Result<()> {
        tracing::info!(
            "Running open-here server on port {}",
            self.get_port().unwrap()
        );

        let config_ = self.config.clone();
        actix_web::rt::System::new("main").block_on(async move {
            {
                HttpServer::new(move || {
                    App::new()
                        .route("/open", web::get().to(open))
                        .data(config_.clone())
                })
                .listen(self.listener)?
                .run()
                .await
            }
        })
    }
}
