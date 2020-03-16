//! [RefCell<T>] and the Interior Mutability Pattern
//!
//! [refcell<t>]: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

/// Message trait to trigger to send a message.
pub trait Messenger {
    fn send(&self, message: &str);
}

/// Tracking the limit and call `send()` method of `Messenger` trait implementor.
pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: 'a + Messenger> LimitTracker<'a, T> {
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
    use super::{LimitTracker, Messenger};

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        struct MockMessenger(RefCell<Vec<String>>);
        impl Messenger for MockMessenger {
            fn send(&self, msg: &str) {
                self.0.borrow_mut().push(String::from(msg));
            }
        }
        let messenger = MockMessenger(RefCell::new(vec![]));
        let mut tracker = LimitTracker::new(&messenger, 100);
        tracker.set_value(75);
        tracker.set_value(90);
        tracker.set_value(100);
        let wants = vec![
            "Warning: You've used up over 75% of your quota!",
            "Urgent: You've used up over 90% of your quota!",
            "Error: You are over your quota!",
        ];
        assert_eq!(wants.len(), messenger.0.borrow().len());
        for (i, want) in wants.iter().enumerate() {
            assert_eq!(*want, &messenger.0.borrow()[i]);
        }
    }
    #[test]
    fn double_borrow_ok() {
        struct MockMessenger(RefCell<Vec<String>>);
        impl Messenger for MockMessenger {
            fn send(&self, _msg: &str) {
                let one = self.0.borrow();
                let two = self.0.borrow();
                assert_eq!(0, one.len());
                assert_eq!(0, two.len());
            }
        }
        let messenger = MockMessenger(RefCell::new(vec![]));
        let mut tracker = LimitTracker::new(&messenger, 100);
        tracker.set_value(75);
    }
    #[test]
    #[should_panic(expected = "already borrowed: BorrowMutError")]
    fn double_borrow_mut_panic() {
        struct MockMessenger(RefCell<Vec<String>>);
        impl Messenger for MockMessenger {
            fn send(&self, _msg: &str) {
                let _a = self.0.borrow_mut();
                let _b = self.0.borrow_mut();
            }
        }
        let messenger = MockMessenger(RefCell::new(vec![]));
        let mut tracker = LimitTracker::new(&messenger, 100);
        tracker.set_value(75);
    }
}
