//! [mio] examples
//!
//! [mio]: https://lib.rs/mio/latest/mio/index.html
//!
//! # Examples
//!
//! ```
//! use std::net::{SocketAddr, TcpListener};
//!
//! use mio::{net::TcpStream, Events, Interest, Poll, Token};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Dummy server.
//! let addr: SocketAddr = "127.0.0.1:0".parse()?;
//! let server = TcpListener::bind(&addr)?;
//!
//! // Client event loop.
//! let mut poll = Poll::new()?;
//! let mut stream = TcpStream::connect(server.local_addr()?)?;
//! poll.registry()
//!     .register(&mut stream, Token(0), Interest::READABLE | Interest::WRITABLE)?;
//!
//! let mut events = Events::with_capacity(1024);
//! loop {
//!     poll.poll(&mut events, None)?;
//!     for event in &events {
//!         match event.token() {
//!             Token(0) => {
//!                 if event.is_writable() {
//!                     println!("{:?} is writable", &stream);
//!                     return Ok(());
//!                 }
//!                 if event.is_readable() {
//!                     println!("{:?} is readable", &stream);
//!                 }
//!             }
//!             Token(token) => println!("unknown token: {}", token),
//!         }
//!     }
//! }
//! #     Ok(())
//! # }
//! ```
pub mod registry;
