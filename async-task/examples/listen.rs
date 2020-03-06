//! TCP server example
//!
//! # Examples
//!
//! Default 5sec timeout
//!
//! ```
//! $ cargo run --example listen localhost:8000 5
//! Finished dev [unoptimized + debuginfo] target(s) in 0.03s
//! Running `/home/kei/git/books-rs/target/debug/examples/listen 'localhost:8000' 5`
//! listening on [::1]:8000 with 5s timeout
//! (TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: Readers { ready:
//! false, wakers: [] } }, writers: Mutex { data: Writers { ready: true, wakers: [] } } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:42854), fd: 7 }) } }, V6([::1]:42854))
//! (TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: Readers { ready:
//! false, wakers: [] } }, writers: Mutex { data: Writers { ready: true, wakers: [] } } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:42856), fd: 7 }) } }, V6([::1]:42856))
//! (TcpStream { watcher: Watcher { entry: Entry { token: Token(2), readers: Mutex { data: Readers { ready:
//! false, wakers: [] } }, writers: Mutex { data: Writers { ready: true, wakers: [] } } }, source: Some(TcpStream { addr: V6([::1]:8000), peer: V6([::1]:42858), fd: 7 }) } }, V6([::1]:42858))
//! Error: Custom { kind: TimedOut, error: "future timed out" }
//! ```
use core::{
    future::Future,
    task::{Context, Poll},
    time::Duration,
};
use std::env::args;

use async_std::{
    io,
    net::{TcpListener, ToSocketAddrs},
};
use crossbeam_utils::sync::Parker;
use pin_utils::pin_mut;

type Result<R> = std::result::Result<R, Box<dyn std::error::Error + Send + Sync + 'static>>;

fn main() -> Result<()> {
    let mut args = args();
    let addr = args.nth(1).unwrap_or(String::from("localhost:8080"));
    let sec: u64 = args
        .next()
        .unwrap_or(String::from("5"))
        .parse()
        .unwrap_or(5);
    let timeout = Duration::from_secs(sec);
    block_on(listen(addr, timeout))
}

async fn listen(addr: impl ToSocketAddrs, timeout: Duration) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let addr = listener.local_addr()?;
    println!("listening on {} with {:?} timeout", addr, timeout);
    loop {
        match io::timeout(timeout, listener.accept()).await {
            Ok(client) => println!("{:?}", client),
            Err(err) => return Err(err.into()),
        }
    }
}

/// Simplest `block_on` without the reentrance check.
fn block_on<F: Future>(future: F) -> F::Output {
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = async_task::waker_fn(move || unparker.unpark());
    let cx = &mut Context::from_waker(&waker);
    pin_mut!(future);
    loop {
        match future.as_mut().poll(cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => parker.park(),
        }
    }
}
