//! Cons list example
//!
//! [cons]: https://doc.rust-lang.org/book/ch15-01-box.html
use the_book::ch15::sec01::List::{Cons, Nil};

fn main() {
    print_i32_list();
}

fn print_i32_list() {
    let mut list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
    while let Cons(val, next) = list {
        println!("{}", val);
        list = *next;
    }
}
