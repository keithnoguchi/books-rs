//! [Clean Shutdown]
//!
//! Let's cleanup all the tasks gracefully.  This is the continuation of
//! the chat server example, based off of [accept loop], [receiving messages],
//! [sending messages], and [connecting readers and writers].
//!
//! # Examples
//!
//! ```sh
//! $ cargo run --example ch03-07-server [::1]:8000
//! Compiling async-std-book v0.1.0 (/home/kei/git/books-rs/async-std)
//! Finished dev [unoptimized + debuginfo] target(s) in 1.58s
//! Running `/home/kei/git/books-rs/target/debug/examples/ch03-07-server '[::1]:8000'`
//! [supervisor] starting
//! [supervisor] started
//! [server] starting
//! [broker] started
//! [reader@[::1]:44656] starting
//! [reader@[::1]:44656] started for "one"
//! [writer@[::1]:44656] starting for "one"
//! [writer@[::1]:44656] started for "one"
//! [reader@[::1]:44658] starting
//! [reader@[::1]:44658] started for "two"
//! [writer@[::1]:44658] starting for "two"
//! [writer@[::1]:44658] started for "two"
//! [reader@[::1]:44656] finished for "one"
//! [writer@[::1]:44656] finished for "one"
//! [reader@[::1]:44658] finished for "two"
//! [writer@[::1]:44658] finished for "two"
//! ```
//! [clean shutdown]: https://book.async.rs/tutorial/clean_shutdown.html
//! [accept loop]: ch03-02-server.rs
//! [receiving messages]: ch03-03-server.rs
//! [sending messages]: ch03-04-server.rs
//! [connecting readers and writers]: ch03-05-server.rs
use async_std::io::prelude::{BufReadExt, WriteExt};
use async_std::io::BufReader;
use async_std::net::{TcpListener, TcpStream};
use async_std::stream::StreamExt;
use async_std::task;
use futures::channel::mpsc;
use futures::sink::SinkExt;
use std::collections::HashMap;
use std::{error::Error, result, sync::Arc};

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

/// Client activity events.
#[derive(Debug)]
enum Event {
    /// The client join event.
    Join {
        name: String,
        stream: Arc<TcpStream>,
    },
    /// The client leave event.
    Leave(String),
    Message {
        from: String,
        msg: String,
    },
}

fn main() -> Result<()> {
    let addr = std::env::args()
        .skip(1) // skip argv[0]
        .next()
        .unwrap_or(String::from("localhost:8037"));
    task::block_on(supervisor(addr))
}

/// `supervisor()` creates `server()` and `broker()` tasks and handle the
/// cleanup process.
async fn supervisor(addr: String) -> Result<()> {
    let mut sleep = std::time::Duration::from_secs(1);
    loop {
        eprintln!("[supervisor] starting");
        let (tx, rx) = mpsc::unbounded();
        let server = task::spawn(server(tx, addr.clone()));
        let broker = task::spawn(broker(rx));
        eprintln!("[supervisor] started");
        if let Err(err) = broker.await {
            eprintln!("[broker] {}", err);
        }
        if let Err(err) = server.await {
            eprintln!("[server] {}", err);
        }
        eprintln!("[supervisor] {:?} sleep before restarting", sleep);
        task::sleep(sleep).await;
        sleep *= 2;
    }
}

/// 'server()` listens on the `addr` address and create `reader()` task
/// for servicing the clients.
async fn server(broker: Sender<Event>, addr: String) -> Result<()> {
    let mut readers = Vec::new();
    let broker = Arc::new(broker);
    eprintln!("[server] starting");
    let s = TcpListener::bind(addr).await?;
    while let Some(s) = s.incoming().next().await {
        match s {
            Err(err) => eprintln!("[server] accept error: {:?}", err),
            Ok(s) => {
                let reader = task::spawn(reader(broker.clone(), s));
                readers.push(reader);
            }
        }
    }
    drop(broker);
    for reader in readers {
        if let Err(err) = reader.await {
            eprintln!("[server] reader has been gone: {}", err);
        }
    }
    eprintln!("[server] finished");
    Ok(())
}

/// `reader()` reads a message from the client and send it to the
/// `broker()`.
async fn reader(broker: Arc<Sender<Event>>, mut s: TcpStream) -> Result<()> {
    let mut broker = &*broker;
    s.write_all("What is your name? ".as_bytes()).await?;
    let s = Arc::new(s);
    let peer = s
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or(String::from("unknown"));
    eprintln!("[reader@{}] starting", peer);
    let rx = s.clone();
    let mut lines = BufReader::new(&*rx).lines();
    let from = match lines.next().await {
        None => Err(format!("[reader@{}] client closed connection", peer))?,
        Some(from) => from?,
    };
    let name = from.clone();
    let stream = s.clone();
    broker.send(Event::Join { name, stream }).await?;
    eprintln!("[reader@{}] started for {:?}", peer, from);
    while let Some(line) = lines.next().await {
        let line = line?;
        let from = from.clone();
        let msg = format!("{}: {}\n", from, line.trim());
        broker.send(Event::Message { from, msg }).await?;
    }
    broker.send(Event::Leave(from.to_string())).await?;
    eprintln!("[reader@{}] finished for {:?}", peer, from);
    Ok(())
}

/// `broker()` mediates the `reader()` and `writer()` for the chat
/// message multicasting over [TcpStream].
///
/// [TcpStream]: https://docs.rs/async-std/1.4.0/async_std/net/struct.TcpStream.html
async fn broker(mut readers: Receiver<Event>) -> Result<()> {
    let mut writers = Vec::new();
    let mut peers = HashMap::new();
    eprintln!("[broker] started");
    while let Some(event) = readers.next().await {
        match event {
            Event::Join { name, stream } => {
                peers.entry(name.clone()).or_insert({
                    let (tx, rx) = mpsc::unbounded();
                    let writer = task::spawn(writer(rx, name, stream));
                    writers.push(writer);
                    tx
                });
            }
            Event::Leave(name) => {
                peers.remove(&name);
            }
            Event::Message { from, msg } => {
                for (to, mut writer) in &peers {
                    let result = if to == &from {
                        writer.send(None).await
                    } else {
                        writer.send(Some(msg.clone())).await
                    };
                    if let Err(err) = result {
                        eprintln!("[broker] sending error to {}: {}", to, err);
                    }
                }
            }
        }
    }
    drop(peers);
    for writer in writers {
        if let Err(err) = writer.await {
            eprintln!("[broker]: writer has been gone: {}", err);
        }
    }
    eprintln!("[broker] finished");
    Ok(())
}

/// `writer` responsibles for the communication with the client.
async fn writer(
    mut broker: Receiver<Option<String>>,
    name: String,
    s: Arc<TcpStream>,
) -> Result<()> {
    let peer = s
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or(String::from("unknown"));
    eprintln!("[writer@{}] starting for {:?}", peer, name);
    let prompt = format!("{}> ", name);
    let mut s = &*s;
    s.write_all(prompt.as_bytes()).await?;
    eprintln!("[writer@{}] started for {:?}", peer, name);
    while let Some(msg) = broker.next().await {
        if let Some(msg) = msg {
            s.write_all(msg.as_bytes()).await?;
        }
        s.write_all(prompt.as_bytes()).await?;
    }
    eprintln!("[writer@{}] finished for {:?}", peer, name);
    Ok(())
}
