//! Using [trait objects] That Allow for Values of Different Types
//!
//! [trait objects]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
//!
//! ## Example
//!
//! ```rust
//! use the_book::ch17::sec01::{Button, Draw, Screen};
//!
//! struct TextBox {
//!     width: u32,
//!     height: u32,
//!     text: String,
//! }
//!
//! impl TextBox {
//!     fn new(width: u32, height: u32, default_text: &str) -> Self {
//!         Self {
//!             width,
//!             height,
//!             text: String::from(default_text),
//!         }
//!     }
//!     fn text(&self) -> &str {
//!         &self.text
//!     }
//!     fn add_text(&mut self, text: &str) {
//!         self.text.push_str(text);
//!     }
//! }
//!
//! impl Draw for TextBox {
//!     fn draw(&self) {
//!         println!(
//!             "drawing {} in TextBox({} x {})",
//!             self.text, self.width, self.height,
//!         );
//!     }
//! }
//!
//! let mut screen = Screen::new(100, 200);
//! let submit = Button::new(10, 5, "submit!");
//! let cancel = Button::new(10, 5, "cancel");
//! let mut text_box = TextBox::new(10, 20, "");
//! text_box.add_text("Some text right here!");
//!
//! screen.add_component(Box::new(submit));
//! screen.add_component(Box::new(cancel));
//! screen.add_component(Box::new(text_box));
//! screen.draw();
//! ```

/// Base [Screen] type to maintain [Draw] trait objects.  It will call
/// all the object through the draw method.
///
/// [screen]: struct.Screen.html
/// [draw]: trait.Draw.html
pub struct Screen {
    width: u32,
    height: u32,
    components: Vec<Box<dyn Draw>>,
}

/// [Draw] trait object to be implemented by all the GUI components.
///
/// [draw]: trait.Draw.html
pub trait Draw {
    fn draw(&self);
}

/// Button type which implements [Draw] trait.
///
/// [draw]: trait.Draw.html
pub struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Screen {
    /// `new()` method creates the base [Screen] type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use the_book::ch17::sec01::{Draw, Screen};
    ///
    /// let screen = Screen::new(100, 200);
    /// screen.draw();
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            components: vec![],
        }
    }
    /// `add_component()` method adds the [Draw] trait object
    /// component.
    ///
    /// # Example
    ///
    /// ```rust
    /// use the_book::ch17::sec01::{Button, Draw, Screen};
    ///
    /// let submit = Button::new(5, 10, "submit");
    /// let cancel = Button::new(5, 10, "cancel");
    /// let mut screen = Screen::new(15, 15);
    ///
    /// screen.add_component(Box::new(submit));
    /// screen.add_component(Box::new(cancel));
    /// screen.draw();
    /// ```
    pub fn add_component(&mut self, c: Box<dyn Draw>) {
        self.components.push(c);
    }
}

impl Draw for Screen {
    /// `draw()` method draws the screen by calling all
    /// the `components` added through `add_component()`
    /// method call.
    ///
    /// # Example
    ///
    /// ```rust
    /// use the_book::ch17::sec01::{Button, Draw, Screen};
    ///
    /// let mut screen = Screen::new(200, 400);
    /// let submit = Button::new(5, 10, "submit!");
    ///
    /// screen.add_component(Box::new(submit));
    /// screen.draw();
    /// ```
    fn draw(&self) {
        println!("drawing those in ({} x {}) screen", self.width, self.height,);
        for c in &self.components {
            c.draw();
        }
    }
}

impl Button {
    /// `new()` method creates the `Button` [Draw] trait object.
    ///
    /// [draw]: trait.Draw.html
    ///
    /// # Example
    ///
    /// ```rust
    /// use the_book::ch17::sec01::{Button, Draw, Screen};
    ///
    /// let submit = Button::new(5, 1, "Submit!");
    /// let mut screen = Screen::new(100, 200);
    ///
    /// screen.add_component(Box::new(submit));
    /// screen.draw();
    /// ```
    pub fn new(width: u32, height: u32, label: &str) -> Self {
        Self {
            width,
            height,
            label: String::from(label),
        }
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "drawing Buttn{{{}}} in ({}x{})",
            self.label, self.width, self.height,
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn screen_new() {
        use super::Screen;

        let screen = Screen::new(10, 20);
        assert_eq!(10, screen.width);
        assert_eq!(20, screen.height);
        assert_eq!(0, screen.components.len());
    }
    #[test]
    fn screen_draw() {
        use super::{Draw, Screen};

        let screen = Screen::new(5, 20);
        assert_eq!(5, screen.width);
        assert_eq!(20, screen.height);
        screen.draw();
    }
}
