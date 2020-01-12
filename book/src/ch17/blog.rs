//! Implemented an [Object-Oriented Design Pattern]
//!
//! [object-oriented design pattern]: https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html

/// Post to abstract the state design pattern through the [State]
/// trait object.
///
/// [state]: trait.State.html
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Self::default()
    }
    /// Retruns the *approved* content back to the caller.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use the_book::ch17::blog::Post;
    ///
    /// let want = String::new();
    /// let got = Post::new();
    /// // Not yet approved.
    /// assert_eq!(&want, got.content());
    /// ```
    pub fn content(&self) -> &str {
        ""
    }
    /// Update the draft text with add_text().  The `content()` method
    /// won't return anything at this state, as the added text has not
    /// been approved yet.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use the_book::ch17::blog::Post;
    ///
    /// let mut post = Post::new();
    /// post.add_text("This is the first blog entry");
    /// // You haven't request a review.
    /// assert_eq!("", post.content());
    /// ```
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    /// Request an approval of the current draft text.  The `content()`
    /// method still won't return anything, as the draft has not been
    /// approved yet.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use the_book::ch17::blog::Post;
    ///
    /// let mut post = Post::new();
    /// post.add_text("Let's start the blog!");
    /// post.add_text(" and another text.");
    /// post.request_review();
    /// // Still waiting for the approval.
    /// assert_eq!("", post.content());
    /// ```
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    /// `approve()` method approves the contents so that `content()`
    /// method can return the *approved* content for this [post].
    ///
    /// [post]: struct.Post.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// use the_book::ch17::blog::Post;
    ///
    /// let mut post = Post::new();
    /// post = Post::new();
    /// post.add_text("This is the first post");
    /// post.add_text("let's add additional text");
    /// post.request_review();
    /// post.approve();
    /// // Now it returns the content, as it's approved.
    /// let mut want = String::from("This is the first post");
    /// want.push_str("let's add additional text");
    /// assert_eq!(&want, post.content());
    /// ```
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

impl Default for Post {
    fn default() -> Self {
        Self {
            content: String::new(),
            state: Some(Box::new(Draft {})),
        }
    }
}

/// [State] trait to implement the state object pattern.
///
/// [state]: trait.State.html
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

/// Draft [state], which is the only state to allow to change the
/// contents.
///
/// [state]: trait.State.html
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

/// PendingReview [state], which has been transitioned from [Draft] state
/// through the [State] request_review() method.
///
/// [state]: trait.State.html
/// [draft]: struct.Draft.html
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // We ignore the multiple review requests.
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Approved {})
    }
}

/// Approved [state] which moves from [PendingReview] state through `approved()`
/// [Post] method.
///
/// [state]: trait.State.html
/// [pendingreview]: struct.PendingReview.html
/// [post]: struct.Post.html
struct Approved {}

impl State for Approved {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn post_initial_content() {
        use super::Post;

        let want = String::new();
        let got = Post::new();
        assert_eq!(&want, got.content());
    }
    #[test]
    fn draft_state_content() {
        use super::Post;

        let got = Post::new();
        assert_eq!("", got.content());
    }
    #[test]
    fn add_text_in_draft_state() {
        use super::Post;

        let texts = vec![
            "This is the first post of my blog.",
            "This is another comments to the first entry",
        ];
        let mut want = String::new();
        for text in &texts {
            want.push_str(text);
        }
        let mut post = Post::new();
        for text in &texts {
            post.add_text(text);
        }
        assert_eq!(want, post.content);
        assert_eq!("", post.content());
    }
    #[test]
    fn request_review_in_draft_state() {
        use super::Post;

        let texts = vec![
            "This is the first post of my blog.",
            "Oops, let's add additional text",
        ];
        let mut want = String::new();
        for text in &texts {
            want.push_str(text);
        }
        let mut post = Post::new();
        for text in &texts {
            post.add_text(text);
        }
        post.request_review();
        assert_eq!(want, post.content);
        assert_eq!("", post.content());
    }
    #[test]
    fn approve_in_pending_review_state() {
        use super::Post;

        let mut post = Post::new();
        post.add_text("Here is the text");
        post.add_text("more text");
        post.request_review();
        post.approve();
        let mut want = String::from("Here is the text");
        want.push_str("more text");
        assert_eq!(&want, post.content());
    }
}
