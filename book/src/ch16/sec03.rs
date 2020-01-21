//! Counter with [Mutex<T>]
//!
//! [mutex<t>]: https://doc.rust-lang.org/book/ch16-03-shared-state.html
//!
//! # Examples
//!
//! ```rust
//! use std::sync::Arc;
//! use std::thread::{self, Result};
//! use the_book::ch16::sec03::Counter;
//!
//! fn main() -> Result<()> {
//!     let mut handlers = Vec::with_capacity(10);
//!     let counter = Arc::new(Counter::new(0));
//!
//!     for _ in 0..1000 {
//!         let counter = counter.clone();
//!         let handler = thread::spawn(move || counter.inc(3));
//!         handlers.push(handler);
//!     }
//!     for handler in handlers {
//!         handler.join()?;
//!     }
//!     assert_eq!(3000, counter.get());
//!     Ok(())
//! }
//! ```
use std::ops::{AddAssign, SubAssign};
use std::sync::Mutex;

/// [Sync] Counter through [Mutex<T>] zerotype.
///
/// [sync]: https://doc.rust-lang.org/std/marker/trait.Sync.html
/// [mutex<t>]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
pub struct Counter<T: Copy + AddAssign + SubAssign>(Mutex<T>);

impl<T: Copy + AddAssign + SubAssign> Counter<T> {
    /// `new()` method to initialize the counter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use the_book::ch16::sec03::Counter;
    ///
    /// let counter = Counter::new(100);
    /// assert_eq!(100, counter.get());
    /// ```
    pub fn new(val: T) -> Self {
        let counter = Mutex::new(val);
        Self(counter)
    }
    /// `get()` method gets the current value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use the_book::ch16::sec03::Counter;
    ///
    /// let counter = Counter::new(1000);
    /// assert_eq!(1000, counter.get());
    /// ```
    ///
    /// # Panics
    ///
    /// It panics when it can't get a lock.
    pub fn get(&self) -> T {
        let num = self.0.lock().expect("cannot get lock in get()");
        *num
    }
    /// `inc()` method increments counter value by one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::sync::Arc;
    /// use std::thread::{self, Result};
    /// use the_book::ch16::sec03::Counter;
    ///
    /// fn main() -> Result<()> {
    ///     let mut handlers = Vec::with_capacity(100);
    ///     let counter = Arc::new(Counter::new(0));
    ///
    ///     for _ in 0..100 {
    ///         let counter = counter.clone();
    ///         let handler = thread::spawn(move || { counter.inc(1); });
    ///         handlers.push(handler);
    ///     }
    ///     for handler in handlers {
    ///         handler.join()?;
    ///     }
    ///     assert_eq!(100, counter.get());
    ///     Ok(())
    /// }
    /// ```
    /// # Panics
    ///
    /// It panics when it cannot get the lock.
    ///
    pub fn inc(&self, value: T) {
        let mut num = self.0.lock().expect("cannot get lock in inc()");
        *num += value;
    }
    /// `dec()` method decrements the provided value from the counter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::sync::Arc;
    /// use std::thread::{self, Result};
    ///
    /// use the_book::ch16::sec03::Counter;
    ///
    /// fn main() -> Result<()> {
    ///     let mut handlers = Vec::with_capacity(1_000);
    ///     let counter = Arc::new(Counter::new(100_000));
    ///
    ///     for _ in 0..100 {
    ///         let counter = counter.clone();
    ///         let handler = thread::spawn(move || counter.dec(2));
    ///         handlers.push(handler);
    ///     }
    ///     for handler in handlers {
    ///         handler.join()?;
    ///     }
    ///     assert_eq!(99_800, counter.get());
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// It panics when it cannot get the lock.
    ///
    pub fn dec(&self, value: T) {
        let mut num = self.0.lock().expect("cannot get lock in dec()");
        *num -= value;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new() {
        use super::Counter;

        let counter = Counter::new(0);
        assert_eq!(0, counter.get());
    }
    #[test]
    fn inc() -> std::thread::Result<()> {
        use super::Counter;
        use std::sync::Arc;
        use std::thread;

        let mut handlers = Vec::with_capacity(1000);
        let counter = Arc::new(Counter::new(0));

        for _ in 0..1000 {
            let counter = counter.clone();
            let handler = thread::spawn(move || counter.inc(3));
            handlers.push(handler);
        }
        for handler in handlers {
            handler.join()?;
        }
        assert_eq!(3000, counter.get());
        Ok(())
    }
    #[test]
    fn dec() -> std::thread::Result<()> {
        use super::Counter;
        use std::sync::Arc;
        use std::thread;

        let mut handlers = Vec::with_capacity(1000);
        let counter = Arc::new(Counter::new(1_000i32));

        for _ in 0..1000 {
            let counter = counter.clone();
            let handler = thread::spawn(move || counter.dec(3));
            handlers.push(handler);
        }
        for handler in handlers {
            handler.join()?;
        }
        assert_eq!(-2_000, counter.get());
        Ok(())
    }
}
