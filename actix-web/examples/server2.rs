//! The [multi-threaded] HTTP server
//!
//! [multi-threaded]: https://actix.rs/docs/server/
use core::{sync::atomic, time::Duration};
use std::{env, io, sync::mpsc, thread};

use actix_rt::System;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
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
    let (tx, rx) = mpsc::channel();
    let sleep = delay * 10;
    thread::spawn(move || server(tx, "127.0.0.1:8088", 60, delay));
    let server = rx.recv().expect("no server instance");
    // wait a bit.
    time::delay_for(sleep).await;
    // pause accepting new connection.
    server.pause().await;
    // resume accepting new connection.
    server.resume().await;
    // stop server.
    server.stop(true).await;
    println!("done!");
    Ok(())
}

struct State {
    counter: atomic::AtomicU64,
    delay: Duration,
}

fn server(tx: mpsc::Sender<Server>, addr: &str, timeout: u64, delay: Duration) -> io::Result<()> {
    let sys = System::new("http-server");
    let state = web::Data::new(State {
        counter: atomic::AtomicU64::new(1),
        delay,
    });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
    })
    .bind(addr)?
    .shutdown_timeout(timeout)
    .workers(16)
    .run();
    let _ = tx.send(server);
    println!("fired");
    sys.run()
}

async fn index(data: web::Data<State>) -> impl Responder {
    let counter = data.counter.fetch_add(1, atomic::Ordering::SeqCst);
    time::delay_for(data.delay).await;
    let data = format!("Hello #{}", counter);
    HttpResponse::Ok().body(&data)
}
