//! Example: An Echo Server
//!
//! [an echo server]: https://github.com/tokio-rs/book/blob/master/getting-started/echo.md
use std::error::Error;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn Error>> {
    let mut runtime = Runtime::new()?;

    runtime.block_on(async {
        let _l = TcpListener::bind("127.0.0.1:8081").await?;
        Ok(())
    })
}
