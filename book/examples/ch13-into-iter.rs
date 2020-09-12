//! Processing a Series of Items with [Iterators]
//!
//! [iterators]: https://doc.rust-lang.org/book/ch13-02-iterators.html
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let v = vec![1, 2, 3];
    let mut i = v.into_iter();
    // You can't borrow vector v here because
    // it's already moved into iterator i.
    //println!("{:?}", v);
    assert_eq!(Some(1), i.next());
    assert_eq!(Some(2), i.next());
    assert_eq!(Some(3), i.next());
    assert_eq!(None, i.next());
    assert_eq!(None, i.next());
    assert_eq!(None, i.next());
    assert_eq!(None, i.next());
    Ok(())
}
