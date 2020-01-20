//! Processing a Series of Items with [Iterators]
//!
//! [iterators]: https://doc.rust-lang.org/book/ch13-02-iterators.html
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut v = vec![1, 2, 3];
    let mut i = v.iter_mut();

    assert_eq!(Some(&mut 1i32), i.next());
    assert_eq!(Some(&mut 2), i.next());
    assert_eq!(Some(&mut 3), i.next());
    assert_eq!(None, i.next());
    assert_eq!(None, i.next());
    assert_eq!(None, i.next());
    assert_eq!(None, i.next());
    println!("{:?}", v);
    Ok(())
}
