//! Shared [mutable] state
//!
//! [mutable]: https://actix.rs/docs/application/
use std::sync::Mutex;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(CounterState {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

struct CounterState {
    counter: Mutex<u32>,
}

async fn index(data: web::Data<CounterState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Hello #{}", counter)
}
