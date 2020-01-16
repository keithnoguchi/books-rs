//! [Hello TCP client] example
//!
//! [hello tcp client]: https://github.com/tokio-rs/book/blob/master/getting-started/hello-world.md
use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn Error>> {
    let mut runtime = Runtime::new()?;

    runtime.block_on(async {
        let mut s = TcpStream::connect("127.0.0.1:8080").await?;
        s.write_all(b"Hello world!\n").await?;
        Ok(())
    })
}
