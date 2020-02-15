//! How to use [Pinning]
//!
//! # Examples
//!
//! ```
//! use futures::{
//!     pin_mut,
//!     executor::block_on,
//! };
//!
//! use async_book::execute_unpin_future;
//!
//! let future = async { 1 + 2 };
//!
//! # // This won't work
//! # // block_on(execute_unpin_future(future));
//!
//! let future = Box::pin(future);
//! block_on(execute_unpin_future(future));
//!
//! let future = async { 4 + 5 };
//! pin_mut!(future);
//! block_on(execute_unpin_future(future));
//! ```
//! [pinning]: https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
use std::future::Future;

pub async fn execute_unpin_future(future: impl Future<Output = u32> + Unpin) -> u32 {
    future.await
}
