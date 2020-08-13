//! [Traits]: Defining Shared Behavior
//!
//! [traits]: https://doc.rust-lang.org/book/ch10-02-traits.html
use the_book::ch10::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet::new("keithnoguchi", "Yo yo!");
    let news = NewsArticle::new("Breaking news!", "Keith Noguchi");
    test_summarize("keithnoguchi: Yo yo!", &tweet);
    test_summarize("(Read more from Keith Noguchi...)", &news);
}

/// Traits as parameter.  We can borrow the variables.
fn test_summarize(want: &str, article: &impl Summary) {
    assert_eq!(want, &article.summarize());
}
