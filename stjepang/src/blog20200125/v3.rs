//! Build your own [block_on()] with [thread_local] caching
//!
//! [block_on()]: https://stjepang.github.io/2020/01/25/build-your-own-block-on.html
//! [thread_local]: https://doc.rust-lang.org/std/macro.thread_local.html
use std::future::Future;
use std::task::{Context, Poll};

/// Build your own [block_on()] with [thread_local] caching
///
/// # Examples
///
/// ```
/// use futures::channel::oneshot;
/// use std::thread;
/// use std::time::Duration;
/// use stjepang::blog20200125::v3::block_on;
///
/// let (tx, rx) = oneshot::channel();
///
/// thread::spawn(move || {
///     thread::sleep(Duration::from_millis(1));
///     tx.send("Hello world with cached block_on()").unwrap()
/// });
///
/// let msg = block_on(async {
///     println!("Awaiting...");
///     rx.await.unwrap()
/// });
/// assert_eq!("Hello world with cached block_on()", msg);
/// ```
/// [block_on()]: https://stjepang.github.io/2020/01/25/build-your-own-block-on.html
/// [thread_local]: https://doc.rust-lang.org/std/macro.thread_local.html
pub fn block_on<F: Future>(future: F) -> F::Output {
    pin_utils::pin_mut!(future);

    // There is only one instance is needed, as once the thread is in
    // block_on(), no way to get out of from here, unless there is
    // a recursive call through the future itself...
    thread_local! {
        static CACHE: (crossbeam::sync::Parker, std::task::Waker) = {
            let parker = crossbeam::sync::Parker::new();
            let unparker = parker.unparker().clone();
            let waker = async_task::waker_fn(move || unparker.unpark());
            (parker, waker)
        };
    }

    CACHE.with(|(parker, waker)| {
        let cx = &mut Context::from_waker(&waker);
        loop {
            match future.as_mut().poll(cx) {
                Poll::Ready(output) => return output,
                Poll::Pending => parker.park(),
            }
        }
    })
}
