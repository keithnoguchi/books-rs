//! [Route] guide server
//!
//! [route]: https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
use tonic::transport::Server;

use tonic_book::{GreeterService, RouteGuideService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("[::1]:8080"))
        .parse()?;
    Ok(Server::builder()
        .add_service(GreeterService::new())
        .add_service(RouteGuideService::new())
        .serve(addr)
        .await?)
}
