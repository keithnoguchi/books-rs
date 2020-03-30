//! Building multithreaded web server
use std::{
    error, fs,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for s in listener.incoming() {
        match s {
            Err(err) => eprintln!("connection error: {}", err),
            Ok(s) => {
                if let Err(err) = handle_connection(s) {
                    eprintln!("stream error: {}", err);
                }
            }
        };
    }
    Ok(())
}

fn handle_connection(mut s: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];
    let mut n = s.read(&mut buf)?;
    println!("{}=read({})", n, String::from_utf8_lossy(&buf[0..n]));
    let get = b"GET / HTTP/1.1\r\n";
    let (status, filename) = if buf.starts_with(get) {
        ("200 OK", "examples/hello.html")
    } else {
        ("404 NOT FOUND", "examples/404.html")
    };
    let body = fs::read_to_string(filename)?;
    let resp = format!("HTTP/1.1 {}\r\n\r\n{}", status, body);
    n = s.write(resp.as_bytes())?;
    s.flush()?;
    println!("{}=write({})", n, resp);
    Ok(())
}
