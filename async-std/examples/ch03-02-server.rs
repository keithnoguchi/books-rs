//! Writing an [accept loop] chat server example
//!
//! This is the first part of the chat server.
//!
//! # Examples
//!
//! ```sh
//! $ cargo run --example ch03-02-server [::1]:8000
//! Compiling async-std-book v0.1.0 (/home/kei/git/books-rs/async-std)
//! Finished dev [unoptimized + debuginfo] target(s) in 0.87s
//! Running `target/debug/examples/ch03-02-server '[::1]:8000'`
//! [server] listen on TcpListener { watcher: Watcher { entry: Entry { token: Token(1), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpListener { addr: V6([::1]:8000), fd: 3 }) } }
//! accepted the connection over Ok(TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:51966), fd: 7 }) } })
//! accepted the connection over Ok(TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: [] }, writers: Mutex { data: [] } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:51968), fd: 7 }) } })
//! ...
//! ```
//! [accept loop]: https://book.async.rs/tutorial/accept_loop.html
use async_std::net::{TcpListener, ToSocketAddrs};
use async_std::stream::StreamExt;
use async_std::task;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("localhost:8032"));
    task::block_on(server(&addr))
}

/// `server()` listens on the `addr` ToSocketAddr trait object
/// and accepts the TCP connection from the client.
async fn server(addr: impl ToSocketAddrs) -> Result<()> {
    let s = TcpListener::bind(addr).await?;
    eprintln!("[server] listen on {:?}", s);
    while let Some(s) = s.incoming().next().await {
        eprintln!("accepted the connection over {:?}", s);
    }
    Ok(())
}
