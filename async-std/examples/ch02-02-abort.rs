//! [panic()!] aborts spawned task
//!
//! # Examples
//!
//! ```sh
//! $ cargo run --example ch02-02-abort
//! Compiling async-std-book v0.1.0 (/home/kei/git/books-rs/async-std)
//! Finished dev [unoptimized + debuginfo] target(s) in 0.74s
//! Running `/home/kei/git/books-rs/target/debug/examples/ch02-02-abort`
//! thread 'async-std/executor' panicked at 'i'll just abort and core dump, as no one can catch me...', async-std/examples/ch02-02-abort.rs:11:9
//! note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
//!  Aborted (core dumped)
//!  ```
//! [panic()!]: https://book.async.rs/concepts/tasks.html#errors-and-panics
use async_std::task;
use std::time::Duration;

fn main() {
    task::spawn(async {
        panic!("i'll just abort and core dump, as no one can catch me...");
    });
    // Main task/thread is just sleeping.
    task::block_on(async {
        task::sleep(Duration::from_secs(5)).await;
    });
}
