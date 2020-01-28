//! Turorial: [Writing a Chat]
//!
//! [writing a chat]: https://book.async.rs/tutorial/index.html
use async_std::task;
use async_std_book::Server;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() -> Result<(), Error> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("[::1]:8000"));
    task::block_on(Server::new(addr).run())
}
