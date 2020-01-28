//! [Connecting Readers and Writers]: Broker example
//!
//! This is the fourth version of the chat server, based off of
//! [accept loop], [receiving messages], and [sending messages]
//! examples.
//!
//! # Examples
//!
//! ```sh
//! $ cargo run --example ch03-05-server [::1]:8000
//! Compiling async-std-book v0.1.0 (/home/kei/git/books-rs/async-std)
//! Finished dev [unoptimized + debuginfo] target(s) in 1.18s
//! Running `target/debug/examples/ch03-05-server '[::1]:8000'`
//! [broker] starting broker task
//! [reader:[::1]:51944] connected
//! [reader:[::1]:51946] connected
//! [reader:uno@[::1]:51944] registered
//! [broker] "uno" is joining
//! [writer] "uno" started
//! [reader:due@[::1]:51946] registered
//! [broker] "due" is joining
//! [writer] "due" started
//! [broker] "due" left
//! [writer] finished
//! ```
//! [connecting readers and writers]: https://book.async.rs/tutorial/connecting_readers_and_writers.html
//! [accept loop]: ch03-02-server.rs
//! [receiving messages]: ch03-03-server.rs
//! [sending messages]: ch03-04-server.rs
use async_std::io::prelude::{BufReadExt, WriteExt};
use async_std::io::BufReader;
use async_std::net::{TcpListener, TcpStream, ToSocketAddrs};
use async_std::stream::StreamExt;
use async_std::task::{self, JoinHandle};
use futures::channel::mpsc;
use futures::sink::SinkExt;
use std::collections::HashMap;
use std::{error::Error, future::Future, result, sync::Arc};

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

/// Event is for the communication between the broker and the servers.
#[derive(Debug)]
enum Event {
    Join {
        name: String,
        stream: Arc<TcpStream>,
    },
    Leave(String),
    Message {
        from: String,
        to: Vec<String>,
        msg: String,
    },
}

fn main() -> Result<()> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("localhost:8035"));
    task::block_on(server(&addr))
}

/// `server()` listens on the `addr` address and spawns the `reader()` task
/// for each connections.
async fn server(addr: impl ToSocketAddrs) -> Result<()> {
    let (tx, rx) = mpsc::unbounded();
    spawn(broker(rx));
    let l = TcpListener::bind(addr).await?;
    while let Some(s) = l.incoming().next().await {
        match s {
            Err(err) => eprintln!("[server] accept error: {:?}", err),
            Ok(s) => {
                spawn(reader(tx.clone(), s));
            }
        }
    }
    Ok(())
}

/// `reader()` reads the message from the client over [TcpStream]
/// and send it to the `broker()` for the rest of the processes,
/// including the multicasting to the friends.
async fn reader(mut broker: Sender<Event>, s: TcpStream) -> Result<()> {
    let peer = s
        .peer_addr()
        .map(|s| s.to_string())
        .unwrap_or(String::from("unknown"));
    eprintln!("[reader:{}] connected", peer);
    let s = Arc::new(s);
    let rx = s.clone();
    let mut lines = BufReader::new(&*rx).lines();
    let name = match lines.next().await {
        None => Err(format!("[reader:{}] premature connection close", peer))?,
        Some(name) => name?,
    };
    let event = Event::Join {
        name: name.clone(),
        stream: s.clone(),
    };
    broker.send(event).await?;
    eprintln!("[reader:{}@{}] registered", name, peer);
    while let Some(msg) = lines.next().await {
        let msg = msg?;
        let (dest, msg) = match msg.find(':') {
            None => {
                eprintln!("[reader:{}@{}] wrong format: {:?}", name, peer, msg);
                continue;
            }
            Some(idx) => (&msg[..idx], &msg[idx + 1..]),
        };
        let from = name.clone();
        let to: Vec<String> = dest
            .split(',')
            .map(|dest| dest.trim().to_string())
            .collect();
        let msg = msg.trim().to_string();
        let event = Event::Message { from, to, msg };
        broker.send(event).await?;
    }
    let event = Event::Leave(name);
    broker.send(event).await?;
    Ok(())
}

/// `broker()` receives `Event` from `reader()` and fanout
/// the message to the specific user's `writer()`s.
async fn broker(mut reader: Receiver<Event>) -> Result<()> {
    eprintln!("[broker] starting broker task");
    let mut peers = HashMap::new();
    while let Some(msg) = reader.next().await {
        match msg {
            Event::Join { name, stream } => {
                let (tx, rx) = mpsc::unbounded();
                eprintln!("[broker] {:?} is joining", name);
                spawn(writer(rx, name.clone(), stream));
                peers.insert(name, tx);
            }
            Event::Leave(name) => {
                peers.remove(&name);
                eprintln!("[broker] {:?} left", name);
            }
            Event::Message { from, to, msg } => {
                for name in &to {
                    if let Some(writer) = peers.get_mut(name) {
                        let msg = format!("{}> {}", from, msg);
                        writer.send(msg).await?;
                    }
                }
            }
        }
    }
    reader.close();
    eprintln!("[broker] draining messages");
    while let Some(_) = reader.next().await {}
    Ok(())
}

async fn writer(mut broker: Receiver<String>, name: String, s: Arc<TcpStream>) -> Result<()> {
    eprintln!("[writer] {:?} started", name);
    while let Some(msg) = broker.next().await {
        let msg = format!("{}\n", msg);
        (&*s).write_all(msg.as_bytes()).await?;
    }
    eprintln!("[writer] finished");
    Ok(())
}

/// `spawn()` spawns a future through `task::spawn()` and print out the
/// error message on stderr in case of the future returns error.
#[inline]
fn spawn<F>(f: F) -> JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + Sync + 'static,
{
    task::spawn(async move {
        if let Err(err) = f.await {
            eprintln!("{:?}", err);
        }
    })
}
