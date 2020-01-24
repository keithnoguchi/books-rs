//! [async-std] examples
//!
//! # Examples
//!
//! Chat server
//!
//! ```no_run
//! use async_std::task;
//! use async_std_book::Server;
//!
//! type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
//!
//! fn main() -> Result<(), Error> {
//!     let addr = std::env::args()
//!         .skip(1)
//!         .next()
//!         .unwrap_or(String::from("[::1]:8000"));
//!     task::block_on(Server::new(addr).run())
//! }
//! ```
//! [async-std]: https://book.async.rs
pub use server::Server;
pub mod server;
