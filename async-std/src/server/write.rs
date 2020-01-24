//! [`Writer`] type
//!
//! [`writer`]: struct.Writer.html
use async_std::net::TcpStream;
use futures::future::FutureExt;
use futures::io::AsyncWriteExt;
use futures::select;
use futures::stream::StreamExt;
use std::sync::Arc;

use super::Cancel;
use super::Receiver;
use super::Result;

/// `Writer` waits for a message from `Broker` and writes to the `TcpStream`.
pub struct Writer {
    name: String,
    prompt: String,
    to: String,
}

impl Writer {
    pub fn new(to: String) -> Self {
        Self {
            name: String::from("writer"),
            prompt: format!("{}> ", to),
            to,
        }
    }
    pub async fn run(
        self,
        cancel: Cancel,
        broker: &mut Receiver<Option<String>>,
        stream: Arc<TcpStream>,
    ) -> Result<()> {
        let mut stream = &*stream;
        let peer = stream
            .peer_addr()
            .map(|addr| addr.to_string())
            .unwrap_or_else(|_| String::from("unknown"));
        let peer = format!("{}@{}", self.name, peer);
        eprintln!("[{}] started for {}", peer, self.to);
        let mut cancel = cancel.fuse();
        let mut broker = broker.fuse();
        loop {
            select! {
                msg = cancel.next().fuse() => match msg {
                    None => break Ok(()),
                    Some(void) => match void {},
                },
                msg = broker.next().fuse() => match msg {
                    None => break Ok(()),
                    Some(msg) => {
                        if let Some(msg) = msg {
                            stream.write_all(msg.as_bytes()).await?;
                        }
                        stream.write_all(self.prompt.as_bytes()).await?;
                    }
                },
            }
        }
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        eprintln!("[{}] dropped for {}", self.name, self.to);
    }
}
