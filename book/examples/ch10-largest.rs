//! [Generic Types], Traits, and Lifetimes
//!
//! [generic types]: https://doc.rust-lang.org/book/ch10-00-generics.html
use the_book::ch10::largest;

fn main() {
    let list = [];
    assert_eq!(None, largest(&list));
    let list = [1];
    assert_eq!(&1, largest(&list).unwrap());
    let list = [1, 2, 3, 4, 5];
    assert_eq!(&5, largest(&list).unwrap());
    let list = [5, 4, 3, 2, 1];
    assert_eq!(&5, largest(&list).unwrap());
}
