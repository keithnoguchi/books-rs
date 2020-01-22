//! Processing a Series of Items with [Iterators]
//!
//! [iterators]: https://doc.rust-lang.org/book/ch13-02-iterators.html
fn main() {
    // The `map()` iterator adaptor example which adapts i32 value to f32.
    let v = vec![4i32, 6, 8];
    let v2: Vec<f32> = v.iter().map(|x| (*x as f32) / 4.0).collect();
    assert_eq!(vec![1.0f32, 1.5, 2.0], v2);
}
