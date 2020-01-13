//! LimitTracker type and Messenger trait to demonstrate [RefCell<T>] usage.
//!
//! [refcell<t>]: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
use std::cell::RefCell;

use the_book::ch15::x03_cell::{LimitTracker, Messenger};

struct Cacher {
    msgs: RefCell<Vec<String>>,
}

impl Cacher {
    fn new() -> Self {
        Self {
            msgs: RefCell::new(Vec::new()),
        }
    }
}

impl Messenger for Cacher {
    fn send(&self, msg: &str) {
        self.msgs.borrow_mut().push(msg.to_string());
    }
}

fn main() {
    let cacher = Cacher::new();
    let mut tracker = LimitTracker::new(&cacher, 100);

    tracker.set_value(75);
    tracker.set_value(90);
    tracker.set_value(100);
    let wants = vec![
        "Warning: You've used up over 75% of your quota!",
        "Urgent: You've used up over 90% of your quota!",
        "Error: You are over your quota!",
    ];
    for (i, want) in wants.iter().enumerate() {
        assert_eq!(*want, &cacher.msgs.borrow()[i]);
    }
}
