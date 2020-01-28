//! Implementing a [Chat Client]
//!
//! [chat client]: https://book.async.rs/tutorial/implementing_a_client.html
#![recursion_limit = "1024"]
use async_std::io::{self, BufReader};
use async_std::net::{TcpStream, ToSocketAddrs};
use async_std::task;
use futures::future::FutureExt;
use futures::io::AsyncBufReadExt;
use futures::io::AsyncWriteExt;
use futures::select;
use futures::stream::StreamExt;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("localhost:8039"));
    task::block_on(run(&addr))
}

async fn run(addr: impl ToSocketAddrs) -> Result<()> {
    io::stderr().write_all(b"What is your name? ").await?;
    let mut lines = BufReader::new(io::stdin()).lines();
    let name = match lines.next().await {
        None => return Err("exited".into()),
        Some(name) => name?.trim().to_string(),
    };
    let s = TcpStream::connect(addr).await?;
    let (rx, mut tx) = (&s, &s);
    tx.write_all(name.as_bytes()).await?;
    tx.write_all(b"\n").await?;
    let prompt = format!("{}> ", name);
    io::stderr().write_all(prompt.as_bytes()).await?;
    let mut lines = lines.fuse();
    let mut resp = BufReader::new(rx).lines().fuse();
    loop {
        select! {
            line = lines.next().fuse() => match line {
                None => break,
                Some(line) => {
                    let line = line?.trim().to_string();
                    let line = format!("{}\n", line);
                    tx.write_all(line.as_bytes()).await?;
                    io::stderr().write_all(prompt.as_bytes()).await?;
                }
            },
            line = resp.next().fuse() => match line {
                None => break,
                Some(line) => {
                    let line = line?;
                    if line.len() == 0 {
                        continue;
                    }
                    let output = format!("{}\n{}", line, prompt);
                    io::stderr().write_all(output.as_bytes()).await?;
                },
            },
        }
    }
    Ok(())
}
