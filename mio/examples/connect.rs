//! [mio] connection example
//!
//! [mio]: https://lib.rs/mio/latest/mio/index.html
use std::{
    error::Error,
    net::{TcpListener, SocketAddr},
};
use mio::{
    Events, Poll, PollOpt, Ready, Token,
    net::TcpStream,
};

fn main() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "127.0.0.1:0".parse()?;
    let server = TcpListener::bind(&addr)?;
    let poll = Poll::new()?;
    let mut events = Events::with_capacity(1024);
    let stream = TcpStream::connect(&server.local_addr()?)?;

    poll.register(&stream, Token(0), Ready::readable() | Ready::writable(), PollOpt::edge())?;
    loop {
        poll.poll(&mut events, None)?;
        for event in &events {
            match event.token() {
                Token(0) => {
                    if event.readiness().is_writable() {
                        println!("{:?} is writable", &stream);
                    }
                    if event.readiness().is_readable() {
                        println!("{:?} is readable", &stream);
                    }
                }
                Token(token) => println!("unknown token: {}", token),
            }
        }
    }
}
