//! [Common Programming Concept]
//!
//! [common programming concept]: https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html

fn main() {
    count_down(3);
    let a = [10, 20, 30, 40, 50];
    count_up(&a);
}

fn count_down(number: usize) {
    println!("rev() based loop");
    for i in (0..number).rev() {
        println!("{}", i + 1);
    }
    println!("LIFTOFF!!");
}

fn count_up(a: &[i32]) {
    println!("iter() based loop");
    for i in a.iter() {
        println!("{}", i);
    }
}
