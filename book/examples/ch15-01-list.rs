//! Smart Pointers example
use the_book::ch15::x01_list::{List, MyBox};

fn main() {
    let mut i32s = List::Nil;
    i32s = List::Cons(9i32, Box::new(i32s));
    i32s = List::Cons(8, Box::new(i32s));
    i32s = List::Cons(7, Box::new(i32s));
    let mut chars = List::Nil;
    chars = List::Cons('c', Box::new(chars));
    chars = List::Cons('b', Box::new(chars));
    chars = List::Cons('a', Box::new(chars));
    let mut f32s = List::Nil;
    f32s = List::Cons(9.2, Box::new(f32s));
    f32s = List::Cons(12.2, Box::new(f32s));
    f32s = List::Cons(11.2f32, Box::new(f32s));
    print_list("List<i32>", &i32s);
    print_list("List<char>", &chars);
    print_list("List<f32>", &f32s);
    // You can have multiple immutable borrow.
    let name = String::from("rust");
    let boxed_name = Box::new(&name); // just immutable borrowing the &str.
    let myboxed_name = MyBox::new(&name); // again, it's just immutable borrowing.
                                          // And the implicit deref coercision.
    hello(&name);
    hello(&boxed_name);
    hello(&myboxed_name);
    hello(&(*myboxed_name)[..]); // without the deref coercision.
    let name = String::from("rust");
    let mut myboxed_name = MyBox::new(name);
    (*myboxed_name).push_str("acian");
    hello(&myboxed_name);
    println!("main finished");
}

fn print_list<T>(title: &str, mut list: &List<T>)
where
    T: std::fmt::Display,
{
    println!("{}", title);
    while let List::Cons(val, next) = list {
        println!("{}", val);
        list = next;
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}
