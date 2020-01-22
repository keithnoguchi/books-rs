//! Improving Our I/O Project with [Iterator]
//!
//! [iterator]: https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html
use std::env;
use std::fs;
use std::process;

use the_book::ch13::sec03::{search, Config};

fn main() {
    // parse the command line.
    let c = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    // read the contents from the file.
    let contents = fs::read_to_string(c.filename()).unwrap_or_else(|err| {
        eprintln!("{}: {}", c.filename(), err);
        process::exit(1);
    });
    // search for the query string.
    for line in search(c.query(), &contents) {
        println!("{}", line);
    }
}
