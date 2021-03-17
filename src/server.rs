//! The server handling open requests.

use crate::cmd;
use crate::{OpenTarget, PathTarget, Response, UrlTarget};

use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;
use structopt::StructOpt;

/// Configuration from the environment for the open-here server
#[derive(Debug, Clone, StructOpt)]
pub struct Config {
    /// Host and port which open-here server should listen on
    #[structopt(default_value = "127.0.0.1:9123")]
    pub host: String,

    /// If true will print the command instead of executing it
    #[structopt(short, long)]
    pub dry_run: bool,

    /// Max filesize for "/open/path" (default: 25MiB)
    #[structopt(short, long, default_value = "26214400")]
    pub max_filesize: usize,
}

fn open(target: OpenTarget, dry_run: bool) -> Response {
    let span = tracing::debug_span!("open", open = %format!("{:?}", target));
    let _guard = span.enter();

    let runner = cmd::Runner::from_system_runner();

    let res: Response = if dry_run {
        runner.dry_run(&target)
    } else {
        runner.run(&target)
    };

    if let Err(err) = &res {
        tracing::warn!("{}", err);
    }

    res
}

/// Handle GET /open/url by opening the target URL with the system runner
fn open_url(cfg: web::Data<Config>, json: web::Json<UrlTarget>) -> HttpResponse {
    let target = json.0;

    let res = open(OpenTarget::Url(target), cfg.dry_run);

    HttpResponse::Ok().json(res)
}

/// Handle GET /open/url by opening the target URL with the system runner
fn open_path(
    cfg: web::Data<Config>,
    query: web::Query<PathTarget>,
    content: web::Bytes,
) -> HttpResponse {
    let target = PathTarget {
        filename: query.0.filename,
        content: content.to_vec(),
    };

    let res = open(OpenTarget::Path(target), cfg.dry_run);

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

        let config = self.config.clone();
        actix_web::rt::System::new("main").block_on(async move {
            {
                HttpServer::new(move || {
                    App::new()
                        .route("/open/url", web::get().to(open_url))
                        .route("/open/path", web::get().to(open_path))
                        .data(config.clone())
                        .data(web::PayloadConfig::new(config.max_filesize))
                })
                .listen(self.listener)?
                .run()
                .await
            }
        })
    }
}
