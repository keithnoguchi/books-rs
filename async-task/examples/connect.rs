//! TCP client connect cancellation and the timeout example
//!
//! # Examples
//!
//! Success example
//!
//! ```sh
//! $ cargo run --example connect google.com:80
//! Compiling async-task-book v0.1.0 (/home/kei/git/books-rs/async-task)
//! Finished dev [unoptimized + debuginfo] target(s) in 1.35s
//! Running `/home/kei/git/books-rs/target/debug/examples/connect 'google.com:80'`
//! [client] run
//! [client] run
//! [client] run
//! V4(172.217.0.46:80)
//! [Ok(V4(192.168.255.201:46670))] dropped
//! ```
//!
//! Cancel example
//!
//! ```sh
//! $ cargo run --example connect google.com:800
//! Compiling async-task-book v0.1.0 (/home/kei/git/books-rs/async-task)
//! Finished dev [unoptimized + debuginfo] target(s) in 1.31s
//! Running `/home/kei/git/books-rs/target/debug/examples/connect 'google.com:800'`
//! [client] run
//! [client] run
//! quit
//! canceled
//! [client] run
//! ```
//!
//! Timed out example
//!
//! ```sh
//! $ cargo run --example connect google.com:800
//! Finished dev [unoptimized + debuginfo] target(s) in 0.05s
//! Running `/home/kei/git/books-rs/target/debug/examples/connect 'google.com:800'`
//! [client] run
//! [client] run
//! [client] run
//! future timed out
//! [client] run
//! [client] run
//! [client] run
//! future timed out
//! [client] run
//! [client] run
//! [client] run
//! future timed out
//! ```
use core::{
    fmt,
    future::Future,
    task::{Context, Poll},
    time::Duration,
};
use std::{error::Error, panic::catch_unwind, thread};

use async_std::{io, net::TcpStream};
use crossbeam_channel::{unbounded, Sender};
use crossbeam_utils::sync::Parker;
use futures::{
    future::{abortable, pending, FutureExt},
    select,
};
use once_cell::sync::Lazy;
use pin_utils::pin_mut;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

// Default concurrent client number.
const SERVER: &str = "google.com:80";
const TIMEOUT: u64 = 2000; // ms
const RETRY: usize = 3;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let addr = args.next().unwrap_or(SERVER.to_string());
    let timeout: Duration = args
        .next()
        .map(|ms| {
            ms.parse()
                .map(|ms| Duration::from_millis(ms))
                .unwrap_or(Duration::from_millis(TIMEOUT))
        })
        .unwrap_or(Duration::from_millis(TIMEOUT));
    let retry: usize = args
        .next()
        .map(|nr| nr.parse().unwrap_or(RETRY))
        .unwrap_or(RETRY);
    block_on(supervisor(addr, timeout, retry))
}

struct Connector {
    retry: usize,
    timeout: Duration,
}

impl Connector {
    fn new() -> Self {
        Self {
            retry: RETRY,
            timeout: Duration::from_millis(TIMEOUT),
        }
    }
    fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    fn retry(mut self, retry: usize) -> Self {
        // Five retry max.
        self.retry = retry.max(1).min(5);
        self
    }
    async fn connect(self, addr: String) -> Result<Peer> {
        let mut timeout = self.timeout;
        let mut ret = None;
        for i in 1..=self.retry {
            timeout *= i as u32;
            match io::timeout(timeout, TcpStream::connect(&addr)).await {
                Err(err) => ret = Some(err),
                Ok(s) => return Ok(Peer::new(s)),
            }
        }
        Err(Box::new(ret.take().unwrap()))
    }
}

impl Drop for Connector {
    fn drop(&mut self) {
        eprintln!("[connector] dropped");
    }
}

struct Peer {
    stream: TcpStream,
    tasks: Vec<JoinHandle<()>>,
}

impl Peer {
    fn new(stream: TcpStream) -> Self {
        // dummy tasks.
        let mut tasks = vec![];
        for i in 0..100 {
            let task = spawn(pending(), format!("task{}", i));
            tasks.push(task);
        }
        Self { stream, tasks }
    }
    async fn cancel(mut self) {
        while let Some(task) = self.tasks.pop() {
            task.cancel();
            let _ = task.await;
        }
    }
}

impl fmt::Display for Peer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "peer({:?}),tasks={}",
            self.stream.local_addr().unwrap(),
            self.tasks.len()
        )
    }
}

impl Drop for Peer {
    fn drop(&mut self) {
        eprintln!("[{}] dropped", self);
    }
}

async fn supervisor(addr: String, timeout: Duration, retry: usize) -> Result<()> {
    let (peer, abort) = abortable(Connector::new().timeout(timeout).retry(retry).connect(addr));
    let peer = spawn(peer, String::from("connector")).fuse();
    pin_mut!(peer);
    let mut buf = String::new();
    let cancel = io::stdin();
    let peer = select! {
        _ = cancel.read_line(&mut buf).fuse() => {
            eprintln!("canceled");
            abort.abort();
            return Ok(());
        }
        peer = peer => match peer {
            None => Err("canceled")?,
            Some(peer) => peer??,
        }
    };
    eprintln!("[{}] connected", peer);
    peer.cancel().await;
    Ok(())
}

fn block_on<F: Future>(future: F) -> F::Output {
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = async_task::waker_fn(move || unparker.unpark());
    let cx = &mut Context::from_waker(&waker);
    pin_mut!(future);
    loop {
        match future.as_mut().poll(cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => parker.park(),
        }
    }
}

type TaskId = String;
type Task = async_task::Task<TaskId>;
type JoinHandle<R> = async_task::JoinHandle<R, TaskId>;

fn spawn<F, R>(future: F, tag: TaskId) -> JoinHandle<F::Output>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let schedule = |task| QUEUE.send(task).unwrap();
    let (task, handle) = async_task::spawn(future, schedule, tag);
    task.schedule();
    handle
}

static QUEUE: Lazy<Sender<Task>> = Lazy::new(|| {
    let (tx, rx) = unbounded::<Task>();
    for _ in 0..num_cpus::get().max(1) {
        let rx = rx.clone();
        thread::spawn(move || {
            rx.iter().for_each(|task| {
                // ignore the task panic.
                let _ = catch_unwind(|| {
                    task.run();
                });
            });
        });
    }
    tx
});
