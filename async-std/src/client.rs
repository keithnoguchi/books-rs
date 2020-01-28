//! Implementing a [Chat Client]
//!
//! # Examples
//!
//! ```no_run
//! use async_std::io;
//! use async_std::task;
//! use async_std_book::Client;
//!
//! type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
//!
//! fn main() -> Result<(), Error> {
//!     let addr = std::env::args()
//!         .skip(1)
//!         .next()
//!         .unwrap_or(String::from("[::1]:8000"));
//!     task::block_on(Client::new(addr).run(io::stdin(), io::stderr()))
//! }
//! ```
//! [chat client]: https://book.async.rs/tutorial/implementing_a_client.html
use async_std::io::BufReader;
use async_std::net::TcpStream;
use futures::future::FutureExt;
use futures::io::AsyncBufReadExt;
use futures::io::AsyncRead;
use futures::io::AsyncWrite;
use futures::io::AsyncWriteExt;
use futures::io::Lines;
use futures::select;
use futures::stream::StreamExt;

use super::Result;

pub struct Client {
    addr: String,
    prompt: Option<String>,
}

impl Client {
    /// `new` creates a new server instance.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_std_book::Client;
    ///
    /// let addr = std::env::args()
    ///     .skip(1)
    ///     .next()
    ///     .unwrap_or(String::from("localhost:8000"));
    /// let _client = Client::new(addr);
    /// ```
    pub fn new(addr: String) -> Self {
        Self { addr, prompt: None }
    }
    /// `run` creates a `Future` instance which handles all the
    /// business logic.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_std::io;
    /// use async_std::task;
    /// use async_std_book::Client;
    ///
    /// let addr = std::env::args()
    ///     .skip(1)
    ///     .next()
    ///     .unwrap_or(String::from("localhost:8000"));
    /// task::block_on(Client::new(addr).run(io::stdin(), io::stderr()));
    /// ```
    pub async fn run<R: AsyncRead + Unpin, W: AsyncWrite + Unpin>(
        mut self,
        reader: R,
        mut writer: W,
    ) -> Result<()> {
        let mut lines = BufReader::new(reader).lines();
        let name = self.name(&mut writer, &mut lines).await?;
        let s = TcpStream::connect(&self.addr).await?;
        let (tx, rx) = (&s, &s);
        Self::send(tx, &name).await?;
        let mut lines = lines.fuse();
        let mut server = BufReader::new(rx).lines().fuse();
        loop {
            self.prompt(&mut writer).await?;
            select! {
                line = lines.next().fuse() => match line {
                    None => break,
                    Some(line) => {
                        let line = line?.trim().to_string();
                        Self::send(tx, &line).await?;
                    }
                },
                line = server.next().fuse() => match line {
                    None => break,
                    Some(line) => {
                        let line = line?.trim().to_string();
                        if line.len() == 0 {
                            continue;
                        }
                        Self::write(&mut writer, &line).await?;
                    }
                },
            }
        }
        Ok(())
    }
    async fn name<R: AsyncRead + Unpin, W: AsyncWrite + Unpin>(
        &mut self,
        writer: &mut W,
        lines: &mut Lines<BufReader<R>>,
    ) -> Result<String> {
        writer.write_all(b"What is your name? ").await?;
        let name = match lines.next().await {
            None => return Err("premature reader close".into()),
            Some(name) => name?.trim().to_string(),
        };
        self.prompt = Some(format!("{}> ", name));
        Ok(name)
    }
    async fn prompt<W: AsyncWrite + Unpin>(&self, writer: &mut W) -> Result<()> {
        match &self.prompt {
            Some(prompt) => writer.write_all(prompt.as_bytes()).await?,
            None => return Err("no prompt set".into()),
        }
        Ok(())
    }
    async fn write<W: AsyncWrite + Unpin>(writer: &mut W, msg: &str) -> Result<()> {
        let msg = format!("{}\n", msg);
        writer.write_all(msg.as_bytes()).await?;
        Ok(())
    }
    async fn send(mut server: &TcpStream, msg: &str) -> Result<()> {
        server.write_all(msg.as_bytes()).await?;
        server.write_all(b"\n").await?;
        Ok(())
    }
}
