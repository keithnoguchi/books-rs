//! [Graceful Shutdown and Cleanup]
//!
//! [graceful shutdown and cleanup]: https://doc.rust-lang.org/book/ch20-03-graceful-shutdown-and-cleanup.html
use std::{
    env,
    error::Error,
    fmt::Debug,
    fs,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    thread,
    time::Duration,
};
use tracing::{error, info, instrument, warn};

use the_book::ch20::ThreadPool;

static SERVER: &str = "127.0.0.1:7880";

#[instrument]
fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let mut args = env::args().skip(1);
    let addr = args.next().unwrap_or(SERVER.to_string());
    let count = args
        .next()
        .map(|num| num.parse().unwrap_or(num_cpus::get()))
        .unwrap_or(num_cpus::get());
    let max = args
        .next()
        .map(|num| num.parse().unwrap_or(std::usize::MAX))
        .unwrap_or(std::usize::MAX);
    server(addr, count, max)
}

#[instrument]
fn server<A: ToSocketAddrs + Debug>(
    addr: A,
    count: usize,
    max: usize,
) -> Result<(), Box<dyn Error>> {
    let pool = ThreadPool::new(count);
    let server = TcpListener::bind(addr)?;
    info!("listen on {}", server.local_addr()?);
    for s in server.incoming().take(max) {
        match s {
            Err(err) => warn!("accept error: {}", err),
            Ok(s) => {
                if let Err(err) = pool.execute(|| handler(s)) {
                    error!("pool execution error: {}", err);
                }
            }
        }
    }
    Ok(())
}

#[instrument]
fn handler(mut s: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];
    s.read(&mut buf)?;
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
    s.write_fmt(format_args!("HTTP/1.1 {}\r\n\r\n{}", status, body))?;
    s.flush()
}
