//! [Rc<T>], the Reference Counted Smart Pointer
//!
//! [rc<t>]: https://doc.rust-lang.org/book/ch15-04-rc.html
use std::{fmt::Debug, rc::Rc};

#[derive(Debug)]
pub enum List<T: Debug> {
    Cons(T, Rc<List<T>>),
    Nil,
}

impl<T: Debug> Drop for List<T> {
    fn drop(&mut self) {
        println!("dropping {:?}", self);
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::List::{Cons, Nil};
    #[test]
    fn single_reference() {
        let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
        assert_eq!(1, Rc::strong_count(&a));
    }
    #[test]
    fn double_references() {
        let a = Rc::new(Cons(1, Rc::new(Nil)));
        assert_eq!(1, Rc::strong_count(&a));
        {
            let b = Rc::new(Cons(2, Rc::clone(&a)));
            assert_eq!(1, Rc::strong_count(&b));
            assert_eq!(2, Rc::strong_count(&a));
        }
        assert_eq!(1, Rc::strong_count(&a));
    }
    #[test]
    fn tripple_references() {
        let a = Rc::new(Cons(1, Rc::new(Nil)));
        assert_eq!(1, Rc::strong_count(&a));
        {
            let b = Rc::new(Cons(2, Rc::clone(&a)));
            assert_eq!(1, Rc::strong_count(&b));
            assert_eq!(2, Rc::strong_count(&a));
            let b = Rc::new(Cons(3, Rc::clone(&a)));
            assert_eq!(1, Rc::strong_count(&b));
            assert_eq!(3, Rc::strong_count(&a));
        }
        assert_eq!(1, Rc::strong_count(&a));
    }
}
