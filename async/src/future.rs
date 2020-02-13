//! [Simple future] trait example
//!
//! [simple future]: https://rust-lang.github.io/async-book/02_execution/02_future.html
use std::task::Poll;

/// Simple [Future]
///
/// [future]: https://doc.rust-lang.org/std/future/trait.Future.html
pub trait Future {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}
