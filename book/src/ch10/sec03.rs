//! Validating References with [Lifetimes]
//!
//! [lifetimes]: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

/// `longest` as an lifetime function example.
///
/// # Examples
///
/// ```
/// use the_book::ch10::longest;
///
/// let a = String::from("abcd");
/// let b = "xyz";
///
/// let longest = longest(&a, b);
/// assert_eq!("abcd", longest);
/// ```
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
