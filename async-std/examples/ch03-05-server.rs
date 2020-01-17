//! [Connecting Readers and Writers]: Broker example
//!
//! This is the fourth version of the chat server, based off of
//! [accept loop], [receiving messages], and [sending messages]
//! examples.
//!
//! # Examples
//!
//! ```sh
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
use std::{env, error::Error, future::Future, result, sync::Arc};

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

/// Event is for the communication between the broker and the servers.
#[derive(Debug)]
enum Event {
    Register {
        name: String,
        stream: Arc<TcpStream>,
    },
    Deregister(String),
    Message {
        from: String,
        to: Vec<String>,
        msg: String,
    },
}

fn main() -> Result<()> {
    let argv: Vec<String> = env::args().collect();
    let addr = match argv.len() {
        0..=1 => "localhost:8035",
        _ => &argv[1],
    };
    task::block_on(listener(&addr))
}

/// `listener()` listens on the `addr` address and spawns the server tasks
/// for each connections.
async fn listener(addr: impl ToSocketAddrs) -> Result<()> {
    let (tx, rx) = mpsc::unbounded();
    task::spawn(broker(rx));
    let l = TcpListener::bind(addr).await?;
    while let Some(s) = l.incoming().next().await {
        match s {
            Err(err) => eprintln!("[listener] accept error: {:?}", err),
            Ok(s) => {
                spawn(server(s, tx.clone()));
            }
        }
    }
    Ok(())
}

async fn broker(mut rx: Receiver<Event>) -> Result<()> {
    let mut peers = HashMap::new();
    while let Some(msg) = rx.next().await {
        match msg {
            Event::Register { name, stream } => {
                let (tx, rx) = mpsc::unbounded();
                eprintln!("[broker] register {}", name);
                peers.insert(name, tx);
                spawn(responder(rx, stream));
            }
            Event::Deregister(name) => {
                peers.remove(&name);
                eprintln!("[broker] deregistered {}", name);
            }
            Event::Message { from: _, to, msg } => {
                for name in &to {
                    if let Some(tx) = peers.get_mut(name) {
                        tx.send(msg.clone()).await?;
                    }
                }
            }
        }
    }
    rx.close();
    eprintln!("[broker] draining messages");
    while let Some(_) = rx.next().await {}
    Ok(())
}

/// `server()` serves the specific client over the [TcpStream].
async fn server(s: TcpStream, mut tx: Sender<Event>) -> Result<()> {
    let peer = match s.peer_addr() {
        Err(_) => String::from("unknown"),
        Ok(s) => s.to_string(),
    };
    eprintln!("[server:{}] connected", peer);
    let s = Arc::new(s);
    let rx = s.clone();
    let mut lines = BufReader::new(&*rx).lines();
    let name = match lines.next().await {
        None => Err(format!("[server:{}] premature connection close", peer))?,
        Some(name) => name?,
    };
    let event = Event::Register {
        name: name.clone(),
        stream: s.clone(),
    };
    tx.send(event).await?;
    eprintln!("[server:{}@{}] registered", name, peer);
    while let Some(msg) = lines.next().await {
        let msg = msg?;
        let (dest, msg) = match msg.find(':') {
            Some(idx) => (&msg[..idx], &msg[idx + 1..]),
            None => {
                eprintln!("[server:{}@{}] wrong format: {:?}", name, peer, msg);
                continue;
            }
        };
        let to: Vec<String> = dest
            .split(',')
            .map(|dest| dest.trim().to_string())
            .collect();
        let msg = msg.trim().to_string();
        let event = Event::Message {
            from: name.clone(),
            to,
            msg,
        };
        tx.send(event).await?;
    }
    let event = Event::Deregister(name);
    tx.send(event).await?;
    Ok(())
}

async fn responder(mut rx: Receiver<String>, s: Arc<TcpStream>) -> Result<()> {
    eprintln!("[responder] started");
    while let Some(msg) = rx.next().await {
        (&*s).write_all(msg.as_bytes()).await?;
    }
    eprintln!("[responder] finished");
    Ok(())
}

/// `spawn()` spawns a future through `task::spawn()` and print out the
/// error message on stderr in case of the future returns error.
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
