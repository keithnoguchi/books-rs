//! [Traits]: Defining Shared Behavior
//!
//! [traits]: https://doc.rust-lang.org/book/ch10-02-traits.html
//!
//! # Examples
//!
//! ```
//! use the_book::ch10::{Article, Tweet, Summary};
//!
//! let article = Article {
//!     headline: String::from("Headline!"),
//!     content: String::from("Article"),
//! };
//! let tweet = Tweet {
//!     username: String::from("Sam I am"),
//!     content: String::from("tweet, tweet, tweet!"),
//! };
//!
//! assert_eq!(String::from("(Read more...)"), article.summarize());
//! assert_eq!(String::from("tweet, tweet, tweet! @Sam I am"), tweet.summarize());
//! ```
//!
//! Traits as Parameter
//!
//! ```
//! use the_book::ch10::{notify, notify2, Article, Summary};
//!
//! let article = Article {
//!     headline: String::from("Headline!"),
//!     content: String::from("Article"),
//! };
//! assert_eq!(String::from("Breaking news!: (Read more...)"), notify(&article));
//! assert_eq!(notify(&article), notify2(&article));
//! ```
//!
//! Traits as Parameter with the multiple trait bounds.
//!
//! ```
//! use the_book::ch10::{detailed_notify, detailed_notify2, Summary, Tweet};
//!
//! let tweet = Tweet {
//!     username: "I".to_string(),
//!     content: "yep".to_string(),
//! };
//! assert_eq!(
//!     String::from("Breaking news!: yep @I\nTweet { username: \"I\", content: \"yep\" }"),
//!     detailed_notify(&tweet),
//! );
//! assert_eq!(detailed_notify(&tweet), detailed_notify2(&tweet));
//! ```
//!
//! Returning types that implements traits
//!
//! ```
//! use the_book::ch10::{summarizable, Summary};
//!
//! let summary = summarizable("sam i am", "yep, this is the tweet");
//! assert_eq!(
//!     String::from("yep, this is the tweet @sam i am"),
//!     summary.summarize(),
//! );
//! ```
use core::fmt::Debug;

/// `impl` based `notify` is a syntax sugar of `notify2`, trait bounds.
pub fn notify(item: &impl Summary) -> String {
    format!("Breaking news!: {}", item.summarize())
}

pub fn notify2<T: Summary>(item: &T) -> String {
    format!("Breaking news!: {}", item.summarize())
}

pub fn detailed_notify(item: &(impl Summary + Debug)) -> String {
    format!("Breaking news!: {}\n{:?}", item.summarize(), *item)
}

pub fn detailed_notify2<T: Summary + Debug>(item: &T) -> String {
    format!("Breaking news!: {}\n{:?}", item.summarize(), *item)
}

pub fn summarizable(username: &str, content: &str) -> impl Summary {
    Tweet {
        username: username.to_string(),
        content: content.to_string(),
    }
}

/// Default trait implementation.
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

/// [`Summary`] trait implementor.
///
/// [`summary`]: trait.Summary.html
pub struct Article {
    pub headline: String,
    pub content: String,
}

/// `Article` uses the default `summarize` method.
impl Summary for Article {}

/// [`Summary`] trait implementor.
///
/// [`summary`]: trait.Summary.html
#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} @{}", self.content, self.username)
    }
}
