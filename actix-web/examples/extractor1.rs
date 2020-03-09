//! Path [extractor] example
//!
//! [extractor]: https://actix.rs/docs/extractors/
use std::io;

use actix_web::{web, App, HttpServer, Result};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/users/{userid}/{friend}", web::get().to(index)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

async fn index(info: web::Path<(u32, String)>) -> Result<String> {
    Ok(format!("Welcome {} userid {}!", info.1, info.0))
}
