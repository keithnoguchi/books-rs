//! Application [scope]
//!
//! [scope]: https://actix.rs/docs/application/
use actix_web::{web, App, HttpServer, Responder};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new().service(
            web::scope("/app")
                .route("/index.html", web::get().to(index))
                .route("/index2.html", web::get().to(index))
                .route("test", web::get().to(index)),
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

async fn index() -> impl Responder {
    "Hello world!"
}
