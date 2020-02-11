//! Build your own [block_on()] with [crossbeam::sync::Parker]
//!
//! [block_on()]: https://stjepang.github.io/2020/01/25/build-your-own-block-on.html
//! [crossbeam::sync::Parker]: https://docs.rs/crossbeam/latest/crossbeam/sync/struct.Parker.html
use std::future::Future;
use std::task::{Context, Poll};

/// Build your own [block_on()] with crossbeam::sync::Parker
///
/// # Examples
///
/// ```
/// use futures::channel::oneshot;
/// use std::thread;
/// use std::time::Duration;
/// use stjepang_blog::post20200125::v2::block_on;
///
/// let (tx, rx) = oneshot::channel();
///
/// // Spin up the sender thread.
/// thread::spawn(move || {
///     thread::sleep(Duration::from_millis(1));
///     tx.send("Hello world, v2").unwrap();
/// });
///
/// // Block on the thread to wait for the message.
/// let msg = block_on(async {
///     println!("Awaiting...");
///     rx.await.unwrap()
/// });
/// assert_eq!("Hello world, v2", msg);
/// ```
/// [block_on()]: https://stjepang.github.io/2020/01/25/build-your-own-block-on.html
/// [crossbeam::Parker]: https://docs.rs/crossbeam/latest/crossbeam/sync/struct.Parker.html
pub fn block_on<F: Future>(future: F) -> F::Output {
    pin_utils::pin_mut!(future);

    let parker = crossbeam::sync::Parker::new();
    let unparker = parker.unparker().clone();
    let waker = async_task::waker_fn(move || unparker.unpark());

    let cx = &mut Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => parker.park(),
        }
    }
}
