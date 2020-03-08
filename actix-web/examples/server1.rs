//! The HTTP [Server]
//!
//! [server]: https://actix.rs/docs/server/
use core::time::Duration;
use std::{env, io, sync::mpsc, thread};

use actix_rt::System;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use tokio::time;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || server(tx, "127.0.0.1:8088", 60));
    let server = rx.recv().expect("no server instance");
    let sleep: u64 = env::args()
        .nth(1)
        .map(|ms| ms.parse().unwrap_or(1))
        .unwrap_or(1);
    // wait a bit.
    time::delay_for(Duration::from_millis(sleep)).await;
    // pause accepting new connection.
    server.pause().await;
    // resume accepting new connection.
    server.resume().await;
    // stop server.
    server.stop(true).await;
    println!("done!");
    Ok(())
}

fn server(tx: mpsc::Sender<Server>, addr: &str, timeout: u64) -> io::Result<()> {
    let sys = System::new("http-server");
    let server = HttpServer::new(|| App::new().route("/", web::get().to(|| HttpResponse::Ok())))
        .bind(addr)?
        .shutdown_timeout(timeout)
        .run();
    let _ = tx.send(server);
    println!("fired");
    sys.run()
}
