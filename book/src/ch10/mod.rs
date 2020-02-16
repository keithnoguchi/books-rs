//! [Generic Types], Traits, and Lifetimes
//!
//! [generic types]: https://doc.rust-lang.org/book/ch10-00-generics.html
pub mod sec00;
pub mod sec01;
pub mod sec02;
pub mod sec03;

pub use sec01::{largest, Point};
pub use sec02::{detailed_notify, detailed_notify2, notify, notify2, summarizable};
pub use sec02::{Article, Pair, Summary, Tweet};
pub use sec03::longest;
