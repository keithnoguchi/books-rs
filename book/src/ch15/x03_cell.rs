//! [RefCell<T>] and the Interior Mutability Pattern
//!
//! [refcell<t>]: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
//!
//! # Example
//!
//! ```rust
//! use std::cell::RefCell;
//!
//! use the_book::ch15::x03_cell::{LimitTracker, Messenger};
//!
//! struct Cacher {
//!     msgs: RefCell<Vec<String>>,
//! }
//!
//! impl Cacher {
//!     fn new() -> Self {
//!         Self {
//!             msgs: RefCell::new(Vec::new()),
//!         }
//!     }
//! }
//!
//! impl Messenger for Cacher {
//!     fn send(&self, msg: &str) {
//!         self.msgs.borrow_mut().push(String::from(msg));
//!     }
//! }
//!
//! let cacher = Cacher::new();
//! let mut tracker = LimitTracker::new(&cacher, 100);
//!
//! tracker.set_value(75);
//! tracker.set_value(90);
//! tracker.set_value(100);
//! let wants = vec![
//!     "Warning: You've used up over 75% of your quota!",
//!     "Urgent: You've used up over 90% of your quota!",
//!     "Error: You are over your quota!",
//! ];
//! for (i, want) in wants.iter().enumerate() {
//!     assert_eq!(*want, &cacher.msgs.borrow()[i]);
//! }
//! ```

pub struct LimitTracker<'a, T>
where
    T: 'a + Messenger,
{
    messenger: &'a T,
    value: usize,
    max: usize,
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

impl<'a, T> LimitTracker<'a, T>
where
    T: 'a + Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        Self {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage = self.value as f64 / self.max as f64;
        if percentage >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage >= 0.9 {
            self.messenger
                .send("Urgent: You've used up over 90% of your quota!");
        } else if percentage >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> Self {
            Self {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl super::Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let want = "Warning: You've used up over 75% of your quota!";
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = super::LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(75);
        assert_eq!(1, mock_messenger.sent_messages.borrow().len());
        assert_eq!(want, mock_messenger.sent_messages.borrow()[0]);
    }
}
