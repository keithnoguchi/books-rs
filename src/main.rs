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

async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Got request at {:?}", req.uri());
    let url_str = "http://www.rust-lang.org/en-US/";
    let url = url_str.parse::<Uri>().expect("failed to parse URL");
    let res = Client::new().get(url).compat().await;
    println!("request finished-- returning response");
    res
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);
    let f = Server::bind(&addr).serve(|| service_fn(|req| serve_req(req).boxed().compat()));

    if let Err(e) = f.compat().await {
        eprintln!("server error: {}", e);
    }
}

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let futures_03_future = run_server(addr);
    let futures_01_future = futures_03_future.unit_error().boxed().compat();
    run(futures_01_future);
}
