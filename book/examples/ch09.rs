//! [Error] Handling
//!
//! [error]: https://doc.rust-lang.org/book/ch09-00-error-handling.html
use std::{error, fs::File};

fn main() -> Result<(), Box<dyn error::Error>> {
    let f = File::open("test.txt")?;
    println!("{:#?} = open(\"test.txt\"", f);
    Ok(())
}
