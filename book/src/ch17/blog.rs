//! Implemented an [Object-Oriented Design Pattern]
//!
//! [object-oriented design pattern]: https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html

/// Post to abstract the state design pattern through the [State]
/// trait object.
///
/// [state]: trait.State.html
pub struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}

impl Post {
    pub fn new() -> Self {
        Self::default()
    }
    /// # Examples
    ///
    /// ```rust
    /// use the_book::ch17::blog::Post;
    ///
    /// let want = String::new();
    /// let got = Post::new();
    /// assert_eq!(&want, got.content());
    /// ```
    pub fn content(&self) -> &str {
        match &self.state {
            None => "",
            Some(state) => state.content(),
        }
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
    fn content(&self) -> &str {
        ""
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
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
}

/// PendingReview [state], which has been transitioned from [Draft] state
/// through the [State] request_review() method.
///
/// [state]: trait.State.html
/// [draft]: struct.Draft.html
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Self {})
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
        match got.state {
            None => panic!("unexpected None state"),
            Some(got) => assert_eq!("", got.content()),
        }
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
}
