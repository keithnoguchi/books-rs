// SPDX-License-Identifier: GPL-2.0
use futures::{self, Future};
use std::net::SocketAddr;
use tokio::{self, net};

// https://tokio.rs/docs/getting-started/echo/
#[allow(dead_code)]
pub fn server(addr: &SocketAddr) -> impl Future<Item = (), Error = ()> {
    use futures::Stream;
    use tokio::io::AsyncRead;
    const NAME: &str = "hello::server";
    net::tcp::TcpListener::bind(addr)
        .unwrap()
        .incoming()
        .for_each(|sock| {
            println!("[{}]: connection from {:?}", NAME, sock);
            let (rx, tx) = sock.split();
            let amount = tokio::io::copy(rx, tx);
            let msg = amount.then(|ret| {
                match ret {
                    Ok((len, _, _)) => println!("[{}]: wrote {} bytes", NAME, len),
                    Err(err) => println!("[{}]: error: {}", NAME, err),
                }
                Ok(())
            });
            tokio::spawn(msg);
            Ok(())
        })
        .map_err(|err| {
            eprintln!("[{}]: accept error: {:?}", NAME, err);
        })
}

// https://tokio.rs/docs/getting-started/hello-world/
#[allow(dead_code)]
pub fn client(addr: &SocketAddr) -> impl Future<Item = (), Error = ()> {
    const NAME: &str = "hello::client";
    net::tcp::TcpStream::connect(addr)
        .and_then(|stream| {
            println!("[{}]: created stream", NAME);
            tokio::io::write_all(stream, "hello world\n").then(|ret| {
                println!("[{}]: wrote to stream; success={:?}", NAME, ret.is_ok());
                Ok(())
            })
        })
        .map_err(|e| {
            println!("connection error: {:?}", e);
        })
}

// https://tokio.rs/docs/futures/combinators/
#[allow(dead_code)]
pub fn client_and_then(addr: &SocketAddr) -> impl Future<Item = (), Error = ()> {
    const NAME: &str = "hello::client_and_then";
    net::tcp::TcpStream::connect(addr)
        .and_then(|sock| tokio::io::write_all(sock, b"hello world"))
        .map(|_| println!("[{}]: write complete", NAME))
        .map_err(|err| eprintln!("[{}]: write error: {}", NAME, err))
}

// https://tokio.rs/docs/futures/combinators/
#[allow(dead_code)]
pub fn client_and_then_and_then(addr: &SocketAddr) -> impl Future<Item = (), Error = ()> {
    const NAME: &str = "hello::client_and_then_and_then";
    net::tcp::TcpStream::connect(addr)
        .and_then(|sock| tokio::io::write_all(sock, b"hello world"))
        .and_then(|(sock, _)| tokio::io::read_exact(sock, vec![0; 10]))
        .and_then(|(_, buf)| {
            println!("[{}]: got {:?}", NAME, buf);
            Ok(())
        })
        .map_err(|err| eprintln!("[{}]: error {}", NAME, err))
}
