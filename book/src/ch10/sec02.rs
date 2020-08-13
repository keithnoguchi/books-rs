//! [Traits]: Defining Shared Behavior
//!
//! [traits]: https://doc.rust-lang.org/book/ch10-02-traits.html

/// Summary trait
pub trait Summary {
    /// Required implementation of `Summary` trait.
    fn summarize_author(&self) -> String;

    /// `summarize` method with the default implementation.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

/// `NewsArticle`, a `Summary` trait example with the default implementations
///
/// # Example
///
/// ```
/// use the_book::ch10::{NewsArticle, Summary};
///
/// let article = NewsArticle::new("Breaking news!", "Keith Noguchi");
/// assert_eq!("(Read more from Keith Noguchi...)", &article.summarize());
/// ```
#[derive(Default)]
pub struct NewsArticle {
    _headline: String,
    author: String,
    _location: String,
    _content: String,
}

impl NewsArticle {
    pub fn new(headline: &str, author: &str) -> Self {
        Self {
            _headline: headline.into(),
            author: author.into(),
            ..Self::default()
        }
    }
}

// Use the default `summarize()` method.
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

/// `Tweet`, a `Summary` trait example.
///
/// # Example
///
/// ```
/// use the_book::ch10::{Summary, Tweet};
///
/// let tweet = Tweet::new("keithnoguchi", "Ho, ho, ho!");
/// assert_eq!("keithnoguchi: Ho, ho, ho!", &tweet.summarize());
/// ```
#[derive(Default)]
pub struct Tweet {
    username: String,
    content: String,
    _reply: bool,
    _retweet: bool,
}

impl Tweet {
    pub fn new(username: &str, content: &str) -> Self {
        Self {
            username: username.into(),
            content: content.into(),
            ..Self::default()
        }
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
