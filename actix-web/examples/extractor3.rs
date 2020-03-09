//! Query based path [extractor] example
//!
//! [extractor]: https://actix.rs/docs/extractors/
use std::io;

use actix_web::{web, App, HttpRequest, HttpServer, Result};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/users/{id}/{name}", web::get().to(users)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

async fn users(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("name").unwrap().parse().unwrap();
    let id: u32 = req.match_info().get("id").unwrap().parse().unwrap();
    Ok(format!("Welcome {}, userid {}", name, id))
}
