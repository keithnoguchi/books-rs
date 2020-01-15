///! [Tokio echo server]
///
/// [tokio echo server]: https://github.com/tokio-rs/book/blob/master/overview.md
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn Error>> {
    let mut runtime = Runtime::new()?;

    runtime.block_on(async {
        let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

        loop {
            let (mut socket, _) = listener.accept().await?;

            tokio::spawn(async move {
                let mut buf = [0; 1024]; // char buf[1024];

                loop {
                    let n = match socket.read(&mut buf).await {
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("failed to read from socket: {:?}", e);
                            return;
                        }
                    };
                    if let Err(e) = socket.write_all(&buf[..n]).await {
                        println!("failed to write to socket: {:?}", e);
                        return;
                    }
                }
            });
        }
    })
}
