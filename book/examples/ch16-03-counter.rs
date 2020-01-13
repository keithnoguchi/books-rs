//! Counter with [Mutex<T>]
//!
//! [mutex<t>]: https://doc.rust-lang.org/book/ch16-03-shared-state.html
use std::sync::Arc;
use std::thread::{self, Result};

use the_book::ch16::x03_counter::Counter;

fn main() -> Result<()> {
    let mut handlers = Vec::with_capacity(100_000);
    let counter = Arc::new(Counter::new(100u128));

    for _ in 0..1_000 {
        let counter = counter.clone();
        let handler = thread::spawn(move || counter.inc(2_000_000));
        handlers.push(handler);
    }
    for handler in handlers {
        handler.join()?
    }
    assert_eq!(2_000_000_100, counter.get());
    Ok(())
}
