//! The server handling open requests.

use crate::cli;
use crate::cmd;

use actix_web::{web, App, HttpServer, HttpResponse};

fn open(form: web::Query<cli::OpenTarget>) -> HttpResponse {
    let span = tracing::debug_span!("open", target = %format!("{:?}", form.target));
    let _guard = span.enter();

    let open = cli::OpenTarget { target: form.target.to_string(), };

    let _res = cmd::get_system_runner().run(open);
    tracing::debug!("{:?}", _res);

    HttpResponse::Ok().finish()
}

pub fn serve() -> std::io::Result<()> {
    actix_web::rt::System::new("main").block_on(async move {
        {
            HttpServer::new(|| {
                App::new().route("/open", web::get().to(open))
            })
                .bind("127.0.0.1:8010")?
                .run()
                .await
        }
    })
}
