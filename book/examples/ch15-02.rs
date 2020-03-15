//! Treating Smart Pointer Like Regular References with [the Deref trait]
//!
//! [the deref trait]: https://doc.rust-lang.org/book/ch15-02-deref.html
use the_book::ch15::sec02::MyBox;

fn main() {
    let x = 5;
    let mut y = MyBox::new(x);

    println!("x={}, y={}", x, *y);
    *y = 9;
    println!("x={}, y={}", x, *y);
}
