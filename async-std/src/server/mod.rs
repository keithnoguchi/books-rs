//! A Chat Server
//!
//! A chat [`Server`] [example]
//!
//! # Examples
//!
//! ```no_run
//! use async_std::task;
//! use async_std_book::Server;
//!
//! type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
//!
//! fn main() -> Result<(), Error> {
//!     let addr = std::env::args()
//!         .skip(1)
//!         .next()
//!         .unwrap_or(String::from("[::1]:8000"));
//!     task::block_on(Server::new(addr).run())
//! }
//! ```
//! [`server`]: struct.Server.html
//! [example]: https://book.async.rs/tutorial/index.html
use async_std::task;
use futures::channel::mpsc;
use std::sync::Arc;
use std::time::Duration;

/// Sub modules.
mod broker;
mod listen;
mod message;
mod read;
mod write;

/// module local aliases.
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;
type Cancel = Receiver<message::Void>;

use super::Result;
use broker::Broker;
use listen::Listener;

/// A chat `Server` type.
pub struct Server {
    name: String,
    interval: Duration,
    addr: String,
}

impl Server {
    /// `new` creates a new server instance.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_std_book::Server;
    ///
    /// let addr = std::env::args()
    ///     .skip(1)
    ///     .next()
    ///     .unwrap_or(String::from("localhost:8000"));
    /// let _server = Server::new(addr);
    /// ```
    pub fn new(addr: String) -> Self {
        Self {
            name: String::from("server"),
            interval: Duration::from_secs(1),
            addr,
        }
    }
    /// `run` creates a `Future` instance which executes all the
    /// business logic.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_std::task;
    /// use async_std_book::Server;
    ///
    /// let addr = std::env::args()
    ///     .skip(1)
    ///     .next()
    ///     .unwrap_or(String::from("localhost:8000"));
    /// task::block_on(Server::new(addr).run());    
    /// ```
    pub async fn run(self) -> Result<()> {
        eprintln!("[{}] starting", self.name);
        let mut interval = self.interval;
        let mut tasks = Vec::new();
        let addr = Arc::new(self.addr);
        eprintln!("[{}] started", self.name);
        loop {
            let (tx, rx) = mpsc::unbounded();
            tasks.push(task::spawn(Listener::new(tx, addr.clone()).run()));
            tasks.push(task::spawn(Broker::new(rx).run()));
            while let Some(task) = tasks.pop() {
                let id = task.task().id();
                if let Err(err) = task.await {
                    eprintln!("[{}] task[{}] error: {}", self.name, id, err);
                }
            }
            eprintln!("[{}] sleeping for {:?}", self.name, interval);
            task::sleep(interval).await;
            interval *= 2;
            eprintln!("[{}] restarting", self.name);
        }
    }
}
