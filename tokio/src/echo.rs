// SPDX-License-Identifier: GPL-2.0
use futures::{Future, Poll, Stream};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

pub enum Handler {
    Greeting(TcpStream),
    Handling(TcpStream),
}

impl Future for Handler {
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(futures::Async::NotReady)
    }
}

pub fn server2(addr: &SocketAddr) -> Box<dyn Future<Item = (), Error = ()> + Send> {
    const NAME: &str = "echo::server2";
    let l = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(err) => {
            eprintln!("[{}]: cannot bind: {}", NAME, err);
            return Box::new(futures::future::err(()));
        }
    };
    // https://tokio.rs/docs/going-deeper/returning/
    let s = l
        .incoming()
        .map_err(|err| eprintln!("[{}]: cannot accept: {}", NAME, err))
        .for_each(|_sock| Ok(()));
    Box::new(s)
}

pub fn server1(addr: &SocketAddr) -> impl Future<Item = (), Error = ()> {
    const NAME: &str = "echo::server1";
    let l = TcpListener::bind(addr)
        .unwrap_or_else(|err| panic!(format!("[{}]: cannot bind: {}", NAME, err)));
    l.incoming()
        .map_err(|err| eprintln!("[{}]: accept failed: {}", NAME, err))
        .for_each(|sock| {
            tokio::spawn(
                greeting(sock)
                    .map_err(|err| eprintln!("[{}]: greeting error: {}", NAME, err))
                    .map(|sock| {
                        tokio::spawn(
                            handle(sock)
                                .map_err(|err| eprintln!("[{}]: handle error: {}", NAME, err))
                                .map(|sock| match sock.peer_addr() {
                                    Ok(peer) => {
                                        println!("[{}]: taken care of {}", NAME, peer);
                                    }
                                    Err(err) => {
                                        eprintln!("[{}]: peer_addr(): {:?}", NAME, err);
                                    }
                                }),
                        );
                    }),
            );
            Ok(())
        })
}

fn greeting(sock: TcpStream) -> impl Future<Item = TcpStream, Error = tokio::io::Error> {
    tokio::io::write_all(sock, "Howdy!\n").map(|(sock, _)| sock)
}

fn handle(sock: TcpStream) -> impl Future<Item = TcpStream, Error = tokio::io::Error> {
    use tokio::io::AsyncRead;
    let (reader, writer) = sock.split();
    tokio::io::copy(reader, writer).map(|(_, r, w)| r.unsplit(w))
}
