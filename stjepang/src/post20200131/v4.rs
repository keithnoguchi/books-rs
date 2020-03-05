//! Build your own [executor], v4, handling panics
//!
//! [executor]: https://stjepang.github.io/2020/01/31/build-your-own-executor.html
use core::{
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll,
    },
};
use std::{panic::catch_unwind, thread};

type Task = async_task::Task<()>;
pub struct JoinHandle<R>(async_task::JoinHandle<R, ()>);

impl<R> Future for JoinHandle<R> {
    type Output = R;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.0).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(output) => Poll::Ready(output.expect("task failed")),
        }
    }
}

pub fn spawn<F, R>(future: F) -> JoinHandle<R>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let (task, handle) = async_task::spawn(future, |t| QUEUE.send(t).unwrap(), ());
    task.schedule();
    JoinHandle(handle)
}

use once_cell::sync::Lazy;

static QUEUE: Lazy<crossbeam_channel::Sender<Task>> = Lazy::new(|| {
    let (tx, rx) = crossbeam_channel::unbounded::<Task>();
    for _ in 0..num_cpus::get().max(1) {
        let rx = rx.clone();
        thread::spawn(move || rx.iter().for_each(|task| {
            let _ = catch_unwind(|| task.run());
        }));
    }
    tx
});
