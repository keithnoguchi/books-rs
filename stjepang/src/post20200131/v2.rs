//! Build your own [executor], v2
//!
//! [executor]: https://stjepang.github.io/2020/01/31/build-your-own-executor.html
use std::{future::Future, pin::Pin, thread};

type Task = async_task::Task<()>;
type JoinHandle<R> = Pin<Box<dyn Future<Output = R> + Send>>;

pub fn spawn<F, R>(future: F) -> JoinHandle<R>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let (task, handle) = async_task::spawn(future, |t| QUEUE.send(t).unwrap(), ());
    task.schedule();
    Box::pin(async { handle.await.unwrap() })
}

use once_cell::sync::Lazy;

static QUEUE: Lazy<crossbeam_channel::Sender<Task>> = Lazy::new(|| {
    let (tx, rx) = crossbeam_channel::unbounded::<Task>();
    for _ in 0..num_cpus::get().max(1) {
        let rx = rx.clone();
        thread::spawn(move || rx.iter().for_each(|task| task.run()));
    }
    tx
});
