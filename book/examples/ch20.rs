//! Final Project: Building a Multithreaded Web Server
use std::net::TcpListener;
use the_book::ch20;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _pool = ch20::ThreadPool::new(2);
    let l = TcpListener::bind("localhost:7878")?;
    for s in l.incoming() {
        match s {
            Err(e) => return Err(Box::new(e)),
            Ok(_s) => println!("got the connection!"),
        }
    }
    Ok(())
}
