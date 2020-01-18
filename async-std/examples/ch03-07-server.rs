//! [Clean Shutdown]
//!
//! Let's cleanup all the tasks cleanly.  This is the continuation of the chat
//! server example, based off of the [accept loop], [receiving messages],
//! [sending messages], and [connecting readers and writers].
//!
//! # Examples
//!
//! [clean shutdown]: https://book.async.rs/tutorial/clean_shutdown.html
//! [accept loop]: ch03-02-server.rs
//! [receiving messages]: ch03-03-server.rs
//! [sending messages]: ch03-04-server.rs
//! [connecting readers and writers]: ch03-05-server.rs
use async_std::net::TcpListener;
use async_std::task;
use async_std::stream::StreamExt;
use std::{env, error::Error, result};

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

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
    eprintln!("[supervisor] starting");
    let server = task::spawn(server(addr));
    server.await?;
    eprintln!("[supervisor] done");
    Ok(())
}

/// 'server()` listens on the `addr` address and create `reader()` task
/// for servicing the clients.
async fn server(addr: String) -> Result<()> {
    let s = TcpListener::bind(&addr).await?;
    while let Some(s) = s.incoming().next().await {
        eprintln!("[server] got connection over {:?}", s);
    }
    Ok(())
}
