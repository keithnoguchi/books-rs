//! [`Void`] and [`Event`] message enum
//!
//! [`void`]: enum.Void.html
//! [`event`]: enum.Event.html
use async_std::net::TcpStream;

/// `Event` enum for the communication between [`Broker`] and [`Reader`].
///
/// [`broker`]: ../broker/struct.Broker.html
/// [`reader`]: ../reader/struct.Reader.html
use std::sync::Arc;

use super::Cancel;

/// `Void` enum for the cancellation message.
pub enum Void {}

/// `Event` for the broker and reader communications.
pub enum Event {
    /// `Join` event is sent by `Reader` task when user `name` connected to
    /// the server.
    Join {
        name: String,
        stream: Arc<TcpStream>,
        cancel: Cancel,
    },
    /// `Message` event is sent by `Reader` task when user `from` sends
    /// a `msg` message.
    Message { from: String, msg: String },
}
