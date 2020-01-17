//! [Sending messages] chat server example
//!
//! This is the third part of the chat server example, building
//! on top of the [accept loop] and the [receiving messages] examples.
//!
//! # Examples
//!
//! ```sh
//! $ cargo run --example ch03-04-server -- [::1]:8000
//! Finished dev [unoptimized + debuginfo] target(s) in 0.03s
//! Running `/home/kei/git/books-rs/target/debug/examples/ch03-04-server '[::1]:8000'`
//! listening on TcpListener { watcher: Watcher { entry: Entry { token: Token(1), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpListener { addr: V6([::1]:8000), fd: 3 }) } }
//! handling TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:58382), fd: 7 }) } }
//! handling TcpStream { watcher: Watcher { entry: Entry { token: Token(3), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:58384), fd: 8 }) } }
//! one connected
//! two connected
//! << hey, one?
//! << what's up?
//! << two
//! ```
//! [sending messages]: https://book.async.rs/tutorial/sending_essages.html
//! [accept loop]: ch03-02-server.rs
//! [receving messages]: ch03-03-server.rs
use async_std::io::prelude::{BufReadExt, WriteExt};
use async_std::io::BufReader;
use async_std::net::{TcpListener, TcpStream, ToSocketAddrs};
use async_std::stream::StreamExt;
use async_std::task::{self, JoinHandle};
use futures::channel::mpsc;
use std::{env, error::Error, future::Future, result, sync::Arc};

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

fn main() -> Result<()> {
    let argv: Vec<String> = env::args().collect();
    let addr = match argv.len() {
        2 => &argv[1],
        _ => "localhost:8034",
    };
    task::block_on(listener(addr))
}

/// `listener()` listens on the specified `addr` [ToSocketAddrs] trait object
/// and spawns `server()` task for each incoming connections.
///
/// [ToSocketAddrs]: https://docs.rs/async-std/1.4.0/async_std/net/trait.ToSocketAddrs.html
async fn listener(addr: impl ToSocketAddrs) -> Result<()> {
    let l = TcpListener::bind(addr).await?;
    println!("listening on {:?}", l);
    while let Some(s) = l.incoming().next().await {
        match s {
            Err(err) => eprintln!("accept error: {:?}", err),
            Ok(s) => {
                spawn(server(s));
            }
        }
    }
    Ok(())
}

/// `server()` serves the individual connection over `s` [TcpStream].
///
/// [TcpStream]: https://docs.rs/async-std/1.4.0/async_std/net/struct.TcpStream.html
async fn server(s: TcpStream) -> Result<()> {
    let s = Arc::new(s);
    eprintln!("handling {:?}", s);
    let (tx, rx) = mpsc::unbounded();
    // Create a responder task to response back to this client.
    spawn(responder(rx, s.clone()));
    let mut lines = BufReader::new(&*s).lines();
    // Get the first line as the handle name.
    let name = match lines.next().await {
        None => Err("premature client closure")?,
        Some(name) => name?,
    };
    println!("{} connected", name);
    while let Some(line) = lines.next().await {
        let line = line?;
        // split through ':' separator.
        let (dest, msg) = match line.find(':') {
            Some(idx) => (&line[..idx], &line[idx + 1..]),
            None => {
                eprintln!("wrong format from {:?}", s);
                continue;
            }
        };
        let _dest: Vec<String> = dest
            .split(',')
            .map(|name| name.trim().to_string())
            .collect();
        let msg = msg.trim().to_string();
        // Let's just response back to the original sender for now.
        unicast(&tx, msg)?;
    }
    eprintln!("handled {:?}", s);
    tx.close_channel();
    Ok(())
}

/// `unicast()` sends `msg` message back to the originator.
fn unicast(tx: &Sender<String>, msg: String) -> Result<()> {
    // Echoing back to the sender through the unbound channel for now.
    match tx.unbounded_send(msg) {
        Err(err) => Err(format!("cannot send a message over the channel: {:?}", err))?,
        Ok(_) => Ok(()),
    }
}

/// `responder()` wait for the message over the `rx` [Receiver] channel
/// and send it to the `s` [TcpStream].
///
/// [Receiver]: type.Receiver.html
async fn responder(mut rx: Receiver<String>, s: Arc<TcpStream>) -> Result<()> {
    let mut s = &*s;
    while let Some(msg) = rx.next().await {
        s.write_all(msg.as_bytes()).await?;
        eprintln!("<< {}", msg);
    }
    eprintln!("finish responding on {:?}", s);
    Ok(())
}

// `spawn()` spawns a task for the passed `f` [Future] and
// print the error message over stderr if `f` returns error.
//
// [Future]: https://doc.rust-lang.org/stable/std/future/trait.Future.html
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
