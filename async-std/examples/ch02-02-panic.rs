//! [panic()!] propergate it to the blocked task/thread
//!
//! # Examples
//!
//! ```rust
//! $ cargo run --example ch02-02-panic
//! Compiling async-std-book v0.1.0 (/home/kei/git/books-rs/async-std)
//! Finished dev [unoptimized + debuginfo] target(s) in 0.59s
//! Running `target/debug/examples/ch02-02-panic`
//! thread 'main' panicked at 'I'll propergate this panic to calling task/thread', async-std/examples/ch02-02-panic.rs:8:9
//! note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
//! ```
//! [panic()!]: https://book.async.rs/concepts/tasks.html#errors-and-panics
use async_std::task;

fn main() {
    task::block_on(async {
        panic!("I'll propergate this panic to calling task/thread");
    });
}
