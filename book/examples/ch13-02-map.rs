//! Processing a Series of Items with [Iterators]
//!
//! [iterators]: https://doc.rust-lang.org/book/ch13-02-iterators.html
fn main() {
    let v = vec![1, 2, 3];
    let i = v.iter().map(|x| (*x as f32) / 4.0);
    assert_eq!(vec![0.25, 0.5, 0.75], i.collect::<Vec<f32>>());
}
