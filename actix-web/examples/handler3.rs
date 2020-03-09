//! Different return [types]
//!
//! [types]: https://actix.rs/docs/handlers/
use core::sync::atomic::{AtomicU64, Ordering};
use std::io;

use actix_web::{web, App, Either, Error, HttpResponse, HttpServer};

struct Server {
    counter: AtomicU64,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let state = web::Data::new(Server {
        counter: AtomicU64::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

async fn index(state: web::Data<Server>) -> RegisterResult {
    let counter = state.counter.fetch_add(1, Ordering::SeqCst);
    if counter % 2 == 0 {
        Either::A(HttpResponse::BadRequest().body("Bad request"))
    } else {
        Either::B(Ok("Hello!"))
    }
}
