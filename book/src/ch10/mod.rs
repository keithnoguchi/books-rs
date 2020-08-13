//! [Generic Types], Traits, and Lifetimes
//!
//! [generic types]: https://doc.rust-lang.org/book/ch10-00-generics.html
pub mod sec00;
pub mod sec01;
mod sec02;
pub mod sec03;

pub use sec01::{largest, Point};
pub use sec02::{NewsArticle, Summary, Tweet};
pub use sec03::{first_word, longest, ImportantExcerpt};
