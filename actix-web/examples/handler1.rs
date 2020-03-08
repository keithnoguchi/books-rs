//! JSON response handler
//!
//! [JSON]: https://actix.rs/docs/handlers/
use core::time::Duration;
use std::{env, io, sync::mpsc, thread};

use actix_rt::System;
use actix_web::{dev, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;
use tokio::time;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    static DELAY: u64 = 1; // ms
    let delay: Duration = env::args()
        .nth(1)
        .map(|ms| {
            ms.parse()
                .map(|ms| Duration::from_millis(ms))
                .unwrap_or(Duration::from_millis(DELAY))
        })
        .unwrap_or(Duration::from_millis(DELAY));
    let sleep = delay * 10;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || server(tx, "127.0.0.1:8088", 60, delay));
    let server = rx.recv().expect("no server instance");
    time::delay_for(sleep).await;
    server.stop(true).await;
    Ok(())
}

struct Server {
    name: &'static str,
    delay: Duration,
}

type Sender = mpsc::Sender<dev::Server>;

fn server(tx: Sender, addr: &str, timeout: u64, delay: Duration) -> io::Result<()> {
    let name = "http-server";
    let sys = System::new(name);
    let state = web::Data::new(Server { name, delay });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
    })
    .bind(addr)?
    .shutdown_timeout(timeout)
    .workers(4)
    .run();
    let _ = tx.send(server);
    println!("fired");
    sys.run()
}

async fn index(state: web::Data<Server>) -> impl Responder {
    time::delay_for(state.delay).await;
    Data {
        name: state.name,
        delay: state.delay,
    }
}

#[derive(Serialize)]
struct Data {
    name: &'static str,
    delay: Duration,
}

impl Responder for Data {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
