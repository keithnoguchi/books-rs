//! Validating References with [Lifetimes]
//!
//! [lifetime]: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
//!
//! # Examples
//!
//! Generic lifetimes in functions.
//!
//! ```
//! use the_book::ch10::longest;
//!
//! let a = "this is a sentence.";
//! let b = "This is also a sentence.";
//!
//! assert_eq!("This is also a sentence.", longest(&a, &b));
//! ```
//!
//! Different lifetime input case
//!
//! ```
//! use the_book::ch10::longest;
//!
//! let s1 = String::from("long string is long");
//! {
//!     let s2 = String::from("xyz");
//!     assert_eq!(
//!         &String::from("long string is long"),
//!         longest(&s1, &s2),
//!     );
//! }
//! ```

/// It returns the longest strings.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
