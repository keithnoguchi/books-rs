//! JSON body [extractor] example
//!
//! # Examples
//!
//! ```sh
//! $ curl 127.0.0.1:8088/users -H "content-type: application/json" -d '{"name":"keith","id":1000}'
//! Welcome keith, id=1000!
//! ```
//! [extractor]: https://actix.rs/docs/extractors/
use std::io;

use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().route("users", web::post().to(users)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

#[derive(Deserialize)]
struct User {
    name: String,
    id: usize,
}

async fn users(info: web::Json<User>) -> Result<String> {
    Ok(format!("Welcome {}, id={}!", info.name, info.id))
}
