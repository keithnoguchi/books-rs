//! [Closures]: Anonymous Functions that Can Capture Their Environment
//!
//! [closures]: https://doc.rust-lang.org/book/ch13-01-closures.html
use the_book::ch13::Cacher;

fn main() {
    let mut c = Cacher::new(|x: String| x.to_lowercase());

    for x in 1..1_000 {
        let got = c.value(x.to_string());
        assert_eq!(x.to_string(), got);
    }
    let got = c.value(999.to_string());
    assert_eq!(999.to_string(), got);
}
