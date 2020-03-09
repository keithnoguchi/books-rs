//! Query [extractor] example
//!
//! [extractor]: https://actix.rs/docs/extractors/
use std::io;

use actix_web::{web, App, HttpServer};
use serde::Deserialize;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/users", web::get().to(users)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

#[derive(Deserialize)]
struct User {
    username: String,
}

async fn users(info: web::Query<User>) -> String {
    format!("Welcome {}!", info.username)
}
