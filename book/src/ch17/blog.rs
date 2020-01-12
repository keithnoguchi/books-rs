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
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
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
}

/// Draft [State], which is the only state to allow to change the
/// contents.
#[derive(Debug)]
struct Draft {}

impl State for Draft {}

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
            "This is the first post to my blog.",
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
}
