//! Writing Automated [Tests]
//!
//! [tests]: https://doc.rust-lang.org/book/ch11-00-testing.html
fn main() {
    std::env::args().for_each(|arg| println!("{:#?}", arg));
}
