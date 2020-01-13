//! Implemented an [Object-Oriented Design Pattern]
//!
//! Type based state machine.
//!
//! [object-oriented design pattern]: https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html
use std::error::Error;

use the_book::ch17::x03_blog2::Post;

fn main() -> Result<(), Box<dyn Error>> {
    let mut post = Post::new();

    post.add_text("Let's start the blog\n");
    post.add_text("oops, let's add more text to the first blog entry");
    let post = post.request_review();
    let post = post.approve();
    let mut want = String::from("Let's start the blog\n");
    want.push_str("oops, let's add more text to the first blog entry");
    assert_eq!(&want, post.content());
    println!("Here is the published content: '{}'", post.content());
    Ok(())
}
