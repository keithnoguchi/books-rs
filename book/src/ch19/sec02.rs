//! [Advanced Traits]
//!
//! # Examples
//!
//! Simplified [`Iterator`] example
//!
//! ```
//! use the_book::ch19::sec02::{Counter, Iterator};
//!
//! let mut counter = Counter::new(3);
//! assert_eq!(Some(1), counter.next());
//! assert_eq!(Some(2), counter.next());
//! assert_eq!(Some(3), counter.next());
//! assert_eq!(None, counter.next());
//! ```
//!
//! Operator Overloading example
//!
//! ```
//! use the_book::ch19::sec02::Point;
//!
//! let p1 = Point(0, 1);
//! let p2 = Point(9, 3);
//! let p3 = p1 + p2;
//! assert_eq!(9, p3.0);
//! assert_eq!(4, p3.1);
//! ```
//! [Default Generic Type Parameters] example
//!
//! ```
//! use the_book::ch19::sec02::{Meters, Millimeters};
//!
//! let meter = Meters(10.0);
//! let milli = Millimeters(2.0);
//!
//! let got = meter + milli;
//! assert_eq!(Meters(10.002), got);
//!
//! let got = milli + meter;
//! assert_eq!(Millimeters(10_002.0), got);
//! ```
//! [`iterator`]: trait.Iterator.html
//! [advanced traits]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
//! [default generic type parameters]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#default-generic-type-parameters-and-operator-overloading

/// Simple `Iterator` example.
///
/// [`iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

/// Simple `Counter` to demonstrate [`Iterator`] implementation.
///
/// # Examples
///
/// ```
/// use the_book::ch19::sec02::{Counter, Iterator};
///
/// let mut counter = Counter::new(3);
/// assert_eq!(Some(1), counter.next());
/// assert_eq!(Some(2), counter.next());
/// assert_eq!(Some(3), counter.next());
/// assert_eq!(None, counter.next());
/// ```
/// [`iterator`]: trait.Iterator.html
pub struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    pub fn new(max: u32) -> Self {
        Self { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

/// [Default Generic Type Parameters] and Operator Overloading
///
/// # Examples
///
/// ```
/// use the_book::ch19::sec02::Point;
///
/// let p1 = Point(0, 1);
/// let p2 = Point(9, 3);
/// let p3 = p1 + p2;
/// assert_eq!(9, p3.0);
/// assert_eq!(4, p3.1);
/// ```
/// [default generic type parameters]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#default-generic-type-parameters-and-operator-overloading
#[derive(Debug, PartialEq)]
pub struct Point(pub i32, pub i32);

use std::ops::Add;

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

/// [Default Generic Type Parameters] and Operator Overloading
///
/// # Examples
///
/// ```
/// use the_book::ch19::sec02::{Meters, Millimeters};
///
/// let meter = Meters(10.0);
/// let milli = Millimeters(2.0);
///
/// let got = meter + milli;
/// assert_eq!(Meters(10.002), got);
///
/// let got = milli + meter;
/// assert_eq!(Millimeters(10_002.0), got);
/// ```
/// [default generic type parameters]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#default-generic-type-parameters-and-operator-overloading
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Millimeters(pub f32);

impl Add<Meters> for Millimeters {
    type Output = Self;
    fn add(self, other: Meters) -> Self {
        Self(self.0 + other.0 * 1000.0)
    }
}

/// [Default Generic Type Parameters] and Operator Overloading
///
/// # Examples
///
/// ```
/// use the_book::ch19::sec02::{Meters, Millimeters};
///
/// let meter = Meters(10.0);
/// let milli = Millimeters(2.0);
///
/// let got = meter + milli;
/// assert_eq!(Meters(10.002), got);
///
/// let got = milli + meter;
/// assert_eq!(Millimeters(10_002.0), got);
/// ```
/// [default generic type parameters]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#default-generic-type-parameters-and-operator-overloading
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Meters(pub f32);

impl Add<Millimeters> for Meters {
    type Output = Self;
    fn add(self, other: Millimeters) -> Self {
        Self(self.0 + other.0 / 1000.0)
    }
}
