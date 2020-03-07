//! TCP client connect timeout example
use std::{
    error::Error,
    future::Future,
    task::{Context, Poll},
};

use crossbeam_utils::sync::Parker;
use pin_utils::pin_mut;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + 'static>>;

fn main() -> Result<()> {
    block_on(async { Ok(()) })
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
