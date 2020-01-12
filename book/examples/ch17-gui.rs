//! Using [Trait Objects] That Allow for Values of Different Types
//!
//! [trait objects]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
use the_book::ch17::gui::{Draw, Screen};

fn main() {
    let screen = Screen::new(10, 15);
    screen.draw();
}
