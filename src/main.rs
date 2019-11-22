// SPDX-License-Identifier: GPL-2.0
// https://rust-lang.github.io/async-book/01_getting_started/05_http_server_example.html
use futures::compat::Future01CompatExt;
use futures::future::{FutureExt, TryFutureExt};
use hyper::rt::run;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};
use std::net::SocketAddr;

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("hello, world!")))
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    let serve_future =
        Server::bind(&addr).serve(|| service_fn(|req| serve_req(req).boxed().compat()));

    if let Err(err) = serve_future.compat().await {
        eprintln!("server error: {}", err);
    }
}

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let futures_03_future = run_server(addr);
    let futures_01_future = futures_03_future.unit_error().boxed().compat();

    run(futures_01_future);
}
