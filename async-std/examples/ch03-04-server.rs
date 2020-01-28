//! [Sending messages] chat server example
//!
//! This is the third part of the chat server example, building
//! on top of the [accept loop] and the [receiving messages] examples.
//!
//! # Examples
//!
//! ```sh
//! $ cargo run --example ch03-04-server [::1]:8000
//! Compiling async-std-book v0.1.0 (/home/kei/git/books-rs/async-std)
//! Finished dev [unoptimized + debuginfo] target(s) in 1.42s
//! Running `target/debug/examples/ch03-04-server '[::1]:8000'`
//! [server] listening on TcpListener { watcher: Watcher { entry: Entry { token: Token(1), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpListener { addr: V6([::1]:8000), fd: 3 }) } }
//! [reader] handling TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:52010), fd: 7 }) } }
//! [reader] handling TcpStream { watcher: Watcher { entry: Entry { token: Token(3), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:52012), fd: 8 }) } }
//! uno connected
//! due connected
//! << hey
//! [reader] wrong format from TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:52010), fd: 7 }) } }
//! << yo
//! [reader] handled TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:52010), fd: 7 }) } }
//! [writer] finish responding on TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:52010), fd: 7 }) } }
//! [reader] handled TcpStream { watcher: Watcher { entry: Entry { token: Token(3), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:52012), fd: 8 }) } }
//! [writer] finish responding on TcpStream { watcher: Watcher { entry: Entry { token: Token(3), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:52012), fd: 8 }) } }
//! ...
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
use futures::sink::SinkExt;
use std::{error::Error, future::Future, result, sync::Arc};

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

fn main() -> Result<()> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("localhost:8034"));
    task::block_on(server(&addr))
}

/// `server()` listens on the specified `addr` [ToSocketAddrs] trait object
/// and spawns `reader()` task for each incoming connections.
///
/// [ToSocketAddrs]: https://docs.rs/async-std/1.4.0/async_std/net/trait.ToSocketAddrs.html
async fn server(addr: impl ToSocketAddrs) -> Result<()> {
    let s = TcpListener::bind(addr).await?;
    println!("[server] listening on {:?}", s);
    while let Some(s) = s.incoming().next().await {
        match s {
            Err(err) => eprintln!("[server] accept error: {:?}", err),
            Ok(s) => {
                spawn(reader(s));
            }
        }
    }
    Ok(())
}

/// `reader()` serves the individual connection over `s` [TcpStream].
///
/// [TcpStream]: https://docs.rs/async-std/1.4.0/async_std/net/struct.TcpStream.html
async fn reader(s: TcpStream) -> Result<()> {
    let s = Arc::new(s);
    eprintln!("[reader] handling {:?}", s);
    let (mut tx, rx) = mpsc::unbounded();
    // Create a writer task to response back to the client.
    spawn(writer(rx, s.clone()));
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
                eprintln!("[reader] wrong format from {:?}", s);
                continue;
            }
        };
        let _dest: Vec<String> = dest
            .split(',')
            .map(|name| name.trim().to_string())
            .collect();
        let msg = msg.trim().to_string();
        // Let's just forward the message to the writer.
        forwarder(&tx, msg).await?;
    }
    eprintln!("[reader] handled {:?}", s);
    tx.close().await?;
    Ok(())
}

/// `forwarder()` forwards `msg` message to the `writer()`
/// over the unbounded channel.
async fn forwarder(mut tx: &Sender<String>, msg: String) -> Result<()> {
    // Echoing back to the sender through the unbound channel for now.
    match tx.send(msg).await {
        Err(err) => Err(format!(
            "[forwarder] cannot send a message over the channel: {:?}",
            err
        ))?,
        Ok(_) => Ok(()),
    }
}

/// `writer()` wait for the message over the `rx` [Receiver] channel
/// and send it to the `s` [TcpStream].
///
/// [Receiver]: type.Receiver.html
async fn writer(mut rx: Receiver<String>, s: Arc<TcpStream>) -> Result<()> {
    let mut s = &*s;
    while let Some(msg) = rx.next().await {
        s.write_all(msg.as_bytes()).await?;
        eprintln!("<< {}", msg);
    }
    eprintln!("[writer] finish responding on {:?}", s);
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
