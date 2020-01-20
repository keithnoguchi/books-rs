//! [Clean Shutdown]
//!
//! Let's cleanup all the tasks cleanly.  This is the continuation of the chat
//! server example, based off of the [accept loop], [receiving messages],
//! [sending messages], and [connecting readers and writers].
//!
//! # Examples
//!
//! ```sh
//! $ cargo run --example ch03-07-server -- [::1]:8000
//! Compiling async-std-book v0.1.0 (/home/kei/git/books-rs/async-std)
//! Finished dev [unoptimized + debuginfo] target(s) in 1.15s
//! Running `/home/kei/git/books-rs/target/debug/examples/ch03-07-server '[::1]:8000'`
//! [supervisor] starting
//! [supervisor] started
//! [server] starting
//! [broker] started
//! [server] started
//! ```
//! [clean shutdown]: https://book.async.rs/tutorial/clean_shutdown.html
//! [accept loop]: ch03-02-server.rs
//! [receiving messages]: ch03-03-server.rs
//! [sending messages]: ch03-04-server.rs
//! [connecting readers and writers]: ch03-05-server.rs
use async_std::net::{TcpListener, TcpStream};
use async_std::stream::StreamExt;
use async_std::task;
use futures::channel::mpsc;
use std::{env, error::Error, result, sync::Arc};

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

/// Client activity events.
#[derive(Debug)]
enum Event {
    /// The client join event.
    _Join { name: String, stream: TcpStream },
    /// The client leave event.
    _Leave(String),
    _Message {
        from: String,
        to: Vec<String>,
        msg: String,
    },
}

fn main() -> Result<()> {
    let argv: Vec<String> = env::args().collect();
    let addr = match argv.len() {
        0..=1 => String::from("localhost:8037"),
        _ => argv[1].clone(),
    };
    task::block_on(supervisor(addr))
}

/// `supervisor()` creates `server()` and `broker()` tasks and handle the
/// cleanup process.
async fn supervisor(addr: String) -> Result<()> {
    loop {
        eprintln!("[supervisor] starting");
        let (tx, rx) = mpsc::unbounded();
        let server = task::spawn(server(Arc::new(tx), addr.clone()));
        let broker = task::spawn(broker(rx));
        eprintln!("[supervisor] started");
        server.await?;
        broker.await?;
        eprintln!("[supervisor] restarting");
    }
}

/// 'server()` listens on the `addr` address and create `reader()` task
/// for servicing the clients.
async fn server(broker: Arc<Sender<Event>>, addr: String) -> Result<()> {
    eprintln!("[server] starting");
    let s = TcpListener::bind(&addr).await?;
    eprintln!("[server] started");
    while let Some(s) = s.incoming().next().await {
        match s {
            Err(err) => eprintln!("[server] accept error: {:?}", err),
            Ok(s) => {
                task::spawn(reader(broker.clone(), s));
            }
        }
    }
    eprintln!("[server] finished");
    Ok(())
}

/// `reader()` reads a message from the client and send it to the
/// `broker()`.
async fn reader(_broker: Arc<Sender<Event>>, _s: TcpStream) -> Result<()> {
    eprintln!("[reader] starting");
    eprintln!("[reader] finished");
    Ok(())
}

/// `broker()` mediates the `reader()` and `writer()` for the chat
/// message multicasting over [TcpStream].
///
/// [TcpStream]: https://docs.rs/async-std/1.4.0/async_std/net/struct.TcpStream.html
async fn broker(mut reader: Receiver<Event>) -> Result<()> {
    eprintln!("[broker] started");
    while let Some(event) = reader.next().await {
        eprintln!("Got {:?}", event);
    }
    eprintln!("[broker] finished");
    Ok(())
}
