//! [`Broker`] type
//!
//! [`broker`]: struct.Broker.html
use async_std::task;
use futures::channel::mpsc;
use futures::future::FutureExt;
use futures::io::AsyncWriteExt;
use futures::select;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use std::collections::{hash_map::Entry, HashMap};

use super::message::Event;
use super::write::Writer;
use super::Receiver;
use super::Result;

/// `Broker` task to manage client `Writer` instance.
pub struct Broker {
    name: String,
    events: Receiver<Event>,
}

impl Broker {
    pub fn new(events: Receiver<Event>) -> Self {
        Self {
            name: String::from("broker"),
            events,
        }
    }
    pub async fn run(self) -> Result<()> {
        eprintln!("[{}] starting", self.name);
        let mut writers = Vec::new();
        let mut peers = HashMap::new();
        let (writer_tx, writer_rx) = mpsc::unbounded::<(String, Receiver<Option<String>>)>();
        let mut writer_rx = writer_rx.fuse();
        let mut events = self.events.fuse();
        eprintln!("[{}] started", self.name);
        loop {
            let event = select! {
                writer = writer_rx.next().fuse() => match writer {
                    None => {
                        eprintln!("[{}] writer shutdown error", self.name);
                        continue
                    }
                    Some((peer, mut rx)) => {
                        peers.remove(&peer);
                        while let Some(_) = rx.next().await {};
                        continue
                    }
                },
                event = events.next().fuse() => match event {
                    None => break,
                    Some(event) => event,
                },
            };
            match event {
                Event::Join {
                    name,
                    stream,
                    cancel,
                } => match peers.entry(name.clone()) {
                    Entry::Occupied(_) => {
                        let msg = format!("the name {:?} has been take", name);
                        if let Err(err) = (&*stream).write_all(msg.as_bytes()).await {
                            eprintln!("[{}] {}", self.name, err);
                        }
                    }
                    Entry::Vacant(e) => {
                        let (mut tx, mut rx) = mpsc::unbounded();
                        let writer = Writer::new(name.clone());
                        let mut writer_tx = writer_tx.clone();
                        writers.push(task::spawn(async move {
                            let ret = writer.run(cancel, &mut rx, stream).await;
                            if let Err(err) = writer_tx.send((name, rx)).await {
                                eprintln!("[writer] cannot send shutdown msg: {}", err,);
                            }
                            ret
                        }));
                        tx.send(None).await?;
                        e.insert(tx);
                    }
                },
                Event::Message { from, msg } => {
                    let msg = format!("{}: {}\n", from, msg.trim());
                    for (to, mut writer) in &peers {
                        if to == &from {
                            writer.send(None).await?;
                        } else {
                            writer.send(Some(msg.clone())).await?;
                        }
                    }
                }
            }
        }
        peers.drain();
        for writer in writers {
            let id = writer.task().id();
            if let Err(err) = writer.await {
                eprintln!("[{}] writer({}) error: {}", self.name, id, err);
            }
        }
        eprintln!("[{}] finished", self.name);
        Ok(())
    }
}
