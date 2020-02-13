//! [crossbeam-queue] examples
//!
//! # Examples
//!
//! [crossbeam_queue::ArrayQueue]
//!
//! ```
//! use crossbeam_queue::{PopError, PushError, ArrayQueue};
//!
//! let q = ArrayQueue::new(2);
//! assert_eq!(Ok(()), q.push('a'));
//! assert_eq!(Ok(()), q.push('b'));
//! assert_eq!(Err(PushError('b')), q.push('b'));
//! assert_eq!(Ok('a'), q.pop());
//! assert_eq!(Ok('b'), q.pop());
//! assert_eq!(Err(PopError), q.pop());
//! ```
//!
//! [crossbeam_queue::SegQueue]
//!
//! ```
//! use crossbeam_queue::{PopError, SegQueue};
//!
//! let q = SegQueue::new();
//! q.push('a');
//! q.push('b');
//! assert_eq!(Ok('a'), q.pop());
//! assert_eq!(Ok('b'), q.pop());
//! assert_eq!(Err(PopError), q.pop());
//! ```
//! [crossbeam-queue]: https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-queue
//! [crossbeam_queue::ArrayQueue]: https://docs.rs/crossbeam-queue/latest/crossbeam_queue/struct.ArrayQueue.html
//! [crossbeam_queue::SeqQueue]: https://docs.rs/crossbeam-queue/latest/crossbeam_queue/struct.SegQueue.html
