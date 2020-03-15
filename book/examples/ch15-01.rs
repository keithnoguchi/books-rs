//! Cons list example
//!
//! [cons]: https://doc.rust-lang.org/book/ch15-01-box.html
use the_book::ch15::sec01::List::{Cons, Nil};

fn main() {
    print_i32_list();
    print_char_list();
    print_string_list();
}

fn print_i32_list() {
    let mut list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
    while let Cons(val, next) = list {
        println!("{}", val);
        list = *next;
    }
}

fn print_char_list() {
    let mut list = Cons('a', Box::new(Cons('b', Box::new(Cons('c', Box::new(Nil))))));
    println!("{:?}", list);
    while let Cons(value, next) = list {
        println!("{}", value);
        list = *next;
    }
}

fn print_string_list() {
    let mut list = Cons(
        String::from("a"),
        Box::new(Cons(
            String::from("b"),
            Box::new(Cons(String::from("c"), Box::new(Nil))),
        )),
    );
    println!("{:?}", list);
    while let Cons(value, next) = list {
        println!("{}", value);
        list = *next;
    }
}
