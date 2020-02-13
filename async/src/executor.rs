//! Build an [Executor]
//!
//! [executor]: https://rust-lang.github.io/async-book/02_execution/04_executor.html
//!
//! # Examples
//!
//! ```
//! use std::time::Duration;
//!
//! use async_book::{
//!     new_executor_and_spawner,
//!     TimerFuture,
//! };
//!
//! let (executor, spawner) = new_executor_and_spawner();
//!
//! spawner.spawn(async {
//!     println!("howdy!");
//!     // Wait for one microsecond.
//!     TimerFuture::new(Duration::new(0, 1_000)).await;
//!     println!("done!");
//! });
//!
//! // Drop the spawner so that our executor knows it is finished and
//! // won't receive more incoming tasks to run.
//! drop(spawner);
//!
//! // Run the executor until the task queue is empty.
//! executor.run();
//! ```
use std::{
    future::Future,
    sync::mpsc::{self, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};

/// Create a executor and spawner, so that you can spawn a task and places
/// it on a executor.
pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    // Maximum number of tasks to allow queuing in the channel at once.
    // This is just to make `sync_channel` happy, and wouldn't be a real executor.
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (tx, rx) = mpsc::sync_channel(MAX_QUEUED_TASKS);
    (Executor { queue: rx }, Spawner { queue: tx })
}

/// Task executor that receives tasks off of a channel
/// and runs them.
pub struct Executor {
    /// Run queue endpoint to wait for the next task to execute.
    queue: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                if let Poll::Pending = future.as_mut().poll(context) {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

/// `Spawner` spawns new futures onto the task channel.
#[derive(Clone)]
pub struct Spawner {
    queue: SyncSender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            queue: self.queue.clone(),
        });
        self.queue.send(task).expect("too many task queued");
    }
}

/// A future that can reschedule itself to be polled by [`Executor`].
///
/// [`executor`]: struct.Executor.html
struct Task {
    /// In-progress future that should be pushed to completion.
    ///
    /// The `Mutex` is not necessary for correctness, since we only have
    /// one thread executing tasks at once.  However, Rust isn't smart enough
    /// to know that `future` is only mutated from one thread, so we need to
    /// use the `Mutex` to prove thread-safety.  A production executor would
    /// not need this, and could use `UnsafeCell` instead.
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// Run queue tx to re-submit to the [`Executor`].
    queue: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.queue.send(cloned).expect("too many tasks queued");
    }
}
