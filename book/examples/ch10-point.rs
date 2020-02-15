//! [Generic Data Types]
//!
//! [generic data types]: https://doc.rust-lang.org/book/ch10-01-syntax.html
use the_book::ch10::Point;

fn main() {
    let p = Point { x: 1, y: 2 };
    assert_eq!(1, p.x());
    assert_eq!(2, p.y());
    let p = Point { x: 1, y: 2.0 };
    assert_eq!(1, p.x());
    assert_eq!(2.0, p.y());
    let p = Point { x: "one", y: "two" };
    assert_eq!("one", p.x());
    assert_eq!("two", p.y());
}
