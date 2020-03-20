//! [tonic] [helloworld] server
//!
//! [tonic]: https://lib.rs/tonic
//! [helloword]: https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md
//!
//! # Examples
//!
//! ```sh
//! $ cargo run -q --example hello-server [::1]:8080
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
use tonic::transport::Server;

use tonic_book::GreeterService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("[::1]:8080"))
        .parse()?;
    Ok(Server::builder()
        .add_service(GreeterService::new())
        .serve(addr)
        .await?)
}
