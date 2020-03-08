//! [Streaming] response body
//!
//! [streaming]: https://actix.rs/docs/handlers/
use std::io;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use bytes::Bytes;
use futures::{future::ok, stream::once};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().route("/async", web::to(index)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

async fn index() -> HttpResponse {
    let body = once(ok::<_, Error>(Bytes::from_static(b"test")));
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}
