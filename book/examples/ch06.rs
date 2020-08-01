//! [Enums] and Pattern Matching
//!
//! [enums]: https://doc.rust-lang.org/book/ch06-00-enums.html
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    assert_eq!(Some(6), six);
    assert_eq!(None, none);
}

fn plus_one(value: Option<u8>) -> Option<u8> {
    value.and_then(|x| Some(x + 1))
}
