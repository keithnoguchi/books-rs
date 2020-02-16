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
//! Using trait bounds to conditionally implement methods
//!
//! ```
//! use the_book::ch10::{Pair, Tweet};
//!
//! let p = Pair::new(1, 2);
//! p.cmp_display();
//!
//! let p = Pair::new(
//!     Tweet { username: "A".to_string(), content: "Some tweet".to_string() },
//!     Tweet { username: "B".to_string(), content: "Another tweet".to_string() },
//! );
//! // you can't do this, as [`Tweet`] doesn't implement PartialOrd.
//! // p.cmp_display();
//! ```
//! [`tweet`]: struct.Tweet.html
use core::fmt::{Debug, Display};

/// `impl` based `notify`, which is just a syntax sugar of [`notify2`].
///
/// [`notify2`]: fn.notify2.html
pub fn notify(item: &impl Summary) -> String {
    format!("Breaking news!: {}", item.summarize())
}

/// trait bound [`notify`].
///
/// [`notify`]: fn.notify.html
pub fn notify2<T: Summary>(item: &T) -> String {
    format!("Breaking news!: {}", item.summarize())
}

/// `impl` based `detailed_notify`, which is just a syntax sugar
/// of [`detailed_notify2`].
///
/// [`detailed_notify2`]: fn.detailed_notify2.html
pub fn detailed_notify(item: &(impl Summary + Debug)) -> String {
    format!("Breaking news!: {}\n{:?}", item.summarize(), *item)
}

/// trait bound [`detailed_notify`].
///
/// [`detailed_notify`]: fn.detailed_notify.html
pub fn detailed_notify2<T: Summary + Debug>(item: &T) -> String {
    format!("Breaking news!: {}\n{:?}", item.summarize(), *item)
}

/// Trait generator, which returns trait implementor, example.
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

/// Conditional generic implementor with the trait bound.
pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
