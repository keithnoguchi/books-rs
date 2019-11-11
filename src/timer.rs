// SPDX-License-Identifier: GPL-2.0
// https://rust-lang.github.io/async-book/print.html#applied-build-a-timer
use std::future;
use std::pin;
use std::sync;
use std::task;
use std::thread;
use std::time;

pub struct TimerFuture {
    shared_state: sync::Arc<sync::Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<task::Waker>,
}

impl future::Future for TimerFuture {
    type Output = ();
    fn poll(self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            task::Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            task::Poll::Pending
        }
    }
}

impl TimerFuture {
    #[allow(dead_code)]
    pub fn new(duration: time::Duration) -> Self {
        let shared_state = sync::Arc::new(sync::Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        // Spawn the new thread
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });
        TimerFuture { shared_state }
    }
}
