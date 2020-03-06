//! [Task] and [spawn] examples
//! # Examples
//!
//! ```
//! use async_task::spawn;
//! use crossbeam_channel::unbounded;
//!
//! let (s, r) = unbounded();
//! let future = async {
//!     println!("Hello, world!");
//! };
//! let schedule = move |task| s.send(task).unwrap();
//!
//! let (task, handle) = spawn(future, schedule, ());
//! ```
