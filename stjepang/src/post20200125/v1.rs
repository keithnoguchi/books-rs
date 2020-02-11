//! Build your own [block_on()]
//!
//! [block_on()]: https://stjepang.github.io/2020/01/25/build-your-own-block-on.html
use std::future::Future;
use std::task::{Context, Poll};
use std::thread;

/// stjepang's [block_on()] v1.
///
/// # Examples
///
/// ```
/// use futures::channel::oneshot;
/// use std::thread;
/// use std::time::Duration;
/// use stjepang_blog::post20200125::v1::block_on;
///
/// let (s, r) = oneshot::channel();
///
/// // Create a sender thread.
/// thread::spawn(move || {
///     thread::sleep(Duration::from_millis(1));
///     s.send("Hello, world!").unwrap();
/// });
///
/// // Block the thread until the message is received.
/// let msg = block_on(async {
///     println!("Awaiting...");
///     r.await.unwrap()
/// });
/// assert_eq!("Hello, world!", msg);
/// ```
/// [block_on()]: https://stjepang.github.io/2020/01/25/build-your-own-block-on.html
pub fn block_on<F: Future>(future: F) -> F::Output {
    pin_utils::pin_mut!(future);
    let thread = thread::current();
    let waker = async_task::waker_fn(move || thread.unpark());
    let cx = &mut Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => thread::park(),
        }
    }
}
