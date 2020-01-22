//! [Receiving messages] chat server example
//!
//! This is the second part of the chat server example, building
//! on top of the [accept loop] example.
//!
//! # Examples
//!
//! ```sh
//! $ cargo run --example ch03-03-server [::1]:8000
//! Finished dev [unoptimized + debuginfo] target(s) in 0.03s
//! Running `target/debug/examples/ch03-03-server '[::1]:8000'`
//! [server] listen on TcpListener { watcher: Watcher { entry: Entry { token: Token(1), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpListener { addr: V6([::1]:8000), fd: 3 }) } }
//! [reader] start serving to TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:51990), fd: 7 }) } }
//! [reader] start serving to TcpStream { watcher: Watcher { entry: Entry { token: Token(3), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:51992), fd: 8 }) } }
//! name="one"
//! name="two"
//! [reader] {msg: "hey", dests: ["two"]}
//! [reader] {msg: "yo", dests: ["one"]}
//! [reader] finish serving TcpStream { watcher: Watcher { entry: Entry { token: Token(3), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:51992), fd: 8 }) } }
//!  [reader] finish serving TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:51990), fd: 7 }) } }
//!  ...
//! ```
//! [receiving messages]: https://book.async.rs/tutorial/receiving_messages.html
//! [accept loop]: ch03-02-server.rs
use async_std::io::{prelude::BufReadExt, BufReader};
use async_std::net::{TcpListener, TcpStream, ToSocketAddrs};
use async_std::stream::StreamExt;
use async_std::task::{self, JoinHandle};
use std::future::Future;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let addr = std::env::args()
        .skip(1)
        .next()
        .unwrap_or(String::from("localhost:8033"));
    task::block_on(server(&addr))
}

/// `server()` listens on the `addr` [ToSocketAddrs] trait and handles
/// the incoming request.
///
/// [ToSocketAddrs]: https://docs.rs/async-std/1.4.0/async_std/net/trait.ToSocketAddrs.html
async fn server(addr: impl ToSocketAddrs) -> Result<()> {
    let s = TcpListener::bind(addr).await?;
    eprintln!("[server] listen on {:?}", s);
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

/// `reader()` reads messages on `s` [TcpStream].
///
/// [TcpStream]: https://docs.rs/async-std/1.4.0/async_std/net/struct.TcpStream.html
async fn reader(s: TcpStream) -> Result<()> {
    eprintln!("[reader] start serving to {:?}", s);
    // [BufReader] is async ready, so that we can get the String Stream
    // through `lines()` method, just like `std::io::BufReader`.
    //
    // [bufreader]: https://docs.rs/async-std/1.4.0/async_std/io/struct.BufReader.html
    let reader = BufReader::new(&s);
    let mut lines = reader.lines();

    // Asynchronously wait for the lines over the client [TcpStream].
    //
    // [tcpstream]: https://docs.rs/async-std/1.4.0/async_std/net/struct.TcpStream.html
    let name = match lines.next().await {
        // We can returns the String as a error message, as String
        // supports Box<dyn Error + Send + Sync>.
        None => Err("[reader] peer disconnect immediately")?,
        // We get Option<Result<T, E>> over [line.next()].await.
        //
        // [lines.next()]: https://docs.rs/async-std/1.4.0/async_std/io/prelude/trait.BufReadExt.html#method.lines
        Some(line) => line?,
    };
    println!("name={:?}", name);

    while let Some(line) = lines.next().await {
        let line = line?;
        let (dest, msg) = match line.find(':') {
            None => continue,
            Some(idx) => (&line[..idx], &line[idx + 1..]),
        };
        // Get the destination handles separated by ','.
        let dest: Vec<String> = dest
            .split(',')
            .map(|name| name.trim().to_string())
            .collect();
        let msg = msg.trim().to_string();
        println!("[reader] {{msg: {:?}, dests: {:?}}}", msg, dest);
    }
    println!("[reader] finish serving {:?}", s);
    Ok(())
}

/// `spawn()` spawns the task with the passed `Future` and
/// print the error to the stderr in case the it returns error.
#[inline]
fn spawn<F>(f: F) -> JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + Sync + 'static,
{
    task::spawn(async move {
        if let Err(err) = f.await {
            eprintln!("{}", err);
        }
    })
}
