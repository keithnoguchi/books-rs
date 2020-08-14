//! [Closures]: Anonymous Functions that Can Capture Their Environment
//!
//! [closures]: https://doc.rust-lang.org/book/ch13-01-closures.html

/// `Cacher` caches the value got through `calc` [`Fn`] closure.
///
/// [`Fn`]: https://doc.rust-lang.org/std/ops/trait.Fn.html
pub struct Cacher<T: Copy + Clone, F: Fn(T) -> T> {
    pub calc: F,
    pub value: Option<T>,
}

impl<T: Copy + Clone, F: Fn(T) -> T> Cacher<T, F> {
    pub fn new(f: F) -> Self {
        Self {
            calc: f,
            value: None,
        }
    }
    pub fn get(&mut self, input: T) -> T {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calc)(input);
                self.value = Some(v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cacher;
    #[test]
    fn value() {
        let mut c = Cacher::new(|v| v);
        for _ in 0..1000 {
            let got = c.get(1);
            assert_eq!(1, got);
        }
        // It ignores the new value.
        let got = c.get(2);
        assert_eq!(1, got);
    }
}
