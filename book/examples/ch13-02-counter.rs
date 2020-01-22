//! Processing a Series of Items with [Iterators]
//!
//! [iterators]: https://doc.rust-lang.org/book/ch13-02-iterators.html
use the_book::ch13::sec02::Counter;

fn main() {
    let limit = cmdline();
    let counter = Counter::new(limit + 1);

    let mut got = 0;
    for i in counter {
        got += 1;
        println!("{}", i);
    }
    let want = limit;
    assert_eq!(want, got);
}

fn cmdline() -> u32 {
    let argv: Vec<String> = std::env::args().collect();
    match argv.len() {
        0..=1 => 6,
        _ => argv[1].parse().expect("limit should be {integer}"),
    }
}
