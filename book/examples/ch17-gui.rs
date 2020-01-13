//! Using [Trait Objects] That Allow for Values of Different Types
//!
//! [trait objects]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
use the_book::ch17::gui::{Draw, Screen};

struct TextBox {
    width: u32,
    height: u32,
    text: String,
}

impl TextBox {
    fn new(width: u32, height: u32, text: &str) -> Self {
        Self {
            width,
            height,
            text: String::from(text),
        }
    }
}

impl Draw for TextBox {
    fn draw(&self) {
        println!(
            "drawing {} in TextBox({} x {})",
            self.text, self.width, self.height,
        );
    }
}

fn main() {
    let text_box = TextBox::new(10, 5, "Input something here!");
    let mut screen = Screen::new(10, 15);
    screen.add_component(Box::new(text_box));
    screen.draw();
}
