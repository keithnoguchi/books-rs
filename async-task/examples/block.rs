//! [block_on] example
//!
//! [block_on]: https://github.com/async-rs/async-task/blob/master/examples/block.rs
use std::{
    cell::RefCell,
    future::Future,
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

use crossbeam_utils::sync::Parker;
use futures::channel::oneshot;

fn block_on<F: Future>(future: F) -> F::Output {
    pin_utils::pin_mut!(future);

    thread_local! {
        // Parker and waker associated with the current thread.
        static CACHE: RefCell<(Parker, Waker)> = {
            let parker = Parker::new();
            let unparker = parker.unparker().clone();
            let waker = async_task::waker_fn(move || unparker.unpark());
            RefCell::new((parker, waker))
        };
    }

    CACHE.with(|cache| {
        // Panic if `block_on()` is called recursively.
        let (parker, waker) = &mut *cache.try_borrow_mut().ok().expect("recursive block_on()");

        // Create the task context.
        let cx = &mut Context::from_waker(&waker);

        // Keep polling the future until completion.
        loop {
            match future.as_mut().poll(cx) {
                Poll::Ready(output) => return output,
                Poll::Pending => {
                    println!("polled and it's still pending...");
                    parker.park();
                }
            }
        }
    })
}

fn main() {
    let (s, r) = oneshot::channel();

    // sender
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        s.send("Hello, world!").unwrap();
    });

    // receiver
    let msg = block_on(async {
        println!("Awaiting...");
        r.await.unwrap()
    });
    println!("{}", msg);
}
