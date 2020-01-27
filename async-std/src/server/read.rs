//! [`Reader`] type
//!
//! [`reader`]: struct.Reader.html
use async_std::io::BufReader;
use async_std::net::TcpStream;
use futures::channel::mpsc;
use futures::io::AsyncBufReadExt;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use std::sync::Arc;

use super::message::Event;
use super::Result;
use super::Sender;

/// `Reader` polls on the `TcpStream` and send transfer received message
/// to the [`Broker`].
///
/// [`broker`]: ../broker/struct.Broker.html
pub struct Reader {
    name: String,
    broker: Sender<Event>,
}

impl Reader {
    pub fn new(broker: Sender<Event>) -> Self {
        Self {
            name: String::from("reader"),
            broker,
        }
    }
    pub async fn run(mut self, stream: TcpStream) -> Result<()> {
        let peer = stream
            .peer_addr()
            .map(|s| s.to_string())
            .unwrap_or_else(|_| String::from("unknown"));
        let peer = format!("{}@{}", self.name, peer);
        eprintln!("[{}] starting", peer);
        let stream = Arc::new(stream);
        let mut lines = BufReader::new(&*stream).lines();
        let name = match lines.next().await {
            None => return Err(format!("[{}] premature close", peer).into()),
            Some(name) => name?.trim().to_string(),
        };
        let (_tx, cancel) = mpsc::unbounded();
        self.broker
            .send(Event::Join {
                name: name.clone(),
                stream: stream.clone(),
                cancel,
            })
            .await?;
        eprintln!("[{}] started for {}", peer, name);
        while let Some(line) = lines.next().await {
            let msg = line?.trim().to_string();
            let from = name.clone();
            self.broker.send(Event::Message { from, msg }).await?;
        }
        Ok(())
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        eprintln!("[{}] dropped", self.name);
    }
}
