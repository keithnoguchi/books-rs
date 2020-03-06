//! `JoinHandle.cancel()` example
//!
//! # Example
//!
//! `handle.cancel()` case:
//!
//! ```sh
//! $ cargo run --example cancel cancel
//! Finished dev [unoptimized + debuginfo] target(s) in 0.03s
//! Running `/home/kei/git/books-rs/target/debug/examples/cancel cancel`
//! ```
//! Non cancel case: (It stuck forever)
//!
//! ```sh
//! $ cargo run --example cancel
//! Finished dev [unoptimized + debuginfo] target(s) in 0.03s
//! Running `/home/kei/git/books-rs/target/debug/examples/cancel`
//! ^C
//! ```
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use std::{env::args, panic::catch_unwind, thread};

use crossbeam_channel::{unbounded, Sender};
use crossbeam_utils::sync::Parker;
use once_cell::sync::Lazy;
use pin_utils::pin_mut;

fn main() {
    let cancel: bool = args().nth(1).map(|_| true).unwrap_or(false);
    let handle = spawn(pending());
    if cancel {
        // cancel the handle so that the handle future won't block with `block_on()`.
        handle.cancel();
    }
    block_on(handle);
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

type Task = async_task::Task<()>;
type JoinHandle<R> = async_task::JoinHandle<R, ()>;
struct Pending;

impl Future for Pending {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        Poll::Pending
    }
}

async fn pending() {
    let future = Pending {};
    future.await
}

fn spawn<F, R>(future: F) -> JoinHandle<R>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let schedule = |task| QUEUE.send(task).unwrap();
    let (task, handle) = async_task::spawn(future, schedule, ());
    task.schedule();
    handle
}

// ThreadPool run queue.
static QUEUE: Lazy<Sender<Task>> = Lazy::new(|| {
    let (tx, rx) = unbounded::<Task>();
    for _ in 0..num_cpus::get().max(1) {
        let rx = rx.clone();
        thread::spawn(|| {
            for task in rx {
                let _ignore_panic = catch_unwind(|| task.run());
            }
        });
    }
    tx
});
