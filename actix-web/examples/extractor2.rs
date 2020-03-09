//! Serde deserialize based path [extractor] example
//!
//! [extractor]: https://actix.rs/docs/extractors/
use std::io;

use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/users/{id}/{name}", web::get().to(users)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

#[derive(Deserialize)]
struct User {
    name: String,
    id: u32,
}

async fn users(info: web::Path<User>) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}", info.name, info.id))
}
