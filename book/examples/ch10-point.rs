//! [Generic Data Types]
//!
//! [generic data types]: https://doc.rust-lang.org/book/ch10-01-syntax.html
use the_book::ch10::Point;

fn main() {
    let p = Point::new(1, 2);
    assert_eq!(&1, p.x());
    assert_eq!(&2, p.y());
    let p = Point::new(1, 2.0);
    assert_eq!(&1, p.x());
    assert_eq!(&2.0, p.y());
    let p = Point::new("one", "two");
    assert_eq!(&"one", p.x());
    assert_eq!(&"two", p.y());
    let p = Point::new(String::from("hello"), 1.1);
    let q = Point::new(1, String::from("world"));
    let r = p.mixup(q);
    assert_eq!("hello", r.x());
    assert_eq!("world", r.y());
}
