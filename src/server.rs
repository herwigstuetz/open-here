//! The server handling open requests.

use crate::cli;
use crate::cmd;

use actix_web::{web, App, HttpResponse, HttpServer};
use envconfig::Envconfig;

/// Configuration from the environment for the open-here server
#[derive(Envconfig)]
struct Config {
    /// Host and port which open-here server should listen on
    #[envconfig(from = "OPEN_HOST", default = "127.0.0.1:9123")]
    pub host: String,
}

/// Handle GET /open by opening the target with the system runner
fn open(form: web::Query<cli::OpenTarget>) -> HttpResponse {
    let span = tracing::debug_span!("open", target = %format!("{:?}", form.target));
    let _guard = span.enter();

    let open = cli::OpenTarget {
        target: form.target.to_string(),
    };

    let res = cmd::get_system_runner().run(&open);
    if let Err(err) = res {
        tracing::warn!("{}", err);
        HttpResponse::Ok().body(format!("{}", err))
    } else {
        tracing::debug!("{:?}", res);
        HttpResponse::Ok().finish()
    }
}

/// Start open-here server
pub fn serve() -> std::io::Result<()> {
    let cfg = Config::init_from_env().unwrap();

    actix_web::rt::System::new("main").block_on(async move {
        {
            HttpServer::new(|| App::new().route("/open", web::get().to(open)))
                .bind(cfg.host)?
                .run()
                .await
        }
    })
}
