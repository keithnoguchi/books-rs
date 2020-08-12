//! [Error] Handling
//!
//! [error]: https://doc.rust-lang.org/book/ch09-00-error-handling.html
use std::{error, io};

use the_book::ch09::Guess;

fn main() -> Result<(), Box<dyn error::Error>> {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let number: i32 = line.trim().parse()?;
        let guess = Guess::new(number);
        println!("Your guess is {:#?}", *guess);
    }
}
