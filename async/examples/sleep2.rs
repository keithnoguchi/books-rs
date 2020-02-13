//! Build an [Executor]
//!
//! [executor]: https://rust-lang.github.io/async-book/02_execution/04_executor.html
use async_book::{new_executor_and_spawner, TimerFuture};

fn main() {
    let (executor, spawner) = new_executor_and_spawner();
    let millis: u64 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| usage())
        .parse()
        .unwrap_or_else(|_| usage());
    spawner.spawn(TimerFuture::new(std::time::Duration::from_millis(millis)));
    drop(spawner); // Drop the spawner so that executor.run() will exit below.
    executor.run();
}

fn usage() -> ! {
    let progname = std::env::args().next().unwrap_or(String::from("[unknown]"));
    eprintln!("usage: {} <miliseconds>", progname);
    std::process::exit(1);
}
