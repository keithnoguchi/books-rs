//! Struct [`mio::Registry`] example
//!
//! [`mio::registry`]: https://lib.rs/mio/latest/mio/struct.Registry.html
//!
//! # Examples
//!
//! ```
//! use std::{
//!     net::{SocketAddr, TcpListener},
//!     time::{Duration, Instant},
//! };
//! use mio::{
//!     {Events, Poll, Token},
//!     net::TcpStream,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut poll = Poll::new()?;
//!
//! let address: SocketAddr = "127.0.0.1:0".parse()?;
//! let listener = TcpListener::bind(address)?;
//! let mut socket = TcpStream::connect(&listener.local_addr()?);
//! #     Ok(())
//! # }
