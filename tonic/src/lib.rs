//! [tonic] play ground
//!
//! [tonic]: https://lib.rs/tonic

/// Service modules.
mod greet;
pub use greet::GreeterService;

/// Auto-generated client and server modules
pub mod autogen;
pub(crate) use autogen::hello::greeter_server::{Greeter, GreeterServer};
pub use autogen::hello::{self, greeter_client::GreeterClient};
pub use autogen::route::route_guide_server;
