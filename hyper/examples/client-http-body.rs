//! [hyper] response bodies client example
//!
//! [hyper]: https://hyper.rs/guides/client/basic/
use hyper::{body::HttpBody, client::Client};
use tokio::io::AsyncWriteExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or("http://httpbin.org/ip".to_string());
    tokio::runtime::Runtime::new()?.block_on(client(addr))
}

async fn client(addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let uri = addr.parse()?;
    let mut resp = client.get(uri).await?;
    println!("Response: {}", resp.status());
    while let Some(chunk) = resp.body_mut().data().await {
        tokio::io::stdout().write_all(&chunk?).await?;
    }
    Ok(())
}
