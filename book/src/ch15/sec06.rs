//! [RefCell<T>] and the Interior Mutability Pattern, part 2
//!
//! [refcell<t>]: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug)]
pub enum List<T: Debug> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}

impl<T: Debug> List<T> {
    pub fn new(x: T) -> Self {
        List::Cons(Rc::new(RefCell::new(x)), Rc::new(List::Nil))
    }
}

#[cfg(test)]
mod tests {
    use super::List::{self, Cons, Nil};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn new() {
        match List::new(5) {
            Cons(node, _) => assert_eq!(5, *node.borrow()),
            Nil => panic!("unexpected nil"),
        }
    }
    #[test]
    fn update_value() {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let _b = Rc::new(Cons(Rc::new(RefCell::new(99)), Rc::clone(&a)));
        let _c = Rc::new(Cons(Rc::new(RefCell::new(100)), Rc::clone(&a)));
        *value.borrow_mut() += 10;
        match &*a {
            Cons(node, _) => assert_eq!(15, *node.borrow()),
            Nil => panic!("unexpected nil"),
        }
    }
}
