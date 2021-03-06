//! [Buffering] the request body
//!
//! [buffering]: https://hyper.rs/guides/server/echo/
use core::str::FromStr;
use std::{env, error, net::SocketAddr, result};

use futures::stream::TryStreamExt;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use tokio::runtime::Runtime;

fn main() -> result::Result<(), Box<dyn error::Error>> {
    static SERVER: &'static str = "127.0.0.1:8088";
    let addr = env::args()
        .nth(1)
        .map(|addr| SocketAddr::from_str(&addr).unwrap_or(SocketAddr::from_str(SERVER).unwrap()))
        .unwrap_or(SocketAddr::from_str(SERVER).unwrap());
    Runtime::new()?.block_on(server(addr))
}

async fn server(addr: SocketAddr) -> result::Result<(), Box<dyn error::Error>> {
    let svc = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(route)) });
    let server = Server::bind(&addr).serve(svc);
    server.await.map_err(|err| err.into())
}

type Result<T> = result::Result<T, hyper::Error>;

async fn route(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/echo/reverse") => reverse(req).await,
        (&Method::POST, "/echo/uppercase") => uppercase(req).await,
        (&Method::POST, "/echo") => echo(req).await,
        (&Method::GET, "/") | (&Method::GET, "/index.html") => index(req),
        _ => not_found(req),
    }
}

async fn reverse(req: Request<Body>) -> Result<Response<Body>> {
    let full_body = hyper::body::to_bytes(req.into_body()).await?;
    let reverse = full_body.iter().rev().cloned().collect::<Vec<u8>>();
    let mut resp = Response::new(Body::empty());
    *resp.body_mut() = reverse.into();
    Ok(resp)
}

async fn uppercase(req: Request<Body>) -> Result<Response<Body>> {
    let mapping = req.into_body().map_ok(|chunk| {
        chunk
            .iter()
            .map(|byte| byte.to_ascii_uppercase())
            .collect::<Vec<u8>>()
    });
    Ok(Response::new(Body::wrap_stream(mapping)))
}

async fn echo(req: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(req.into_body()))
}

fn index(_req: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Hello from echo server")))
}

fn not_found(_req: Request<Body>) -> Result<Response<Body>> {
    let mut resp = Response::new(Body::empty());
    *resp.status_mut() = StatusCode::NOT_FOUND;
    Ok(resp)
}
