//! Using [trait objects] That Allow for Values of Different Types
//!
//! [trait objects]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html

/// Base [Screen] type to maintain [Draw] trait objects.  It will call
/// all the object through the draw method.
///
/// [screen]: struct.Screen.html
/// [draw]: trait.Draw.html
///
/// ## Example
///
/// ```rust
/// use the_book::ch17::gui::{Draw, Screen};
///
/// let screen = Screen::new(10, 20);
/// screen.draw();
/// ```
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

impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            components: vec![],
        }
    }
}

impl Draw for Screen {
    fn draw(&self) {
        for c in &self.components {
            c.draw();
        }
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
