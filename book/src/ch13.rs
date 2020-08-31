//! [Functional Language] Features: Iterators and Closures
//!
//! [functional language]: https://doc.rust-lang.org/book/ch13-00-functional-features.html

/// [Cacher] keeps the expensive calculation result stored internally when
/// the `value` method called for the first time, so that the subsequent
/// `value` call will be faster by skippnig the actual calculation.
///
/// [cacher]: struct.Cacher.html
pub struct Cacher<F: Fn(i32) -> i32> {
    calc: F,
    value: Option<i32>,
}

impl<F: Fn(i32) -> i32> Cacher<F> {
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
        Self { calc, value: None }
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
    pub fn value(&mut self, value: i32) -> i32 {
        match self.value {
            Some(x) => x,
            None => {
                let x = (self.calc)(value);
                self.value = Some(x);
                x
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cacher_new() {
        let mut _c = super::Cacher::new(|x| x);
    }
    #[test]
    fn cacher_value() {
        let mut c = super::Cacher::new(|x| 3 * x);
        let got = c.value(2);
        assert_eq!(6, got);
    }
}
