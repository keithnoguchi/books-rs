//! Implemented an [Object-Oriented Design Pattern], part 2
//!
//! Rust's type system based state machine.
//!
//! [object-oriented design pattern]: https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html
//!
//! # Examples
//!
//! ```rust
//! use std::error::Error;
//!
//! use the_book::ch17::sec03::Post;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let mut post = Post::new();
//!
//!     post.add_text("Let's create a blog post");
//!     post.add_text(", now!");
//!     let post = post.request_review();
//!     let post = post.approve();
//!     let want = "Let's create a blog post, now!";
//!     assert_eq!(want, post.content());
//!     println!("Let's publish {}", post.content());
//!     Ok(())
//! }
//! ```

/// [Post] type is the final type with the `content()` method after the
/// approval.
///
/// [post]: struct.Post.html
pub struct Post {
    content: String,
}

/// [DraftPost] is the first type created through the [Post]'s `new()`
/// method.
///
/// [draftpost]: struct.DraftPost.html
/// [post]: struct.Post.html
pub struct DraftPost {
    content: String,
}

/// [PendingReviewPost] is the second type switched from [DraftPost]
/// through the `request_review()` method.
///
/// [pendingreviewpost]: struct.PendingReviewPost.html
/// [draftpost]: struct.DraftPost.html
pub struct PendingReviewPost {
    content: String,
}

impl Post {
    /// `new()` returns [DraftPost] type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use the_book::ch17::sec03::{DraftPost, Post};
    ///
    /// let _post = Post::new();
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    /// `conetnt()` returns `&str` content, which has been
    /// approved for the blog publishing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use the_book::ch17::sec03::Post;
    ///
    /// let mut post = Post::new();
    /// post.add_text("This is how to publish the blog post");
    /// post.add_text(", okay!");
    /// let post = post.request_review();
    /// let post = post.approve();
    /// let want = "This is how to publish the blog post, okay!";
    /// assert_eq!(want, post.content());
    /// ```
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    /// `add_text` add more text to the current draft content.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use the_book::ch17::sec03::Post;
    ///
    /// let mut post = Post::new();
    /// post.add_text("I'm at Chipotle");
    /// post.add_text(" easting burrito bowl");
    /// ```
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    /// `request_review` request a review for the current content
    /// and moves to the [PendingReviewPost] state.
    ///
    /// [PendingReviewPost]: struct.PendingReviewPost.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// use the_book::ch17::sec03::Post;
    ///
    /// let mut post = Post::new();
    /// post.add_text("let's request a review");
    /// let mut _post = post.request_review();
    /// ```
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    /// `approve()` method approves the [PendingReviewPost] type
    /// and change it to [Post] type, which contains `content()`
    /// method call.
    ///
    /// [pendingreviewpost]: struct.PendingReviewPost.html
    /// [post]: struct.Post.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// use the_book::ch17::sec03::Post;
    ///
    /// let mut post = Post::new();
    /// post.add_text("let's create a post and approve it");
    /// post.add_text("!");
    /// let post = post.request_review();
    /// let post = post.approve();
    /// let want = "let's create a post and approve it!";
    /// assert_eq!(want, post.content());
    /// ```
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn post_new_to_draft_post() {
        use super::{DraftPost, Post};

        let want = DraftPost {
            content: String::new(),
        };
        let got = Post::new();
        assert_eq!(want.content, got.content);
    }
    #[test]
    fn post_and_add_text() {
        use super::Post;

        let mut post = Post::new();
        post.add_text("Here is the text for the blog post");
        post.add_text(" and more!");
        let want = "Here is the text for the blog post and more!";
        assert_eq!(want, post.content);
    }
    #[test]
    fn post_add_text_and_request_review() {
        use super::Post;

        let mut post = Post::new();
        post.add_text("Let's create a post");
        post.add_text(" for review!");
        let post = post.request_review();
        let want = "Let's create a post for review!";
        assert_eq!(want, post.content);
    }
    #[test]
    fn post_add_text_request_review_and_approve() {
        use super::Post;

        let mut post = Post::new();
        post.add_text("yeah, now, we can approve the blog post");
        post.add_text(", finally!");
        let post = post.request_review();
        let post = post.approve();
        let want = "yeah, now, we can approve the blog post, finally!";
        assert_eq!(want, post.content());
    }
}
