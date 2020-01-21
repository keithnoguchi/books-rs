//! Creating a [Reference Cycle]
//!
//! [reference cycle]: https://doc.rust-lang.org/book/ch15-06-reference-cycles.html
use std::cell::RefCell;
use std::rc::Rc;

use the_book::ch15::sec04::CycleList;

fn main() {
    let a = Rc::new(CycleList::Node('a', RefCell::new(Rc::new(CycleList::Null))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.next());
    let b = Rc::new(CycleList::Node('b', RefCell::new(a.clone())));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("a rc count = {}", Rc::strong_count(&a));
    println!("b next item = {:?}", b.next());
    println!("a next item = {:?}", a.next());
    if let Some(next) = a.next() {
        *next.borrow_mut() = b.clone();
    }
    println!("b rc count = {}", Rc::strong_count(&b));
    println!("a rc count = {}", Rc::strong_count(&a));
    // println!("a next items = {:?}", a.next());
}
