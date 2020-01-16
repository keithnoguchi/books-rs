//! Writing an [accept loop]
//!
//! # Examples
//!
//! ```sh
//! $ cargo run --example ch03-02-server
//! Compiling async-std-book v0.1.0 (/home/kei/git/books-rs/async-std)
//! Finished dev [unoptimized + debuginfo] target(s) in 0.77s
//! Running `/home/kei/git/books-rs/target/debug/examples/ch03-02-server`
//! listening on TcpListener { watcher: Watcher { entry: Entry { token: Token(1), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpListener { addr: V6([::1]:3000), fd: 3 }) } }
//! accepted the connection with Ok(TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:3000), peer: V6([::1]:57726), fd: 7 }) } })
//! accepted the connection with Ok(TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:3000), peer: V6([::1]:57728), fd: 7 }) } })
//! ...
//! ```
//!
//! [accept loop]: https://book.async.rs/tutorial/accept_loop.html
use async_std::net::{TcpListener, ToSocketAddrs};
use async_std::stream::StreamExt;
use async_std::task;
use std::{env, error, result};

type Result<T> = result::Result<T, Box<dyn error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let argv: Vec<String> = env::args().collect();
    let addr = match argv.len() {
        2 => &argv[1],
        _ => "localhost:3000",
    };
    task::block_on(server(addr))
}

/// `server` listens on the `addr` SocketAddr and accepts the TCP connection
/// from the client.
async fn server(addr: impl ToSocketAddrs) -> Result<()> {
    let server = TcpListener::bind(addr).await?;
    eprintln!("listening on {:?}", server);
    let mut incoming = server.incoming();
    while let Some(s) = incoming.next().await {
        eprintln!("accepted the connection with {:?}", s);
    }
    Ok(())
}
