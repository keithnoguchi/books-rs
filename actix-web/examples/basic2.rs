//! [actix-web] server example with [actix-rt]
//!
//! [actix-rt]: https://lib.rs/crates/actix-rt
//! [actix-web]: https://lib.rs/crates/actix-web
use actix_web::{get, web, App, HttpServer, Responder};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}
