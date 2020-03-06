//! waker_fn example
use async_task;

fn main() {
    let message = "hello";
    let waker = async_task::waker_fn(move || println!("{}", message));
    waker.wake_by_ref();
    waker.wake_by_ref();
}
