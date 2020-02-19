//! [mio] connection example
//!
//! [mio]: https://lib.rs/mio/latest/mio/index.html
use std::{
    error::Error,
    net::{TcpListener, SocketAddr},
    time::Duration,
};

use mio::{net::TcpStream, Events, Interest, Poll, Token};

fn main() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "[::1]:0".parse()?;
    let server = TcpListener::bind(&addr)?;

    let mut events = Events::with_capacity(1024);
    let mut stream = TcpStream::connect(server.local_addr()?)?;

    let mut poll = Poll::new()?;
    poll.registry()
        .register(&mut stream, Token(0), Interest::READABLE | Interest::WRITABLE)?;
    loop {
        poll.poll(&mut events, Some(Duration::from_millis(500)))?;
        if events.is_empty() {
            println!("timed out");
            return Ok(());
        }
        for event in &events {
            match event.token() {
                Token(0) => {
                    if event.is_writable() {
                        println!("{:?} is writable", &stream);
                    }
                    if event.is_readable() {
                        println!("{:?} is readable", &stream);
                    }
                }
                Token(token) => println!("unknown token: {}", token),
            }
        }
    }
}
