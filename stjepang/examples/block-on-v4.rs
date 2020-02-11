//! Build your own [block_on()] with crashing recursive block_on() call
//!
//! [block_on()]: https://stjepang.github.io/2020/01/25/build-your-own-block-on.html
use futures::channel::oneshot;
use std::thread;
use std::time::Duration;
use stjepang_blog::post20200125::v4::block_on;

fn main() {
    let (tx, rx) = oneshot::channel();

    // let's fire up the thread and send message.
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        tx.send("Hey you, are you there?").unwrap();
    });

    // let's block_on the message.
    let msg = block_on(async {
        println!("Awaiting...");
        rx.await.unwrap()
    });
    println!("{}", msg);
}
