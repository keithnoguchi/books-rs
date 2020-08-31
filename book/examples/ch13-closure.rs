//! Functional Programming Features: Iterators and [Closures]
//!
//! [closures]: https://doc.rust-lang.org/book/ch13-01-closures.html
fn main() {
    let x = vec![1, 2, 3];
    let equal_to_x = |z: &Vec<i32>| *z == x;
    println!("x = {:?}", x);
    let y = vec![1, 2, 3];
    for _ in 1..1_000 {
        assert!(equal_to_x(&y));
    }
}
