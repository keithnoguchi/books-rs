//! [hyper] hello client
//!
//! [hyper]: https://hyper.rs/guides/client/basic/
use hyper::client::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or(String::from("http://httpbin.org/ip"));
    tokio::runtime::Runtime::new()?.block_on(client(addr))
}

async fn client(addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let uri = addr.parse()?;
    let resp = client.get(uri).await?;
    println!("Response: {}", resp.status());
    Ok(())
}
