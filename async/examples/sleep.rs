//! Let's [build a timer]!
//!
//! [build a timer]: https://rust-lang.github.io/async-book/02_execution/03_wakeups.html
use futures::executor::block_on;

use async_book::TimerFuture;

fn main() {
    let millis: u64 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| usage())
        .parse()
        .unwrap_or_else(|_| usage());
    block_on(TimerFuture::new(std::time::Duration::from_millis(millis)));
}

fn usage() -> ! {
    let progname = std::env::args().next().unwrap_or(String::from("<unknown>"));
    eprintln!("usage: {} <milliseconds>", progname);
    std::process::exit(1);
}
