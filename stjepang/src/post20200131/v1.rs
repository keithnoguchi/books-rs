//! Build your own [executor], v1
//!
//! [executor]: https://stjepang.github.io/2020/01/31/build-your-own-executor.html
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Context;
use std::thread;

pub type JoinHandle<R> = Pin<Box<dyn Future<Output = R> + Send>>;

/// `spawn()` for our own executor.
///
/// # Examples
///
/// With `futures::executor::block_on`:
///
/// ```
/// use futures;
///
/// use stjepang_blog::post20200131::v1::spawn;
///
/// futures::executor::block_on(async {
///     let handle = spawn(async { 1 + 2 });
///     assert_eq!(3, handle.await);
/// });
/// ```
///
/// With our own `block_on`:
///
/// ```
/// use stjepang_blog::post20200125::v4::block_on;
/// use stjepang_blog::post20200131::v1::spawn;
///
/// block_on(async {
///     let handle = spawn(async { 1 + 2 });
///     assert_eq!(3, handle.await);
/// });
/// ```
pub fn spawn<F, R>(future: F) -> JoinHandle<R>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let (tx, rx) = futures_channel::oneshot::channel();

    let future = async move {
        let _ = tx.send(future.await);
    };
    let task = Arc::new(Task {
        state: AtomicUsize::new(0),
        future: Mutex::new(Box::pin(future)),
    });
    QUEUE.send(task).unwrap();

    Box::pin(async move { rx.await.unwrap() })
}

use once_cell::sync::Lazy;

// Module wide run queue.
static QUEUE: Lazy<crossbeam_channel::Sender<Arc<Task>>> = Lazy::new(|| {
    let (tx, rx) = crossbeam_channel::unbounded::<Arc<Task>>();
    for _ in 0..num_cpus::get().max(1) {
        let rx = rx.clone();
        thread::spawn(move || rx.iter().for_each(|task| task.run()));
    }
    tx
});

const WOKEN: usize = 0b01;
const RUNNING: usize = 0b10;

struct Task {
    state: AtomicUsize,
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
}

impl Task {
    fn run(self: Arc<Self>) {
        let task = self.clone();
        let waker = async_task::waker_fn(move || {
            if task.state.fetch_or(WOKEN, Ordering::SeqCst) == 0 {
                QUEUE.send(task.clone()).unwrap();
            }
        });

        self.state.store(RUNNING, Ordering::SeqCst);
        let cx = &mut Context::from_waker(&waker);
        let poll = self.future.try_lock().unwrap().as_mut().poll(cx);

        if poll.is_pending() && self.state.fetch_and(!RUNNING, Ordering::SeqCst) == WOKEN | RUNNING
        {
            QUEUE.send(self).unwrap();
        }
    }
}
