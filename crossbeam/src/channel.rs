//! [crossbeam-channel]: Multi Producer and Multi Consumer channels for message passing
//!
//! [crossbeam-channel]: https://lib.rs/crates/crossbeam-channel
//!
//! # Examples
//!
//! An unbounded channel:
//!
//! ```
//! // Unbounded channel:
//! let (tx, rx) = crossbeam_channel::unbounded();
//!
//! // Single message.
//! tx.send("Hello, world!").unwrap();
//!
//! // Single reception.
//! assert_eq!(Ok("Hello, world!"), rx.recv());
//! ```
//!
//! A bounded channel:
//!
//! ```
//! // Bounded channel.
//! let (tx, rx) = crossbeam_channel::bounded(5);
//!
//! // Sending just four messages to avoid the blocking.
//! for i in (0..4) {
//!     tx.send(format!("{}", i)).unwrap();
//! }
//! for i in (0..4) {
//!     let want = format!("{}", i);
//!     assert_eq!(Ok(want), rx.recv());
//! }
//! ```
//!
//! An unbounded channel with lots of messages:
//!
//! ```
//! let (tx, rx) = crossbeam_channel::unbounded();
//!
//! for i in (0..1_000_000) {
//!     tx.send(format!("{}", i)).unwrap();
//! }
//!
//! for i in (0..1_000_000) {
//!     let want = format!("{}", i);
//!     assert_eq!(Ok(want), rx.recv());
//! }
//! ```
//!
//! A zero-capacity channel with the communication coordination:
//!
//! ```
//! // Special zero-capacity channel.
//! let (tx, rx) = crossbeam_channel::bounded(0);
//!
//! // Spawn a thread to avoid the current thread blocking.
//! std::thread::spawn(move || tx.send("Hi!").unwrap());
//!
//! // Receiving a message.
//! assert_eq!(Ok("Hi!"), rx.recv());
//! ```
//!
//! Sharing channels:
//!
//! ```
//! let (tx1, rx1) = crossbeam_channel::bounded(0);
//! let (tx2, rx2) = (tx1.clone(), rx1.clone());
//!
//! std::thread::spawn(move || {
//!     let got = rx2.recv().unwrap();
//!     tx2.send(got).unwrap();
//! });
//!
//! tx1.send(1).unwrap();
//! let got = rx1.recv();
//! assert_eq!(Ok(1), got);
//! ```
//!
//! Note that cloning only creates a new handle to the same
//! sending or receiving side.
//!
//! ```
//! let (tx1, rx1) = crossbeam_channel::unbounded();
//! let (tx2, rx2) = (tx1.clone(), rx1.clone());
//! let (tx3, rx3) = (tx2.clone(), rx1.clone());
//!
//! tx1.send(10).unwrap();
//! tx2.send(20).unwrap();
//! tx3.send(30).unwrap();
//!
//! assert_eq!(Ok(10), rx2.recv());
//! assert_eq!(Ok(20), rx1.recv());
//! assert_eq!(Ok(30), rx1.recv());
//! ```
//!
//! Disconnection:
//!
//! ```
//! let (tx, rx) = crossbeam_channel::unbounded();
//!
//! tx.send(1).unwrap();
//! tx.send(2).unwrap();
//! tx.send(3).unwrap();
//!
//! // The only sender is dropped, disconnecting the channel.
//! drop(tx);
//!
//! assert_eq!(Ok(1), rx.recv());
//! assert_eq!(Ok(2), rx.recv());
//! assert_eq!(Ok(3), rx.recv());
//!
//! // There are no more messages in the channel.
//! assert!(rx.is_empty());
//!
//! // Note that calling `rx.recv()` does not block.
//! // Instead, `Err(RecvError)` is returned immediately.
//! assert_eq!(Err(crossbeam_channel::RecvError), rx.recv());
//! ```
//!
//! Blocking operations:
//!
//! ```
//! let (tx, rx) = crossbeam_channel::bounded(1);
//!
//! tx.send("foo").unwrap();
//! assert_eq!(Ok("foo"), rx.recv());
//!
//! // This call would block because the channel is empty.
//! // rx.recv();
//!
//! assert_eq!(Err(crossbeam_channel::TryRecvError::Empty), rx.try_recv());
//!
//! drop(tx);
//!
//! assert_eq!(Err(crossbeam_channel::RecvError), rx.recv());
//! ```
//!
//! Iteration:
//!
//! ```
//! let (tx, rx) = crossbeam_channel::unbounded();
//!
//! std::thread::spawn(move || {
//!     tx.send(1).unwrap();
//!     tx.send(2).unwrap();
//!     tx.send(3).unwrap();
//!     drop(tx);
//! });
//!
//! // Collecting all messages from the channel.
//! // Note that the call to `collect` *blocks* until
//! // the sender is dropped.
//! let got: Vec<_> = rx.iter().collect();
//! assert_eq!(vec![1, 2, 3], got);
//! ```
//!
//! Non-blocking iteration:
//!
//! ```
//! let (tx, rx) = crossbeam_channel::unbounded();
//! tx.send(1).unwrap();
//! tx.send(2).unwrap();
//! tx.send(3).unwrap();
//!
//! let got: Vec<_> = rx.try_iter().collect();
//! assert_eq!(vec![1, 2, 3], got);
//! ```
//!
//! Selection:
//!
//! ```
//! let (tx1, rx1) = crossbeam_channel::unbounded();
//! let (tx2, rx2) = crossbeam_channel::unbounded();
//!
//! std::thread::spawn(move || tx1.send(10).unwrap());
//! std::thread::spawn(move || tx2.send(20).unwrap());
//!
//! crossbeam_channel::select! {
//!     recv(rx1) -> msg => assert_eq!(Ok(10), msg),
//!     recv(rx2) -> msg => assert_eq!(Ok(20), msg),
//!     default(std::time::Duration::from_secs(1)) => println!("timed out"),
//! }
//! ```
//!
//! Extra channels
//!
//! ```
//! let start = std::time::Instant::now();
//! let ticker = crossbeam_channel::tick(std::time::Duration::from_millis(50));
//! let timeout = crossbeam_channel::after(std::time::Duration::from_secs(1));
//!
//! loop {
//!     crossbeam_channel::select! {
//!         recv(ticker) -> _ => println!("elapsed: {:?}", start.elapsed()),
//!         recv(timeout) -> _ => break,
//!     }
//! }
//! ```
