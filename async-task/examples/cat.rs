//! [async-std book] [tasks] example with [async-task].
//!
//! # Examples
//!
//! cat with 100us timeout:
//!
//! ```sh
//! $ cargo run --example cat Cargo.toml 100
//! Finished dev [unoptimized + debuginfo] target(s) in 0.03s
//! Running `/home/kei/git/books-rs/target/debug/examples/cat Cargo.toml 100`
//! Error: future timed out
//! ```
//!
//! cat with 1ms timeout:
//!
//! ```sh
//! $ c run --example cat Cargo.toml 1000
//! Compiling async-task-book v0.1.0 (/home/kei/git/books-rs/async-task)
//! Finished dev [unoptimized + debuginfo] target(s) in 1.04s
//! Running `/home/kei/git/books-rs/target/debug/examples/cat Cargo.toml 1000`
//! [package]
//! ...
//! ```
//! [tasks]: https://book.async.rs/concepts/tasks.html
//! [async-std]: https://book.async.rs/
//! [async-task]: https://lib.rs/crates/async-task
use std::{
    cell::RefCell,
    env::args,
    future::Future,
    panic::catch_unwind,
    pin::Pin,
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

use async_std::{
    fs::File,
    io::{timeout, ReadExt, Result},
};
use crossbeam_channel::{unbounded, Sender};
use crossbeam_utils::sync::Parker;
use once_cell::sync::Lazy;

fn main() {
    let mut args = args();
    let path = args.nth(1).unwrap_or("Cargo.toml".to_string());
    let us: u64 = args
        .next()
        .unwrap_or(String::from("1000"))
        .parse()
        .unwrap_or(1000);
    let handle = spawn(read_file(path, us));
    block_on(async move {
        match handle.await {
            Err(err) => eprintln!("Error: {}", err),
            Ok(ret) => println!("{}", ret),
        }
    });
}

async fn read_file(path: String, us: u64) -> Result<String> {
    let mut f = timeout(Duration::from_micros(us), File::open(&path)).await?;
    let mut buf = String::new();
    timeout(Duration::from_micros(us), f.read_to_string(&mut buf)).await?;
    Ok(buf)
}

type Task = async_task::Task<()>;
struct JoinHandle<R>(async_task::JoinHandle<R, ()>);

impl<R> Future for JoinHandle<R> {
    type Output = R;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.0).poll(cx) {
            Poll::Ready(output) => Poll::Ready(output.expect("task failed")),
            Poll::Pending => Poll::Pending,
        }
    }
}

fn spawn<F, R>(future: F) -> JoinHandle<R>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let schedule = |task| QUEUE.send(task).unwrap();
    let (task, handle) = async_task::spawn(future, schedule, ());
    task.schedule();
    JoinHandle(handle)
}

static QUEUE: Lazy<Sender<Task>> = Lazy::new(|| {
    let (s, r) = unbounded::<Task>();
    // executor thread.
    thread::spawn(|| {
        for task in r {
            let _ignore_panic = catch_unwind(|| task.run());
        }
    });
    s
});

fn block_on<F: Future>(future: F) -> F::Output {
    thread_local! {
        // Parker and waker associated with the current thread.
        static CACHE: RefCell<(Parker, Waker)> = {
            let parker = Parker::new();
            let unparker = parker.unparker().clone();
            let waker = async_task::waker_fn(move || unparker.unpark());
            RefCell::new((parker, waker))
        };
    }
    pin_utils::pin_mut!(future);
    CACHE.with(|cache| {
        // Panic if `block_on()` is called recursively.
        let (parker, waker) = &mut *cache.try_borrow_mut().ok().expect("recursive block_on()");

        // Create the task context.
        let cx = &mut Context::from_waker(&waker);

        // Keep polling the future until completion.
        loop {
            match future.as_mut().poll(cx) {
                Poll::Ready(output) => return output,
                Poll::Pending => parker.park(),
            }
        }
    })
}
