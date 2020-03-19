//! [tonic] [helloworld] server
//!
//! [tonic]: https://lib.rs/tonic
//! [helloword]: https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md
//!
//! # Examples
//!
//! ```sh
//! $ cargo run -q --example hello-server [::1]:8081
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
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

use tonic::{transport::Server, Request, Response, Status};

use tonic_book::hello::{self, greeter_server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("[::1]:8080"))
        .parse()?;
    let handler = Handler::default();
    Ok(Server::builder()
        .add_service(greeter_server::GreeterServer::new(handler))
        .serve(addr)
        .await?)
}

#[derive(Debug, Default)]
struct Handler {
    counter: AtomicUsize,
}

#[tonic::async_trait]
impl greeter_server::Greeter for Handler {
    async fn say_hello(
        &self,
        req: Request<hello::HelloRequest>,
    ) -> Result<Response<hello::HelloResponse>, Status> {
        let counter = self.counter.fetch_add(1, SeqCst);
        println!("[{}]: got a request", counter);
        let resp = hello::HelloResponse {
            message: format!("Hello {}{}!", req.into_inner().name, counter),
        };
        Ok(Response::new(resp))
    }
}
