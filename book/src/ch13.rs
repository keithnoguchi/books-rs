//! [Functional Language] Features: Iterators and Closures
//!
//! [functional language]: https://doc.rust-lang.org/book/ch13-00-functional-features.html
use std::{collections::HashMap, hash::Hash};

/// [Cacher] keeps the expensive calculation result stored internally when
/// the `value` method called for the first time, so that the subsequent
/// `value` call will be faster by skippnig the actual calculation.
///
/// [cacher]: struct.Cacher.html
pub struct Cacher<K: Eq + Hash + Clone, V: Clone, F: Fn(K) -> V> {
    calc: F,
    values: HashMap<K, V>,
}

impl<K: Eq + Hash + Clone, V: Clone, F: Fn(K) -> V> Cacher<K, V, F> {
    /// `new` instanciates the new [Cacher] instance.
    ///
    /// # Example
    ///
    /// ```
    /// use the_book::ch13::Cacher;
    ///
    /// let c = Cacher::new(|x| 2 * x);
    /// ```
    ///
    /// [cacher]: struct.Cacher.html
    pub fn new(calc: F) -> Self {
        Self {
            calc,
            values: HashMap::<K, V>::new(),
        }
    }

    /// `value` returns the calculation result if it's already calculated,
    /// otherwise, it calculates the value and stores the reuslt for the
    /// future reference.
    ///
    /// # Example
    ///
    /// ```
    /// let mut c = Cacher::new(|x| 3 * x);
    /// let got = c.value(9);
    /// assert_eq!(27, got);
    /// ```
    pub fn value(&mut self, key: K) -> V {
        let calc = &self.calc;
        self.values
            .entry(key.clone())
            .or_insert_with(|| (calc)(key))
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Cacher;

    #[test]
    fn cacher_new() {
        let mut _c = Cacher::new(|x: i32| x);
    }
    #[test]
    fn cacher_value() {
        let mut c = Cacher::new(|x: i32| 3 * x);
        let got = c.value(2);
        assert_eq!(6, got);
    }
    #[test]
    fn cacher_value_string() {
        let mut c = Cacher::new(|x: String| x.to_lowercase());
        let got1 = c.value("HELLO".into());
        let got2 = c.value("something else".into());
        assert_eq!("hello".to_string(), got1);
        assert_eq!("something else".to_string(), got2);
    }
    #[test]
    fn cacher_value_str_to_len() {
        let mut c = Cacher::new(|x: &str| x.len());
        let got1 = c.value("Hello");
        let got2 = c.value("Hello world");
        assert_eq!(5, got1);
        assert_eq!(11, got2);
    }
}
