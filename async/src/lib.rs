//! [Async Book] examples
//!
//! [async book]: https://rust-lang.github.io/async-book/
mod executor;
mod future;
mod timer;

pub use executor::new_executor_and_spawner;
pub use timer::TimerFuture;
