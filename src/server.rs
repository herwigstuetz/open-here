//! The server handling open requests.

use crate::cli;
use crate::cmd;

use envconfig::Envconfig;

#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "OPEN_HOST", default = "127.0.0.1:9123")]
    pub host: String,
}

use actix_web::{web, App, HttpResponse, HttpServer};

fn open(form: web::Query<cli::OpenTarget>) -> HttpResponse {
    let span = tracing::debug_span!("open", target = %format!("{:?}", form.target));
    let _guard = span.enter();

    let open = cli::OpenTarget {
        target: form.target.to_string(),
    };

    let _res = cmd::get_system_runner().run(&open);
    tracing::debug!("{:?}", _res);

    HttpResponse::Ok().finish()
}

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
