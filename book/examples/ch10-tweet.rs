//! [Traits]: Defining Shared Behavior
//!
//! [traits]: https://doc.rust-lang.org/book/ch10-02-traits.html
use the_book::ch10::{Article, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("yep, this is me"),
        content: String::from("This is the most important tweet"),
    };
    let article = Article {
        headline: String::from("HEADLINE!!!!"),
        content: String::from("yep, it's really important, indeed"),
    };

    assert_eq!(
        String::from("This is the most important tweet @yep, this is me"),
        tweet.summarize(),
    );
    assert_eq!(String::from("(Read more...)"), article.summarize(),);
}
