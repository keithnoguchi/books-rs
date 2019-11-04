// SPDX-License-Identifier: GPL-2.0
//
// https://rust-lang.github.io/async-book/print.html#applied-simple-http-server
use {
    futures::{
        compat::Future01CompatExt,
        future::{FutureExt, TryFutureExt},
    },
    hyper::{rt::run, service::service_fn, Body, Client, Request, Response, Server, Uri},
    std::net::SocketAddr,
};

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("hello, world")))
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);
    let serve_future =
        Server::bind(&addr).serve(|| service_fn(|req| serve_req(req).boxed().compat()));

    if let Err(e) = serve_future.compat().await {
        eprintln!("server error: {}", e);
    }
}

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let futures_03_future = run_server(addr);
    let futures_01_future = futures_03_future.unit_error().boxed().compat();
    run(futures_01_future);
}
