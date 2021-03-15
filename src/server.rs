//! The server handling open requests.

use crate::cmd;
use crate::{OpenTarget, UrlTarget, PathTarget, Response};

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

/// Handle GET /open/url by opening the target URL with the system runner
fn open_url(cfg: web::Data<Config>, form: web::Json<UrlTarget>) -> HttpResponse {
    let open = form.0;

    // TODO: More consistent logging/tracing/spanning
    let span = tracing::debug_span!("open/url", open = %format!("{:?}", open));
    let _guard = span.enter();

    let res: Response = if cfg.dry_run {
        cmd::get_system_runner().dry_run(&OpenTarget::Url(open))
    } else {
        cmd::get_system_runner().run(&OpenTarget::Url(open))
    };

    if let Err(err) = &res {
        tracing::warn!("{}", err);
    }

    HttpResponse::Ok().json(res)
}

/// Handle GET /open/url by opening the target URL with the system runner
//fn open_path(cfg: web::Data<Config>, target: web::Form<PathTarget>, content: web::Bytes) -> HttpResponse {
fn open_path(cfg: web::Data<Config>, target: web::Query<PathTarget>, content: web::Bytes) -> HttpResponse {
//fn open_path(cfg: web::Data<Config>) -> HttpResponse {
    let open = PathTarget {
        filename: target.0.filename,
        content: content.to_vec(),
    };

    tracing::debug!("vec: {}", open.content.len());

    // TODO: More consistent logging/tracing/spanning
    let span = tracing::debug_span!("open/path", open = %format!("{:?}", open));
    let _guard = span.enter();

    let res: Response = if cfg.dry_run {
        cmd::get_system_runner().dry_run(&OpenTarget::Path(open))
    } else {
        cmd::get_system_runner().run(&OpenTarget::Path(open))
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
