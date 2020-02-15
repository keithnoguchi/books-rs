//! [Generic Types], Traits, and Lifetimes
//!
//! # Examples
//!
//! ```
//! use the_book::ch10::sec00::largest;
//!
//! let list = vec![1, 2, 3, 4, 5];
//! assert_eq!(5, largest(&list));
//!
//! let list = vec![5, 4, 3, 2, 1];
//! assert_eq!(5, largest(&list));
//!
//! let list = vec![-1, -2, -3, -4, -5];
//! assert_eq!(-1, largest(&list));
//!
//! let list = vec![-5, -4, -3, -2, -1];
//! assert_eq!(-1, largest(&list));
//! ```
//! [generic types]: https://doc.rust-lang.org/book/ch10-00-generics.html
pub fn largest(list: &[i32]) -> i32 {
    let mut largest = -1;
    list.iter().for_each(|&item| {
        if largest < item {
            largest = item;
        }
    });
    largest
}
