//! [tonic] [helloworld] server
//!
//! # Examples
//!
//! ```sh
//! $ cargo run --example hello-server [::1]:8080
//! warning: /home/kei/git/books-rs/tonic/Cargo.toml: file found to be present in multiple build targets: /home/kei/git/books-rs/tonic/examples/hello-server.rs
//! Compiling tonic-book v0.1.0 (/home/kei/git/books-rs/tonic)
//! Finished dev [unoptimized + debuginfo] target(s) in 2.18s
//! Running `/home/kei/git/books-rs/target/debug/examples/hello-server`
//!
//! ```
//!
//! then, say hello with `grpcurl`:
//!
//! ```sh
//! $ grpcurl -plaintext -import-path ./proto -proto hello.proto -d '{"name": "tonic"}' [::1]:8080 hello.Greeter/SayHello
//! {
//!   "message": "Hello tonic!"
//! }
//! ```
//! [tonic]: https://lib.rs/tonic
//! [helloword]: https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md
use tonic::{transport::Server, Request, Response, Status};

use tonic_book::hello::{
    greeter_server::{Greeter, GreeterServer},
    HelloRequest, HelloResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("[::1]:50051"))
        .parse()?;
    let greeter = MyGreeter::default();
    Ok(Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?)
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        req: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", req);
        let resp = HelloResponse {
            message: format!("Hello {}!", req.into_inner().name),
        };
        Ok(Response::new(resp))
    }
}
