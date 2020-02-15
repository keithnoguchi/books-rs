//! [Generic Types], Traits, and Lifetimes
//!
//! [generic types]: https://doc.rust-lang.org/book/ch10-00-generics.html
use the_book::ch10::largest;
use the_book::ch10::sec01::Error;

fn main() {
    let list = vec![1, 2, 3, 4, 5];
    assert_eq!(5, *largest(&list).unwrap());
    let list: Vec<String> = vec![];
    assert_eq!(Err(Error::RangeError), largest(&list));
}
