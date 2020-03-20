//! [tonic] [helloworld] client
//!
//! [tonic]: https://lib.rs/tonic
//! [helloworld]: https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md
//!
//! # Examples
//!
//! Run server:
//!
//! ```sh
//! $ cargo run -q --example hello-server [::1]:8081
//! ```
//!
//! then run the client:
//!
//! ```sh
//! $ cargo run -q --example hello-client
//! Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Tue, 10 Mar 2020 21:00:41 GMT", "grpc-status": "0"} }, message: HelloResponse { message: "Hello Tonic!" } }
//! ```
use tonic::Request;

use tonic_book::{GreeterClient, HelloRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("http://[::1]:8080"));
    let mut client = GreeterClient::connect(addr).await?;
    let req = Request::new(HelloRequest {
        name: "Tonic".into(),
    });
    let resp = client.say_hello(req).await?;
    println!("{:?}", resp);
    Ok(())
}
