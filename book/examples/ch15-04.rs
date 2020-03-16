//! [Rc<T>], the Reference Counted Smart Pointer
//!
//! [rc<t>]: https://doc.rust-lang.org/book/ch15-04-rc.html
use std::rc::Rc;

use the_book::ch15::sec04::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a's strong count={} in the beginning", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!(
        "a's strong count={} after referenced by b",
        Rc::strong_count(&a)
    );
    {
        let _c = Cons(4, Rc::clone(&a));
        println!(
            "a's strong count={} after referenced by c",
            Rc::strong_count(&a)
        );
    }
    println!(
        "a's strong count={} after getting out of c's scope",
        Rc::strong_count(&a)
    );
    println!("done");
}
