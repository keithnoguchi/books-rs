// SPDX-License-Identifier: GPL-2.0
use futures::{self, future, sync, Future};
use std::net::SocketAddr;
use tokio;

/// https://tokio.rs/docs/futures/spawning/
#[allow(dead_code)]
pub fn server(addr: &SocketAddr) -> impl Future<Item = (), Error = ()> {
    use futures::Stream;
    const NAME: &str = "spawn::server";
    tokio::net::tcp::TcpListener::bind(addr)
        .unwrap()
        .incoming()
        .for_each(|sock| {
            let peer = sock.peer_addr().unwrap();
            tokio::spawn({
                println!("[{}]: handling {}", NAME, sock.peer_addr().unwrap());
                tokio::io::write_all(sock, "hello world")
                    // Drop the socket
                    .map(|_| ())
                    .map_err(|err| eprintln!("[{}]: {:?}", NAME, err))
            });
            println!("[{}]: spawned {} handler", NAME, peer);
            Ok(())
        })
        .map_err(|err| eprintln!("[{}]: {}", NAME, err))
}

/// Background processing example explained in
/// https://tokio.rs/docs/futures/spawning/
#[allow(dead_code)]
pub fn background(addr: &SocketAddr) -> impl Future<Item = (), Error = ()> {
    const NAME: &str = "spawn::background";
    let addr = *addr;
    future::lazy(move || {
        use futures::{Sink, Stream};
        let (tx, rx) = sync::mpsc::channel(1_024);
        tokio::spawn(sum(rx));
        println!("[{}] listen on {:?}", NAME, addr);
        tokio::net::tcp::TcpListener::bind(&addr)
            .unwrap()
            .incoming()
            .for_each(move |sock| {
                println!("[{}]: from {}", NAME, sock.peer_addr().unwrap());
                tokio::spawn({
                    let tx = tx.clone();
                    tokio::io::read_to_end(sock, vec![])
                        .and_then(move |(_, buf)| {
                            tx.send(buf.len())
                                .map_err(|_| tokio::io::ErrorKind::Other.into())
                        })
                        .map(|_| ())
                        .map_err(|err| eprintln!("[{}]: {:?}", NAME, err))
                });
                Ok(())
            })
            .map_err(|err| eprintln!("[{}]: {:?}", NAME, err))
    })
}

fn sum(rx: sync::mpsc::Receiver<usize>) -> impl Future<Item = (), Error = ()> {
    use futures::Stream;
    const NAME: &str = "spawn::sum";
    #[derive(Eq, PartialEq)]
    enum Item {
        Value(usize),
        Tick,
        Done,
    }
    // summary interval tick(5sec).
    let tick_dur = std::time::Duration::from_secs(5);
    let interval = tokio::timer::Interval::new_interval(tick_dur)
        .map(|_| Item::Tick)
        .map_err(|err| eprintln!("[{}]: {}", NAME, err));
    // Turn the stream into a sequence of:
    // Item(Value), Item(Value), Tick, Item(Value)... Done.
    let items = rx
        .map(Item::Value)
        .chain(futures::stream::once(Ok(Item::Done)))
        .select(interval)
        .take_while(|item| future::ok(*item != Item::Done));
    // our logic future.
    items
        .fold(0, |num, item| match item {
            Item::Value(v) => future::ok(num + v),
            Item::Tick => {
                println!("[{}]: bytes read = {}", NAME, num);
                future::ok(0)
            }
            _ => unreachable!(),
        })
        .map(|_| ())
}

/// Coordinating access to a resource example explained in
/// https://tokio.rs/docs/futures/spawning/
#[allow(dead_code)]
pub fn coordinate(requesters: usize) -> impl Future<Item = (), Error = ()> {
    const NAME: &str = "spawn::coordinate";
    future::lazy(move || {
        let (tx, rx) = sync::mpsc::channel(1_024);
        for i in 0..requesters {
            tokio::spawn(ping(i, tx.clone()).and_then(move |(dur, _)| {
                println!("[{}:{}]: duration = {:?}", NAME, i, dur);
                Ok(())
            }));
        }
        tokio::spawn(pong(rx));
        Ok(())
    })
}

type Message = (usize, sync::oneshot::Sender<(usize, std::time::Duration)>);

#[allow(dead_code)]
fn ping(
    id: usize,
    tx: sync::mpsc::Sender<Message>,
) -> impl Future<Item = (usize, std::time::Duration), Error = ()> {
    use futures::Sink;
    const NAME: &str = "spawn::ping";
    let (resp_tx, resp_rx) = sync::oneshot::channel();
    tx.send((id, resp_tx))
        .map_err(|err| eprintln!("[{}]: send error: {}", NAME, err))
        .and_then(|_tx| resp_rx.map_err(|err| eprintln!("[{}] recv error: {}", NAME, err)))
}

#[allow(dead_code)]
fn pong(rx: sync::mpsc::Receiver<Message>) -> impl Future<Item = (), Error = ()> {
    use futures::Stream;
    rx.for_each(|(id, tx)| {
        let start = std::time::Instant::now();
        let rtt = start.elapsed();
        tx.send((id, rtt)).unwrap();
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn lazy() {
        struct Test {
            name: &'static str,
            count: usize,
        }
        let tests = [
            Test {
                name: "four lazy tasks",
                count: 4,
            },
            Test {
                name: "100 lazy tasks",
                count: 100,
            },
        ];
        for t in &tests {
            let name = t.name;
            let count = t.count;
            tokio::run(futures::future::lazy(move || {
                for i in 0..count {
                    tokio::spawn(futures::future::lazy(move || {
                        println!("[{}]: task #{}", name, i);
                        Ok(())
                    }));
                }
                Ok(())
            }));
        }
    }
    #[test]
    fn oneshot() {
        struct Test {
            name: &'static str,
            count: usize,
        }
        let tests = [
            Test {
                name: "single oneshot",
                count: 1,
            },
            Test {
                name: "100 oneshots",
                count: 100,
            },
            Test {
                name: "1,000 oneshots",
                count: 1_000,
            },
        ];
        for t in &tests {
            use futures::future::lazy;
            let name = t.name;
            let count = t.count;
            tokio::run(lazy(move || {
                use futures::Future;
                for _ in 0..count {
                    let (tx, rx) = futures::sync::oneshot::channel();
                    tokio::spawn(lazy(move || {
                        tx.send(format!("{}", name))
                            .map_err(move |err| panic!("[{}] error: {}", name, err))
                    }));
                    tokio::spawn(lazy(move || {
                        rx.and_then(|msg| {
                            println!("[{}] got it!", msg);
                            Ok(())
                        })
                        .map_err(move |err| panic!("[{}] error: {}", name, err))
                    }));
                }
                Ok(())
            }))
        }
    }
    #[test]
    fn mpsc() {
        use futures::future::lazy;
        tokio::run(lazy(|| {
            use futures::{future::Future, sink::Sink, stream, Stream};
            let (tx, rx) = futures::sync::mpsc::channel(1_024);
            tokio::spawn({
                stream::iter_ok(0..10)
                    .fold(tx, |tx, i| {
                        tx.send(format!("message {} from spawned task", i))
                            .map_err(|err| eprintln!("error = {}", err))
                    })
                    .map(|_| ())
            });
            rx.for_each(|msg| {
                println!("Got {}", msg);
                Ok(())
            })
        }));
    }
    #[test]
    fn sum() {
        #[derive(Clone)]
        struct Test {
            name: &'static str,
            bufsiz: usize,
            producers: usize,
            data: usize,
        }
        let tests = [
            Test {
                name: "one producer on one byte channel buffer",
                bufsiz: 1,
                producers: 1,
                data: 256,
            },
            Test {
                name: "one producer on two bytes channel buffer",
                bufsiz: 2,
                producers: 1,
                data: 256,
            },
            Test {
                name: "one producer on 512 bytes channel buffer",
                bufsiz: 512,
                producers: 1,
                data: 256,
            },
            Test {
                name: "one producer on 1,024 bytes channel buffer",
                bufsiz: 1_024,
                producers: 1,
                data: 256,
            },
            Test {
                name: "one producer on 4K bytes channel buffer",
                bufsiz: 4_096,
                producers: 1,
                data: 256,
            },
            Test {
                name: "two producers on one byte channel buffer",
                bufsiz: 1,
                producers: 2,
                data: 256,
            },
            Test {
                name: "two producers on two bytes channel buffer",
                bufsiz: 2,
                producers: 2,
                data: 256,
            },
            Test {
                name: "two producers on 512 bytes channel buffer",
                bufsiz: 512,
                producers: 2,
                data: 256,
            },
            Test {
                name: "two producers on 1,024 bytes channel buffer",
                bufsiz: 1_024,
                producers: 2,
                data: 256,
            },
            Test {
                name: "two producers on 4K bytes channel buffer",
                bufsiz: 4_096,
                producers: 2,
                data: 256,
            },
            Test {
                name: "four producers on one byte channel buffer",
                bufsiz: 1,
                producers: 4,
                data: 256,
            },
            Test {
                name: "four producers on two bytes channel buffer",
                bufsiz: 2,
                producers: 4,
                data: 256,
            },
            Test {
                name: "four producers on 512 bytes channel buffer",
                bufsiz: 512,
                producers: 4,
                data: 256,
            },
            Test {
                name: "four producers on 1,024 bytes channel buffer",
                bufsiz: 1_024,
                producers: 4,
                data: 256,
            },
            Test {
                name: "four producers on 4K bytes channel buffer",
                bufsiz: 4_096,
                producers: 4,
                data: 256,
            },
            Test {
                name: "16 producers on one byte channel buffer",
                bufsiz: 1,
                producers: 16,
                data: 256,
            },
            Test {
                name: "16 producers on two bytes channel buffer",
                bufsiz: 2,
                producers: 16,
                data: 256,
            },
            Test {
                name: "16 producers on 512 bytes channel buffer",
                bufsiz: 512,
                producers: 16,
                data: 256,
            },
            Test {
                name: "16 producers on 1,024 bytes channel buffer",
                bufsiz: 1_024,
                producers: 16,
                data: 256,
            },
            Test {
                name: "16 producers on 4K bytes channel buffer",
                bufsiz: 4_096,
                producers: 16,
                data: 256,
            },
            Test {
                name: "64 producers on one byte channel buffer",
                bufsiz: 1,
                producers: 64,
                data: 256,
            },
            Test {
                name: "64 producers on two bytes channel buffer",
                bufsiz: 2,
                producers: 64,
                data: 256,
            },
            Test {
                name: "64 producers on 512 bytes channel buffer",
                bufsiz: 512,
                producers: 64,
                data: 256,
            },
            Test {
                name: "64 producers on 1,024 bytes channel buffer",
                bufsiz: 1_024,
                producers: 64,
                data: 256,
            },
            Test {
                name: "64 producers on 4K bytes channel buffer",
                bufsiz: 4_096,
                producers: 64,
                data: 256,
            },
            Test {
                name: "256 producers on one byte channel buffer",
                bufsiz: 1,
                producers: 256,
                data: 256,
            },
            Test {
                name: "256 producers on two bytes channel buffer",
                bufsiz: 2,
                producers: 256,
                data: 256,
            },
            Test {
                name: "256 producers on 512 bytes channel buffer",
                bufsiz: 512,
                producers: 256,
                data: 256,
            },
            Test {
                name: "256 producers on 1,024 bytes channel buffer",
                bufsiz: 1_024,
                producers: 256,
                data: 256,
            },
            Test {
                name: "256 producers on 4K bytes channel buffer",
                bufsiz: 4_096,
                producers: 256,
                data: 256,
            },
        ];
        for t in &tests {
            let t = t.clone();
            tokio::run(futures::future::lazy(move || {
                use futures::{Future, Sink};
                let (tx, rx) = futures::sync::mpsc::channel(t.bufsiz);
                for _ in 0..t.producers {
                    tokio::spawn({
                        tx.clone()
                            .send(t.data)
                            .map(|_| ())
                            .map_err(|err| eprintln!("{}", err))
                    });
                }
                tokio::spawn(super::sum(rx));
                println!("{}: done", t.name);
                Ok(())
            }));
        }
    }
    #[test]
    fn ping_and_pong() {
        #[derive(Clone)]
        struct Test {
            name: &'static str,
            bufsiz: usize,
            requesters: usize,
        }
        let tests = vec![
            Test {
                name: "one requester on one byte channel",
                bufsiz: 1,
                requesters: 1,
            },
            Test {
                name: "one requester on two bytes channel",
                bufsiz: 2,
                requesters: 1,
            },
            Test {
                name: "one requester on 512 bytes channel buffer",
                bufsiz: 512,
                requesters: 1,
            },
            Test {
                name: "one requester on 1,024 bytes channel buffer",
                bufsiz: 1_024,
                requesters: 1,
            },
            Test {
                name: "one requester on 4K bytes channel buffer",
                bufsiz: 4_096,
                requesters: 1,
            },
            Test {
                name: "two requesters on one byte channel",
                bufsiz: 1,
                requesters: 2,
            },
            Test {
                name: "two requesters on two bytes channel",
                bufsiz: 2,
                requesters: 2,
            },
            Test {
                name: "two requesters on 512 bytes channel buffer",
                bufsiz: 512,
                requesters: 2,
            },
            Test {
                name: "two requesters on 1,024 bytes channel buffer",
                bufsiz: 1_024,
                requesters: 2,
            },
            Test {
                name: "two requesters on 4K bytes channel buffer",
                bufsiz: 4_096,
                requesters: 2,
            },
            Test {
                name: "16 requesters on one byte channel",
                bufsiz: 1,
                requesters: 16,
            },
            Test {
                name: "16 requesters on two bytes channel",
                bufsiz: 2,
                requesters: 16,
            },
            Test {
                name: "16 requesters on 512 bytes channel buffer",
                bufsiz: 512,
                requesters: 16,
            },
            Test {
                name: "16 requesters on 1,024 bytes channel buffer",
                bufsiz: 1_024,
                requesters: 16,
            },
            Test {
                name: "16 requesters on 4K bytes channel buffer",
                bufsiz: 4_096,
                requesters: 16,
            },
            Test {
                name: "64 requesters on one byte channel",
                bufsiz: 1,
                requesters: 64,
            },
            Test {
                name: "64 requesters on two bytes channel",
                bufsiz: 2,
                requesters: 64,
            },
            Test {
                name: "64 requesters on 512 bytes channel buffer",
                bufsiz: 512,
                requesters: 64,
            },
            Test {
                name: "64 requesters on 1,024 bytes channel buffer",
                bufsiz: 1_024,
                requesters: 64,
            },
            Test {
                name: "64 requesters on 4K bytes channel buffer",
                bufsiz: 4_096,
                requesters: 64,
            },
            Test {
                name: "256 requesters on one byte channel",
                bufsiz: 1,
                requesters: 256,
            },
            Test {
                name: "256 requesters on two bytes channel",
                bufsiz: 2,
                requesters: 256,
            },
            Test {
                name: "256 requesters on 512 bytes channel buffer",
                bufsiz: 512,
                requesters: 256,
            },
            Test {
                name: "256 requesters on 1,024 bytes channel buffer",
                bufsiz: 1_024,
                requesters: 256,
            },
            Test {
                name: "256 requesters on 4K bytes channel buffer",
                bufsiz: 4_096,
                requesters: 256,
            },
            Test {
                name: "512 requesters on one byte channel",
                bufsiz: 1,
                requesters: 512,
            },
            Test {
                name: "512 requesters on two bytes channel",
                bufsiz: 2,
                requesters: 512,
            },
            Test {
                name: "512 requesters on 512 bytes channel buffer",
                bufsiz: 512,
                requesters: 512,
            },
            Test {
                name: "512 requesters on 1,024 bytes channel buffer",
                bufsiz: 1_024,
                requesters: 512,
            },
            Test {
                name: "512 requesters on 4K bytes channel buffer",
                bufsiz: 4_096,
                requesters: 512,
            },
            Test {
                name: "1,024 requesters on one byte channel",
                bufsiz: 1,
                requesters: 1_024,
            },
            Test {
                name: "1,024 requesters on two bytes channel",
                bufsiz: 2,
                requesters: 1_024,
            },
            Test {
                name: "1,024 requesters on 512 bytes channel buffer",
                bufsiz: 512,
                requesters: 1_024,
            },
            Test {
                name: "1,024 requesters on 1,024 bytes channel buffer",
                bufsiz: 1_024,
                requesters: 1_024,
            },
            Test {
                name: "1,024 requesters on 4K bytes channel buffer",
                bufsiz: 4_096,
                requesters: 1_024,
            },
            Test {
                name: "4,096 requesters on one byte channel",
                bufsiz: 1,
                requesters: 4_096,
            },
            Test {
                name: "4,096 requesters on two bytes channel",
                bufsiz: 2,
                requesters: 4_096,
            },
            Test {
                name: "4,096 requesters on 512 bytes channel buffer",
                bufsiz: 512,
                requesters: 4_096,
            },
            Test {
                name: "4,096 requesters on 1,024 bytes channel buffer",
                bufsiz: 1_024,
                requesters: 4_096,
            },
            Test {
                name: "4,096 requesters on 4K bytes channel buffer",
                bufsiz: 4_096,
                requesters: 4_096,
            },
            Test {
                name: "16,384 requesters on one byte channel",
                bufsiz: 1,
                requesters: 16_384,
            },
            Test {
                name: "16,384 requesters on two bytes channel",
                bufsiz: 2,
                requesters: 16_384,
            },
            Test {
                name: "16,384 requesters on 512 bytes channel buffer",
                bufsiz: 512,
                requesters: 16_384,
            },
            Test {
                name: "16,384 requesters on 1,024 bytes channel buffer",
                bufsiz: 1_024,
                requesters: 16_384,
            },
            Test {
                name: "16,384 requesters on 4K bytes channel buffer",
                bufsiz: 4_096,
                requesters: 16_384,
            },
        ];
        for t in &tests {
            let t = t.clone();
            tokio::run(futures::future::lazy(move || {
                use super::Future;
                let (tx, rx) = super::sync::mpsc::channel(t.bufsiz);
                tokio::spawn(super::pong(rx));
                for i in 0..t.requesters {
                    let name = t.name;
                    tokio::spawn(super::ping(i, tx.clone()).map(move |(got, _dur)| {
                        debug_assert_eq!(got, i, "{}", name);
                        ()
                    }));
                }
                Ok(())
            }));
        }
    }
}
