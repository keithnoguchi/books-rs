//! An executor that assigns an ID to every spawned task.
use std::{cell::Cell, future::Future, panic::catch_unwind, thread};

use crossbeam_channel::{unbounded, Sender};
use crossbeam_utils::atomic::AtomicCell;
use futures::executor::block_on;
use once_cell::sync::Lazy;

#[derive(Clone, Copy, Debug)]
struct TaskId(usize);

type Task = async_task::Task<TaskId>;
type JoinHandle<T> = async_task::JoinHandle<T, TaskId>;

thread_local! {
    /// The ID of the current task.
    static TASK_ID: Cell<Option<TaskId>> = Cell::new(None);
}

/// Returns the ID of the currently executing task.
///
/// Returns `None` if called outside the runtime.
fn task_id() -> Option<TaskId> {
    TASK_ID.with(|id| id.get())
}

/// Spawns a future on the executor.
fn spawn<F, R>(future: F) -> JoinHandle<R>
where
    F: Future<Output = R> + Send + 'static,
    R: Send + 'static,
{
    let id = TaskId(COUNTER.fetch_add(1));

    // Create a task that is scheduled by sending itself into the channel.
    let schedule = |task| QUEUE.send(task).unwrap();
    let (task, handle) = async_task::spawn(future, schedule, id);

    // Schedule the task by sending it into the channel.
    task.schedule();

    handle
}

static QUEUE: Lazy<Sender<Task>> = Lazy::new(|| {
    let (s, r) = unbounded::<Task>();

    // Start the executor thread.
    thread::spawn(|| {
        TASK_ID.with(|id| {
            for task in r {
                // Store the task ID into the thread-local before running.
                id.set(Some(*task.tag()));

                // Ignore panics for simplicity.
                let _ignore_panic = catch_unwind(|| task.run());
            }
        });
    });
    s
});

static COUNTER: Lazy<AtomicCell<usize>> = Lazy::new(|| AtomicCell::new(0));

fn main() {
    let mut handles = vec![];

    // Spawn a bund of tasks.
    for _ in 0..10 {
        handles.push(spawn(async move {
            println!("Hello from task with {:?}", task_id());
        }));
    }

    // Wait for the tasks to finish.
    for handle in handles {
        block_on(handle);
    }
}
