//! Treating Smart Pointer Like Regular References with [the Deref trait]
//!
//! [the deref trait]: https://doc.rust-lang.org/book/ch15-02-deref.html
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::MyBox;
    #[test]
    fn new_i32() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    #[test]
    fn new_string() {
        let x = String::from("five");
        assert_eq!(String::from("five"), x);
        let y = MyBox::new(x);
        assert_eq!(String::from("five"), *y);
    }
    #[test]
    fn deref_mut_i32() {
        let x = 5;
        let mut y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
        *y = 9;
        assert_eq!(5, x);
        assert_eq!(9, *y);
    }
    #[test]
    fn deref_mut_string() {
        let x = String::from("five");
        assert_eq!(String::from("five"), x);
        let mut y = Box::new(x);
        assert_eq!(String::from("five"), *y);
        *y = String::from("six");
        assert_eq!(String::from("six"), *y);
    }
}
