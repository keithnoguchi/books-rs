//! [`Listener`] type
//!
//! [`listen`]: struct.Listen.html
use async_std::net::TcpListener;
use async_std::task;
use futures::stream::StreamExt;
use std::sync::Arc;

use super::message::Event;
use super::read::Reader;
use super::Result;
use super::Sender;

/// `Listener` listens on the `addr` and spawns [`Reader`] task
/// for each client.
///
/// [`reader`]: ../read/struct.Reader.html
pub struct Listener {
    name: String,
    broker: Sender<Event>,
    addr: Arc<String>,
}

impl Listener {
    pub fn new(broker: Sender<Event>, addr: Arc<String>) -> Self {
        Self {
            name: String::from("listener"),
            broker,
            addr,
        }
    }
    pub async fn run(self) -> Result<()> {
        let s = TcpListener::bind(&*self.addr).await?;
        let mut readers = Vec::new();
        while let Some(s) = s.incoming().next().await {
            match s {
                Err(err) => eprintln!("[{}] accept error: {}", self.name, err),
                Ok(s) => readers.push(task::spawn(Reader::new(self.broker.clone()).run(s))),
            }
        }
        while let Some(reader) = readers.pop() {
            let id = reader.task().id();
            if let Err(err) = reader.await {
                eprintln!("[{}] reader({}) error: {}", self.name, id, err);
            }
        }
        Ok(())
    }
}
