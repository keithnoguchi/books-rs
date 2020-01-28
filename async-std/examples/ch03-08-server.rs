//! [Handling Disconnection]
//!
//! Let's handle the client disconnection through the void channel.
//!
//! # Examples
//!
//! ```sh
//! $ cargo run --example ch03-08-server [::1]:8000
//! Compiling async-std-book v0.1.0 (/home/kei/git/books-rs/async-std)
//! Finished dev [unoptimized + debuginfo] target(s) in 1.90s
//! Running `/home/kei/git/books-rs/target/debug/examples/ch03-08-server '[::1]:8000'`
//! [server] starting
//! [broker] starting
//! [broker] started
//! [server] started
//! [reader@[::1]:53624] starting
//! [reader@[::1]:53626] starting
//! [reader@[::1]:53626] started for one
//! [writer@[::1]:53626] started for one
//! [reader@[::1]:53624] started for two
//! [writer@[::1]:53624] started for two
//! [reader@[::1]:53624] finished for two
//! [writer@[::1]:53624] finished for two
//! [reader@[::1]:53626] finished for one
//! [writer@[::1]:53626] finished for one
//! [reader@[::1]:53628] starting
//! [reader@[::1]:53628] started for tre
//! [writer@[::1]:53628] started for tre
//! [reader@[::1]:53628] finished for tre
//! [writer@[::1]:53628] finished for tre
//! ```
//! [handling disconnection]: https://book.async.rs/tutorial/handling_disconnection.html
use async_std::io::prelude::{BufReadExt, WriteExt};
use async_std::io::BufReader;
use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use futures::channel::mpsc;
use futures::future::FutureExt;
use futures::select;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use std::collections::{hash_map::Entry, HashMap};
use std::{error::Error, result, sync::Arc};

/// Alias types for the shorter typing.
type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;
type Cancel = mpsc::UnboundedReceiver<Void>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

/// Void enum for the Cancel channel.
enum Void {}

/// Event for the `reader` and `broker` tasks communication.
enum Event {
    Join {
        cancel: Cancel,
        name: String,
        stream: Arc<TcpStream>,
    },
    Message {
        from: String,
        msg: String,
    },
}

fn main() -> Result<()> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("localhost:8038"));
    task::block_on(supervisor(addr))
}

/// `supervisor` task to spawn all the child tasks and keep restarting those.
async fn supervisor(addr: String) -> Result<()> {
    let mut sleep = std::time::Duration::from_secs(1);
    loop {
        let (tx, rx) = mpsc::unbounded();
        let mut tasks = Vec::new();
        tasks.push(task::spawn(server(tx, addr.clone())));
        tasks.push(task::spawn(broker(rx)));
        for task in tasks {
            let id = task.task().id();
            if let Err(err) = task.await {
                eprintln!("[supervisor] task[{}] error: {}", id, err);
            }
        }
        eprintln!("[supervisor] sleep {:?} before the restart", sleep);
        task::sleep(sleep).await;
        sleep *= 2;
    }
}

/// `server` task to listen to the `addr` address.
async fn server(broker: Sender<Event>, addr: String) -> Result<()> {
    eprintln!("[server] starting");
    let s = TcpListener::bind(&addr).await?;
    let mut readers = Vec::new();
    eprintln!("[server] started");
    while let Some(s) = s.incoming().next().await {
        match s {
            Err(err) => {
                eprintln!("[server] client connection error: {}", err);
                continue;
            }
            Ok(s) => {
                readers.push(task::spawn(reader(broker.clone(), s)));
            }
        }
    }
    drop(broker);
    eprintln!("[server] finishing");
    for reader in readers {
        if let Err(err) = reader.await {
            eprintln!("[server] reader error: {}", err);
        }
    }
    eprintln!("[server] finished");
    Ok(())
}

/// `reader` to read the client message over `TcpStream`.
async fn reader(mut broker: Sender<Event>, mut s: TcpStream) -> Result<()> {
    let peer = s
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or(String::from("unknown"));
    eprintln!("[reader@{}] starting", peer);
    s.write_all(b"What is your name? ").await?;
    let (_tx, cancel) = mpsc::unbounded();
    let s = Arc::new(s);
    let mut reader = BufReader::new(&*s).lines();
    let name = match reader.next().await {
        None => Err(format!("premature close by {}", peer))?,
        Some(name) => name?,
    };
    broker
        .send(Event::Join {
            cancel,
            name: name.clone(),
            stream: s.clone(),
        })
        .await?;
    eprintln!("[reader@{}] started for {}", peer, name);
    while let Some(line) = reader.next().await {
        let msg = line?;
        broker
            .send(Event::Message {
                from: name.clone(),
                msg,
            })
            .await?;
    }
    eprintln!("[reader@{}] finished for {}", peer, name);
    Ok(())
}

/// `broker` task to mediate the `reader` and `writer` tasks for the client.
async fn broker(readers: Receiver<Event>) -> Result<()> {
    eprintln!("[broker] starting");
    let (writer_tx, writer_rx) = mpsc::unbounded::<(String, Receiver<Option<String>>)>();
    let mut writer_rx = writer_rx.fuse();
    let mut readers = readers.fuse();
    let mut writers = Vec::new();
    let mut peers = HashMap::new();
    eprintln!("[broker] started");
    loop {
        let event = select! {
            event = readers.next().fuse() => match event {
                None => break,
                Some(event) => event,
            },
            event = writer_rx.next().fuse() => match event {
                None => break,
                Some((writer, _pending_msgs)) => {
                    peers.remove(&writer);
                    continue;
                }
            },
        };
        match event {
            Event::Join {
                cancel,
                name,
                stream,
            } => match peers.entry(name.clone()) {
                Entry::Occupied(_) => {
                    let mut s = &*stream;
                    s.write_all(b"name is already taken.  bye!\n").await?;
                }
                Entry::Vacant(o) => {
                    let (mut tx, mut rx) = mpsc::unbounded();
                    let mut writer_tx = writer_tx.clone();
                    let w_name = name.clone();
                    writers.push(task::spawn(async move {
                        let ret = writer(cancel, &mut rx, w_name.clone(), stream).await;
                        if let Err(err) = writer_tx.send((w_name, rx)).await {
                            eprintln!("[writer] cannot shutdown: {}", err);
                        }
                        ret
                    }));
                    if let Err(err) = tx.send(None).await {
                        eprintln!("[broker] write error to {}: {}", name, err);
                    } else {
                        o.insert(tx);
                    }
                }
            },
            Event::Message { from, msg } => {
                let msg = format!("{}: {}\n", from, msg.trim());
                for (peer, mut tx) in &peers {
                    let ret = if peer == &from {
                        tx.send(None).await
                    } else {
                        tx.send(Some(msg.clone())).await
                    };
                    if let Err(err) = ret {
                        eprintln!("[broker] write error to {}: {}", peer, err);
                    }
                }
            }
        }
    }
    drop(writer_tx);
    drop(peers);
    while let Some((_name, _pending_msgs)) = writer_rx.next().await {}
    for writer in writers {
        if let Err(err) = writer.await {
            eprintln!("[broker] writer error: {}", err);
        }
    }
    eprintln!("[broker] finished");
    Ok(())
}

/// `writer` task to send the message to the client.
async fn writer(
    cancel: Cancel,
    broker: &mut Receiver<Option<String>>,
    name: String,
    s: Arc<TcpStream>,
) -> Result<()> {
    let mut s = &*s;
    let peer = s
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or(String::from("unknown"));
    let prompt = format!("{}> ", name);
    eprintln!("[writer@{}] started for {}", peer, name);
    let mut cancel = cancel.fuse();
    let mut broker = broker.fuse();
    loop {
        select! {
            msg = cancel.next().fuse() => match msg {
                None => break,
                Some(void) => match void {},
            },
            msg = broker.next().fuse() => match msg {
                None => break,
                Some(msg) => {
                    if let Some(msg) = msg {
                        s.write_all(msg.as_bytes()).await?;
                    }
                    s.write_all(prompt.as_bytes()).await?;
                }
            },
        }
    }
    eprintln!("[writer@{}] finished for {}", peer, name);
    Ok(())
}
