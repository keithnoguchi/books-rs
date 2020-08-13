//! [Generic Data Types]
//!
//! [generic data types]: https://doc.rust-lang.org/book/ch10-01-syntax.html
//!
//! # Examples
//!
//! ```
//! use the_book::ch10::sec01::largest;
//!
//! let list = ['a', 'b', 'c', 'd', 'e'];
//! assert_eq!(&'e', largest(&list).unwrap());
//! let list: [i32; 0] = [];
//! assert_eq!(None, largest(&list));
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
pub fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    let mut largest = list.first()?;
    for x in list.iter() {
        if *x > *largest {
            largest = x;
        }
    }
    Some(largest)
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
