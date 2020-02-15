//! [Generic Data Types]
//!
//! [generic data types]: https://doc.rust-lang.org/book/ch10-01-syntax.html
//!
//! # Examples
//!
//! ```
//! use the_book::ch10::sec01::{largest, Error};
//!
//! let list = vec![1, 2, 3, 4, 5];
//! assert_eq!(&5, largest(&list).unwrap());
//! let list = vec!['a', 'b', 'c'];
//! assert_eq!(&'c', largest(&list).unwrap());
//! let list: Vec<String> = vec![];
//! assert_eq!(Err(Error::RangeError), largest(&list));
//! ```
//!
//! `Point` generic strucutre with the same types.
//!
//! ```
//! use the_book::ch10::Point;
//!
//! assert_eq!(1, Point { x: 1, y: 2 }.x);
//! assert_eq!(2.0, Point { x: 1.0, y: 2.0}.y);
//! assert_eq!('a', Point { x: 'a', y: 'b'}.x);
//! ```
//!
//! `Point` generic structure with the different types.
//!
//! ```
//! use the_book::ch10::Point;
//!
//! assert_eq!(1, Point { x: 1, y: 'a' }.x);
//! assert_eq!('a', Point { x: 1.0, y: 'a' }.y);
//! ```
//!
//! Generic methods.
//!
//! ```
//! use the_book::ch10::Point;
//!
//! assert_eq!(1, Point { x: 1, y: 'a' }.x());
//! assert_eq!(2.0, Point { x: 1.1, y: 2.0 }.y());
//! ```
use core::cmp::PartialOrd;

/// Error type.
#[derive(Debug, PartialEq)]
pub enum Error {
    RangeError,
}

type Result<T> = core::result::Result<T, Error>;

/// Returns the largest item out of `list` slice.
pub fn largest<T>(list: &[T]) -> Result<&T>
where
    T: PartialOrd,
{
    if list.is_empty() {
        return Err(Error::RangeError);
    }
    let mut largest = &list[0];
    list.iter().for_each(|item| {
        if *largest < *item {
            largest = item
        }
    });
    Ok(largest)
}

/// Double type point structure.
pub struct Point<T, U = T>
where
    T: Copy,
    U: Copy,
{
    pub x: T,
    pub y: U,
}

impl<T, U> Point<T, U>
where
    T: Copy,
    U: Copy,
{
    pub fn x(&self) -> T {
        self.x
    }
    pub fn y(&self) -> U {
        self.y
    }
}
