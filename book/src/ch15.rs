//! [Smart Pointers]
//!
//! [smart pointers]: https://docs.rust-lang.org/book/ch15-00-smart-pointers.html

/// Smart pointer example with `Box`.
pub enum List<T> {
    Cons(T, Box<Self>),
    Nil,
}
