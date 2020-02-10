//! Build your own [block_on()] with [thread_local] caching
//!
//! [block_on()]: https://stjepang.github.io/2020/01/25/build-your-own-block-on.html
//! [thread_local]: https://doc.rust-lang.org/std/macro.thread_local.html
use futures::channel::oneshot;
use std::thread;
use std::time::Duration;
use stjepang::blog20200125::v3::block_on;

fn main() {
    let (tx, rx) = oneshot::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        tx.send("Hey yo, what's up!?").unwrap();
    });
    let msg = block_on(async {
        println!("Awaiting the message to arrive...");
        rx.await.unwrap()
    });
    println!("{}", msg);
}
