//! [tonic] play ground
//!
//! [tonic]: https://lib.rs/tonic

/// Service modules.
mod data;
mod greet;
mod route;
pub use greet::GreeterService;
pub use route::RouteGuideService;

/// Auto-generated client and server modules
mod autogen;
pub(crate) use autogen::hello::greeter_server::{Greeter, GreeterServer};
pub use autogen::hello::{greeter_client::GreeterClient, HelloRequest, HelloResponse};
pub(crate) use autogen::route::{
    route_guide_server::{RouteGuide, RouteGuideServer},
    Feature, Point,
};
