//! Processing a Series of Items with [Iterators]
//!
//! [iterators]: https://doc.rust-lang.org/book/ch13-02-iterators.html
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let v = vec![1i32, 2, 3];
    let mut i = v.iter();

    assert_eq!(Some(&1i32), i.next());
    assert_eq!(Some(&2i32), i.next());
    assert_eq!(Some(&3i32), i.next());
    assert_eq!(None, i.next());
    assert_eq!(None, i.next());
    assert_eq!(None, i.next());
    assert_eq!(None, i.next());
    println!("{:?}", v);
    Ok(())
}
