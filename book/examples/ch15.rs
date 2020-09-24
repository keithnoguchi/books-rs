//! [Smart Pointers]
//!
//! [smart pointers]: https://docs.rust-lang.org/book/ch15-00-smart-pointers.html
use the_book::ch15::List::{Cons, Nil};

fn main() {
    let mut cons = &Cons(2, Box::new(Cons(9, Box::new(Nil))));
    'outer: loop {
        cons = match cons {
            Nil => break 'outer,
            Cons(val, ref next) => {
                println!("{}", val);
                next
            }
        };
    }
}
