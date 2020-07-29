//! [Common Programming Concept]
//!
//! [common programming concept]: https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html
fn main() {
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("{}", i);
    }
    for i in (1..4).rev() {
        println!("{}", i);
    }
    println!("LIFTOFF!");
}
