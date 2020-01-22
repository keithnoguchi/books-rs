//! Processing a Series of Items with [Iterators]
//!
//! [iterators]: https://doc.rust-lang.org/book/ch13-02-iterators.html
use the_book::ch13::sec02::Counter;

fn main() {
    let limit = 6;
    let want = 18;
    let got: u32 = Counter::new(limit)
        .zip(Counter::new(limit).skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(want, got);
}
