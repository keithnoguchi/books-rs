//! [mio] examples
//!
//! [mio]: https://lib.rs/mio/latest/mio/index.html
//!
//! # Examples
//!
//! ```no_run
//! use std::net::{SocketAddr, TcpListener};
//! use mio::{
//!     Events, Poll, PollOpt, Ready, Token,
//!     net::TcpStream,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let addr: SocketAddr = "127.0.0.1:0".parse()?;
//! let server = TcpListener::bind(&addr)?;
//! let poll = Poll::new()?;
//! let mut events = Events::with_capacity(1024);
//! let stream = TcpStream::connect(&server.local_addr()?)?;
//! poll.register(&stream, Token(0), Ready::readable() | Ready::writable(), PollOpt::edge())?;
//! loop {
//!     poll.poll(&mut events, None)?;
//!     for event in &events {
//!         if event.token() == Token(0) && event.readiness().is_writable() {
//!             println!("{:?} is writable", &stream);
//!             break;
//!         }
//!     }
//! }
//! #     Ok(())
//! # }
//! ```
pub mod registry;
