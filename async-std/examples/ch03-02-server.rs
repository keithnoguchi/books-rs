//! [Chat] server
//!
//! [chat]: https://book.async.rs/tutorial/index.html
use async_std::{net::ToSocketAddrs, task};
use std::{env, error, result};

type Result<T> = result::Result<T, Box<dyn error::Error + Send + Sync>>;

fn main() -> Result<()> {
    // Simple [argument parsing].
    //
    // [argument parsing]: https://doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html
    let argv: Vec<String> = env::args().collect();
    let addr = match argv.len() {
        2 => &argv[1],
        _ => "localhost:3000",
    };
    task::block_on(server(addr))
}

async fn server(addr: impl ToSocketAddrs) -> Result<()> {
    let addr = addr.to_socket_addrs().await?.next();
    println!("{:?}", addr);
    Ok(())
}
