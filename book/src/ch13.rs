//! [Functional Language] Features: Iterators and Closures
//!
//! [functional language]: https://doc.rust-lang.org/book/ch13-00-functional-features.html

/// `Cacher` caches the value retrieved through the func [`FnOnce`] closure.
///
/// [`FnOnce`]: https://doc.rust-lang.org/std/ops/trait.FnOnce.html
pub struct Cacher<T: Clone, F: FnOnce(T) -> T> {
    pub calc: F,
    pub value: Option<T>,
}
