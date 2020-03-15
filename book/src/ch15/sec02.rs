//! Treating Smart Pointer Like Regular References with [the Deref trait]
//!
//! [the deref trait]: https://doc.rust-lang.org/book/ch15-02-deref.html
pub struct MyBox<T: std::fmt::Debug>(T);

impl<T> MyBox<T>
where
    T: std::fmt::Debug,
{
    pub fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T> Drop for MyBox<T>
where
    T: std::fmt::Debug,
{
    fn drop(&mut self) {
        println!("dropping {:?}", self.0);
    }
}

impl<T> std::ops::Deref for MyBox<T>
where
    T: std::fmt::Debug,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for MyBox<T>
where
    T: std::fmt::Debug,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mybox_new() {
        use super::MyBox;
        let x = MyBox::new(3);
        assert_eq!(3, x.0);
    }
    #[test]
    fn mybox_dereference() {
        use super::MyBox;
        use std::ops::Deref;
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y); // multiple deref doesn't cause an issue, as it's just borrows
        assert_eq!(5, *y); // the value the smart pointer is pointing to.
        assert_eq!(5, *(y.deref())); // This is the actual call by the compiler.
    }
    #[test]
    fn mybox_mutable_dereference() {
        use super::MyBox;
        let x = 5;
        let mut y = MyBox(x);
        *y = 6; // Through DerefMut trait.
        assert_eq!(5, x);
        assert_eq!(6, *y);
    }
}
