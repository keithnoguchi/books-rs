//! [Receiving messages] chat server example
//!
//! This is the second part of the chat server example, building
//! on top of the [accept loop] example.
//!
//! # Examples
//!
//! ```sh
//! $  c run --example ch03-03-server -- localhost:8000
//! Finished dev [unoptimized + debuginfo] target(s) in 0.03s
//! Running `/home/kei/git/books-rs/target/debug/examples/ch03-03-server 'localhost:8000'`
//! listen on TcpListener { watcher: Watcher { entry: Entry { token: Token(1), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpListener { addr: V6([::1]:8000), fd: 3 }) } }
//! start serving to TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:57914), fd: 7 }) } }
//! start serving to TcpStream { watcher: Watcher { entry: Entry { token: Token(3), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:57916), fd: 8 }) } }
//! name="one"
//! name="two"
//! {msg: "hey, two", dests: ["two"]}
//! {msg: "hello one", dests: ["one"]}
//! {msg: "bye", dests: ["two"]}
//! {msg: "goodbye", dests: ["two"]}
//! finish serving to TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:57914), fd: 7 }) } }
//! finish serving to TcpStream { watcher: Watcher { entry: Entry { token: Token(3), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:57916), fd: 8 }) } }
//! ```
//! [receiving messages]: https://book.async.rs/tutorial/receiving_messages.html
//! [accept loop]: ch03-02-server.rs
use async_std::io::{prelude::BufReadExt, BufReader};
use async_std::net::{TcpListener, TcpStream, ToSocketAddrs};
use async_std::stream::StreamExt;
use async_std::task::{self, JoinHandle};
use std::{env, error::Error, future::Future};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

fn main() -> Result<()> {
    let argv: Vec<String> = env::args().collect();
    let addr = match argv.len() {
        2 => &argv[1],
        _ => "localhost:8033",
    };
    task::block_on(listener(addr))
}

/// `listener()` listens on the `addr` [ToSocketAddrs] trait and handles
/// the incoming request.
///
/// [ToSocketAddrs]: https://docs.rs/async-std/1.4.0/async_std/net/trait.ToSocketAddrs.html
async fn listener(addr: impl ToSocketAddrs) -> Result<()> {
    let l = TcpListener::bind(addr).await?;
    eprintln!("listen on {:?}", l);
    while let Some(s) = l.incoming().next().await {
        match s {
            Err(err) => eprintln!("accept error: {:?}", err),
            Ok(s) => {
                spawn(receiver(s));
            }
        }
    }
    Ok(())
}

/// `receiver()` receives messages on `s` [TcpStream].
///
/// [TcpStream]: https://docs.rs/async-std/1.4.0/async_std/net/struct.TcpStream.html
async fn receiver(s: TcpStream) -> Result<()> {
    eprintln!("start serving to {:?}", s);
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
        None => Err("peer disconnect immediately")?,
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
        println!("{{msg: {:?}, dests: {:?}}}", msg, dest);
    }
    println!("finish serving to {:?}", s);
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
