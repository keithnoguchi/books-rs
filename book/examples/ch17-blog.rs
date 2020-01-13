//! Implemented an [Object-Oriented Design Pattern]
//!
//! [object-oriented design pattern]: https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html
use std::error::Error;

use the_book::ch17::blog::Post;

fn main() -> Result<(), Box<dyn Error>> {
    let mut post = Post::new();

    post.add_text("Let's start the blog\n");
    post.add_text("oops, let's add more text to the first blog entry");
    post.request_review();
    post.approve();
    println!("Here is the published content: '{}'", post.content());
    Ok(())
}
