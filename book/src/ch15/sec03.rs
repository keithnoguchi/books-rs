//! Running Code on Cleanup with the [Drop Trait]
//!
//! [drop trait]: https://doc.rust-lang.org/book/ch15-03-drop.html
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub struct SmartPointer<T: Debug + Clone>(T);

impl<T: Debug + Clone> SmartPointer<T> {
    pub fn new(x: &T) -> Self {
        Self(x.clone())
    }
}

impl<T: Debug + Clone> Drop for SmartPointer<T> {
    fn drop(&mut self) {
        println!("dropping {:?}", self.0);
    }
}

impl<T: Debug + Clone> Deref for SmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Debug + Clone> DerefMut for SmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SmartPointer;
    #[test]
    fn new_i32() {
        let x = 1_i32;
        let y = SmartPointer::new(&x);
        assert_eq!(1, x);
        assert_eq!(1, *y);
    }
    #[test]
    fn new_string() {
        let x = String::from("Hi");
        let y = SmartPointer::new(&x);
        let want = String::from("Hi");
        assert_eq!(want, x);
        assert_eq!(want, *y);
    }
    #[test]
    fn deref_mut_i32() {
        let x = 3_i32;
        let mut y = SmartPointer::new(&x);
        *y = 9;
        assert_eq!(3, x);
        assert_eq!(9, *y);
    }
    #[test]
    fn deref_mut_string() {
        let x = String::from("Rust");
        let mut y = SmartPointer::new(&x);
        *y = String::from("Rustacian");
        assert_eq!(String::from("Rust"), x);
        assert_eq!(String::from("Rustacian"), *y);
    }
}
