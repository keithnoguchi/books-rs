//! [tonic] play ground
//!
//! [tonic]: https://lib.rs/tonic

/// Service modules.
mod greet;
pub use greet::GreeterService;

/// Auto-generated client and server modules
pub mod autogen;
pub use autogen::hello::greeter_client::GreeterClient;
pub use autogen::hello::greeter_server::GreeterServer;
pub use autogen::hello::{self, greeter_server::Greeter};
pub use autogen::route::route_guide_server;
