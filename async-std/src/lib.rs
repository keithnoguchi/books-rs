//! [async-std] examples
//!
//! # Examples
//!
//! Chat client
//!
//! ```no_run
//! use async_std::io;
//! use async_std::task;
//! use async_std_book::Client;
//!
//! type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
//!
//! fn main() -> Result<(), Error> {
//!     let addr = std::env::args()
//!         .skip(1)
//!         .next()
//!         .unwrap_or(String::from("[::1]:8000"));
//!     task::block_on(Client::new(addr).run(io::stdin(), io::stderr()))
//! }
//! ```
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
#![recursion_limit = "1024"]
pub use client::Client;
pub use server::Server;
pub mod client;
pub mod server;

// crate local alias types.
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, Error>;
