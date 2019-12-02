// SPDX-License-Identifier: GPL-2.0
use futures::{Async, Future, Poll};
use std::net::SocketAddr;
use tokio::net::tcp::{ConnectFuture, TcpStream};

#[derive(Debug)]
pub struct Doubler<T> {
    inner: T,
}

#[allow(dead_code)]
pub fn double<T>(inner: T) -> Doubler<T> {
    Doubler { inner }
}

impl<T> Future for Doubler<T>
where
    T: Future<Item = usize>,
{
    type Item = usize;
    type Error = T::Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.inner.poll()? {
            Async::Ready(v) => Ok(Async::Ready(v * 2)),
            Async::NotReady => Ok(Async::NotReady),
        }
    }
}

enum State {
    #[allow(dead_code)]
    Resolving(ResolveFuture),
    Connecting(ConnectFuture),
}

pub struct ResolveAndConnect {
    state: State,
}

impl Future for ResolveAndConnect {
    type Item = TcpStream;
    type Error = std::io::Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let addr = match self.state {
                State::Resolving(ref mut fut) => futures::try_ready!(fut.poll()),
                State::Connecting(ref mut fut) => return fut.poll(),
            };
            let connecting = TcpStream::connect(&addr);
            self.state = State::Connecting(connecting);
        }
    }
}

#[allow(dead_code)]
pub fn resolve_and_connect(host: &'static str) -> ResolveAndConnect {
    let state = State::Resolving(resolve(host));
    ResolveAndConnect { state }
}

struct ResolveFuture {
    host: &'static str,
}

impl Future for ResolveFuture {
    type Item = SocketAddr;
    type Error = std::io::Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        // It only support the address, e.g. "127.0.0.1", for now.
        use std::io::{self, ErrorKind};
        match self.host.parse::<SocketAddr>() {
            Ok(addr) => Ok(Async::Ready(addr)),
            Err(err) => Err(io::Error::new(ErrorKind::InvalidInput, format!("{}", err))),
        }
    }
}

fn resolve(host: &'static str) -> ResolveFuture {
    ResolveFuture { host }
}

#[cfg(test)]
mod tests {
    #[test]
    fn double_ok() {
        use futures::{Async, Future};
        struct Test {
            name: &'static str,
            data: usize,
            want: Result<Async<usize>, ()>,
        };
        let tests = [
            Test {
                name: "1usize",
                data: 1,
                want: Ok(Async::Ready(2)),
            },
            Test {
                name: "2usize",
                data: 2,
                want: Ok(Async::Ready(4)),
            },
            Test {
                name: "16usize",
                data: 16,
                want: Ok(Async::Ready(32)),
            },
            Test {
                name: "10_001usize",
                data: 10_001,
                want: Ok(Async::Ready(20_002)),
            },
        ];
        for t in &tests {
            let got = super::double(futures::future::ok::<usize, ()>(t.data)).poll();
            debug_assert_eq!(t.want, got, "{}", t.name);
        }
    }
    #[test]
    fn double_err() {
        use futures::{Async, Future};
        struct Test {
            name: &'static str,
            data: std::io::ErrorKind,
            want: Result<Async<usize>, std::io::ErrorKind>,
        };
        let tests = [
            Test {
                name: "InvalidInput",
                data: std::io::ErrorKind::InvalidInput,
                want: Err(std::io::ErrorKind::InvalidInput),
            },
            Test {
                name: "InvalidData",
                data: std::io::ErrorKind::InvalidData,
                want: Err(std::io::ErrorKind::InvalidData),
            },
        ];
        for t in &tests {
            let got =
                super::double(futures::future::err::<usize, std::io::ErrorKind>(t.data)).poll();
            debug_assert_eq!(t.want, got, "{}", t.name);
        }
    }
    #[test]
    fn resolve_ok() {
        use futures::{Async, Future};
        use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
        struct Test {
            name: &'static str,
            addr: &'static str,
            want: Async<SocketAddr>,
        };
        let tests = [
            Test {
                name: "IPv4 localhost:8080",
                addr: "127.0.0.1:8080",
                want: Async::Ready(SocketAddr::V4(SocketAddrV4::new(
                    Ipv4Addr::new(127, 0, 0, 1),
                    8080,
                ))),
            },
            Test {
                name: "IPv4 8.8.8.8:52",
                addr: "8.8.8.8:52",
                want: Async::Ready(SocketAddr::V4(SocketAddrV4::new(
                    Ipv4Addr::new(8, 8, 8, 8),
                    52,
                ))),
            },
            Test {
                name: "IPv4 1.2.3.4:56789",
                addr: "1.2.3.4:56789",
                want: Async::Ready(SocketAddr::V4(SocketAddrV4::new(
                    Ipv4Addr::new(1, 2, 3, 4),
                    56789,
                ))),
            },
            Test {
                name: "IPv6 ::1:80",
                addr: "[::1]:80",
                want: Async::Ready(SocketAddr::V6(SocketAddrV6::new(
                    Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
                    80,
                    0,
                    0,
                ))),
            },
            Test {
                name: "IPv6 2002::53:53",
                addr: "[2002::53]:53",
                want: Async::Ready(SocketAddr::V6(SocketAddrV6::new(
                    Ipv6Addr::new(0x2002, 0, 0, 0, 0, 0, 0, 0x53),
                    53,
                    0,
                    0,
                ))),
            },
            Test {
                name: "IPv6 2002::1:80",
                addr: "[2002::1]:80",
                want: Async::Ready(SocketAddr::V6(SocketAddrV6::new(
                    Ipv6Addr::new(0x2002, 0, 0, 0, 0, 0, 0, 1),
                    80,
                    0,
                    0,
                ))),
            },
        ];
        for t in &tests {
            match super::resolve(t.addr).poll() {
                Ok(got) => debug_assert_eq!(t.want, got, "{}", t.name),
                Err(err) => panic!("{}: {}", t.name, err),
            }
        }
    }
}
