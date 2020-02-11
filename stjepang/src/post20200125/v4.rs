//! Build your own [block_on()] with crashing recursive block_on() call
//!
//! [block_on()]: https://stjepang.github.io/2020/01/25/build-your-own-block-on.html
use std::cell::RefCell;
use std::future::Future;
use std::task::{Context, Poll};

/// Build your own [block_on()] with crashing recursive block_on() call
///
/// # Examples
///
/// ```
/// use futures::channel::oneshot;
/// use std::thread;
/// use std::time::Duration;
/// use stjepang_blog::post20200125::v4::block_on;
///
/// let (tx, rx) = oneshot::channel();
///
/// // Send a message to the blocker.
/// thread::spawn(move || {
///     thread::sleep(Duration::from_millis(1));
///     tx.send("Hello block_on, which crashes with recursion").unwrap();
/// });
///
/// let msg = block_on(async {
///     println!("Awaiting...");
///     rx.await.unwrap()
/// });
/// assert_eq!("Hello block_on, which crashes with recursion", msg);
/// ```
/// [block_on()]: https://stjepang.github.io/2020/01/25/build-your-own-block-on.html
pub fn block_on<F: Future>(future: F) -> F::Output {
    pin_utils::pin_mut!(future);

    // We'll use `RefCell` for the thread local variable and crash if there are
    // recursive `block_on()` call, e.g. `block_on()` call inside the `Future`.
    thread_local! {
        static CACHE: RefCell<(crossbeam::sync::Parker, std::task::Waker)> = {
            let parker = crossbeam::sync::Parker::new();
            let unparker = parker.unparker().clone();
            let waker = async_task::waker_fn(move || unparker.unpark());
            RefCell::new((parker, waker))
        };
    }
    CACHE.with(|cache| {
        let (parker, waker) = &mut *cache.try_borrow_mut().expect("recursive `block_on`");

        let cx = &mut Context::from_waker(&waker);
        loop {
            match future.as_mut().poll(cx) {
                Poll::Ready(output) => return output,
                Poll::Pending => parker.park(),
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::block_on;
    #[test]
    #[should_panic]
    fn should_panic_with_recursive_block_on() {
        block_on(async {
            block_on(async {
                println!("you can't do that!");
            });
        });
    }
}
