//! Using [Box<T>] to Point to Data on the Heap
//!
//! [box<t>]: https://doc.rust-lang.org/book/ch15-01-box.html

pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

/// Treating Smart Pointer Like Regular References with [the Deref trait]
///
/// [the deref trait]: https://doc.rust-lang.org/book/ch15-02-deref.html
pub struct MyBox<T>(T)
where
    T: std::fmt::Debug;

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
    fn list() {
        use super::List::{Cons, Nil};
        let wants = [11, 12, 13, 14];
        let mut list = Nil;
        list = Cons(14, Box::new(list));
        list = Cons(13, Box::new(list));
        list = Cons(12, Box::new(list));
        list = Cons(11, Box::new(list));
        let mut got = &list;
        for want in &wants {
            got = match got {
                Nil => panic!("unexpected premature Nil node"),
                Cons(got, next) => {
                    assert_eq!(want, got);
                    next
                }
            };
        }
        if let Cons(..) = got {
            panic!("unexpected non Nil tail");
        }
    }
    #[test]
    fn reference_dereference() {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    #[test]
    fn box_dereference() {
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
        assert_eq!(5, *y);
    }
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
