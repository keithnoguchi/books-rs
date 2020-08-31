//! [Closures]: Anonymous Functions that Can Capture Their Environment
//!
//! [closures]: https://doc.rust-lang.org/book/ch13-01-closures.html
use std::error::Error;
use the_book::ch13::Cacher;

fn main() -> Result<(), Box<dyn Error>> {
    let mut c = Cacher::new(|x| x);

    for _ in 1..1_000 {
        let got = c.value(1);
        assert_eq!(1, got);
    }
    let got = c.value(999);
    assert_eq!(1, got);
    Ok(())
}
