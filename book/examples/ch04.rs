//! [Understand Ownership]
//!
//! [understand ownership]: https://doc.rust-lang.org/book/ch04-00-understand-ownership.html
fn main() {
    // &str: [string slice]
    //
    // [string slice]: https://doc.rust-lang.org/std/primitive.str.html
    let /* mut */ s = String::from("hello, world");
    let word = first_word(&s);
    //s.clear();
    assert_eq!("hello,", word);
    assert_eq!("hello,", first_word(&s[..]));
    assert_eq!("hello,", first_word("hello, world"));

    // &[i32] [slice type]
    //
    // [slice type]: https://doc.rust-lang.org/std/primitive.slice.html
    let a = [1, 2, 3, 4, 5];
    assert_eq!(&[2, 3], &a[1..3]);
}

fn first_word(s: &str) -> &str {
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
