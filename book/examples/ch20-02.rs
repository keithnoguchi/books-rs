//! Turning Our Single-Threaded Server into a [Multithreaded] Server
//!
//! [multithreaded]: https://doc.rust-lang.org/book/ch20-02-multithreaded.html
use std::{
    env, error, fs,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use tracing::{event, instrument, Level};
use tracing_subscriber::FmtSubscriber;

use the_book::ch20::ThreadPool;

#[instrument]
fn main() -> Result<(), Box<dyn error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::new())?;
    let mut args = env::args().skip(1);
    let addr = args.next().unwrap_or(String::from("127.0.0.1:7879"));
    let count = args.next().map(|val| val.parse().unwrap_or(5)).unwrap_or(5);
    let listener = TcpListener::bind(addr)?;
    event!(Level::INFO, "listening on {}", listener.local_addr()?);
    let pool = ThreadPool::new(4, count);
    let mut i = count;
    for s in listener.incoming() {
        match s {
            Err(err) => eprintln!("accept error: {}", err),
            Ok(s) => pool.execute(|| handler(s)),
        };
        i -= 1;
        if i == 0 {
            break;
        }
    }
    Ok(())
}

#[instrument]
fn handler(mut s: TcpStream) -> io::Result<()> {
    let peer = s.peer_addr()?;
    event!(Level::INFO, "connection from: {}", peer);
    let mut buf = [0; 512];
    let n = s.read(&mut buf)?;
    event!(Level::INFO, "{}", String::from_utf8_lossy(&buf[..n]));
    let get = b"GET / HTTP/1.1";
    let sleep = b"GET /sleep HTTP/1.1";
    let (status, file) = if buf.starts_with(get) {
        ("200 OK", "hello.html")
    } else if buf.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("200 OK", "hello.html")
    } else {
        ("404 NOT FOUND", "404.html")
    };
    let body = fs::read_to_string(format!("examples/{}", file))?;
    let resp = format!("HTTP/1.1 {}\r\n\r\n{}", status, body);
    s.write(resp.as_bytes())?;
    s.flush()
}
