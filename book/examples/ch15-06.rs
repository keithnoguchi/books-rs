//! [RefCell<T>] and the Interior Mutability Pattern, part 2
//!
//! [refcell<t>]: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
use std::{cell::RefCell, rc::Rc};

use the_book::ch15::sec06::List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(10));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Rc::new(Cons(Rc::new(RefCell::new(11)), Rc::clone(&a)));
    let c = Rc::new(Cons(Rc::new(RefCell::new(12)), Rc::clone(&a)));
    println!("a={:?}", a);
    println!("b={:?}", b);
    println!("c={:?}", c);
    *value.borrow_mut() += 5;
    println!("a={:?}", a);
    println!("b={:?}", b);
    println!("c={:?}", c);
}
