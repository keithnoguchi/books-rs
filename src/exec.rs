// SPDX-License-Identifier: GPL-2.0
use futures::future::{self, FutureExt};
use std::sync::{self, mpsc};
use std::task;

pub struct Executor {
    ready_queue: mpsc::Receiver<sync::Arc<Task>>,
}

#[derive(Clone)]
pub struct Spawner {
    task_sender: mpsc::SyncSender<sync::Arc<Task>>,
}

#[allow(dead_code)]
struct Task {
    future: sync::Mutex<Option<future::BoxFuture<'static, ()>>>,
    task_sender: mpsc::SyncSender<sync::Arc<Task>>,
}

pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUE_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = mpsc::sync_channel(MAX_QUEUE_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    pub fn spawn(&self, future: impl future::Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = sync::Arc::new(Task {
            future: sync::Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

impl futures::task::ArcWake for Task {
    #[allow(dead_code)]
    fn wake_by_ref(arc_self: &sync::Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = futures::task::waker_ref(&task);
                let ctx = &mut task::Context::from_waker(&*waker);
                if let task::Poll::Pending = future.as_mut().poll(ctx) {
                    *future_slot = Some(future);
                }
            }
        }
    }
}
