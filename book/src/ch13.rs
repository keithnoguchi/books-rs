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
    /// let mut c = Cacher::<&str, _, _>::new(|x| x.to_lowercase());
    /// c.value("SOMETHING");
    /// assert_eq!(String::from("hello"), c.value("HELLO"));
    /// assert_eq!(String::from("something"), c.value("Something"));
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
    /// use the_book::ch13::Cacher;
    ///
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

/// `Students` demonstrates the [Iterator] implementation.
///
/// [iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
#[derive(Default)]
pub struct Students<T: Default + PartialCmp> {
    students: Vec<T>,
}

impl<T: Default + PartialCmp> Students<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&mut self, name: T) -> &mut Self {
        self.students.push(name);
        self
    }
    pub fn total(&self) -> usize {
        self.students.len()
    }
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.students.iter()
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
    #[test]
    fn students_register() {
        let mut students = super::Students::new();
        students
            .register(String::from("Adam"))
            .register(String::from("David"));
    }
    #[test]
    fn students_total() {
        let mut students = super::Students::new();
        let students = students.register("Adam").register("Bob");
        assert_eq!(2, students.total());
    }
    #[test]
    fn students_iter() {
        let mut students = super::Students::new();
        let students = students.register("Adam").register("Bob");
        let mut iter = students.iter();
        assert_eq!(Some(&"Adam"), iter.next());
        assert_eq!(Some(&"Bob"), iter.next());
        assert_eq!(None, iter.next());
    }
}
