//! Let's [build a timer]!
//!
//! [build a timer]: https://rust-lang.github.io/async-book/02_execution/03_wakeups.html
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;

/// Future based timer example
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use futures::executor::block_on;
///
/// use async_book::TimerFuture;
///
/// // wake up after 1ms.
/// let f = TimerFuture::new(Duration::from_millis(1));
/// block_on(f);
/// ```
pub struct TimerFuture {
    state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    /// Where or not the sleep time has elapsed.
    completed: bool,

    /// The waker for the task that `TimerFuture` is running on.
    /// The thread can use this after setting `completed = true` to tell
    /// `TimerFuture`'s task to wake up, see that `completed = true`,
    /// and move forward.
    waker: Option<Waker>,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        let cloned_state = state.clone();
        // Creating a reactor, which calls `Waker.wake()`
        // to let executor knows the readiness of this
        // task.
        std::thread::spawn(move || {
            std::thread::sleep(duration);
            let mut state = cloned_state.lock().unwrap();
            state.completed = true;
            // wake up the task.
            if let Some(waker) = state.waker.take() {
                waker.wake()
            }
        });
        Self { state }
    }
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        if state.completed {
            Poll::Ready(())
        } else {
            // setup the waker so that the timer thread created
            // in `new()` above can notify the executor call `poll()`
            // again.
            //
            // The reason of the `clone()` here is due to the task
            // movement amoung the different thread, which I need
            // to dig a bit more to understand fully, though...
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
