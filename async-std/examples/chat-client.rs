//! Implementing a [Chat Client]
//!
//! [chat client]: https://book.async.rs/tutorial/implementing_a_client.html
use async_std::io;
use async_std::task;
use async_std_book::Client;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() -> Result<(), Error> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("[::1]:8000"));
    task::block_on(Client::new(addr).run(io::stdin(), io::stderr()))
}
