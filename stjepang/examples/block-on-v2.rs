//! Build your own [block_on()] with crossbeam::Parker
//!
//! [block_on()]: https://stjepang.github.io/2020/01/25/build-your-own-block-on.html
//! [crossbeam::Parker]: https://docs.rs/crossbeam/latest/crossbeam/sync/struct.Parker.html
use futures::channel::oneshot;
use std::thread;
use std::time::Duration;
use stjepang_blog::post20200125::v2::block_on;

fn main() {
    let (tx, rx) = oneshot::channel();

    // Let's spin up the sender thread.
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        tx.send("Hello world, for Paker() based block_on()")
            .unwrap();
    });

    // Block on waiting for the message from the above thread.
    let msg = block_on(async {
        println!("Awaiting...");
        rx.await.unwrap()
    });
    println!("{}", msg);
}
