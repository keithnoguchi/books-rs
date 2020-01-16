//! Example: An Echo Server
//!
//! [an echo server]: https://github.com/tokio-rs/book/blob/master/getting-started/echo.md
use std::error::Error;
use tokio::io::copy;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn Error>> {
    let mut runtime = Runtime::new()?;
    runtime.block_on(async {
        let mut l = TcpListener::bind("127.0.0.1:8081").await?;
        loop {
            let (mut s, addr) = l.accept().await?;
            tokio::spawn(async move {
                let (mut rx, mut tx) = s.split();
                match copy(&mut rx, &mut tx).await {
                    Ok(n) => println!("echoed {} bytes to {}", n, addr),
                    Err(e) => eprintln!("copy error: {:?}", e),
                }
            });
        }
    })
}
