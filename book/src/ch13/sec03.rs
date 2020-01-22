//! Improving Our I/O Project with [Iterator]
//!
//! [iterator]: https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html
use std::env;
use std::error::Error;
use std::fmt;

/// `Config` type which owns the command line iterator to avoid
/// unnessesary `clone()` call.
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, Box<dyn Error>> {
        let progname = args.next().unwrap();
        let usage = move || format!("usage: {} <query> <filename>", progname);
        let query = match args.next() {
            None => return Err(usage().into()),
            Some(arg) => arg,
        };
        let filename = match args.next() {
            None => return Err(usage().into()),
            Some(arg) => arg,
        };
        Ok(Self { query, filename })
    }
    pub fn query(&self) -> &str {
        &self.query
    }
    pub fn filename(&self) -> &str {
        &self.filename
    }
}

impl fmt::Display for Config {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "query:\t\t{}\nfilename:\t{}",
            self.query, self.filename
        )
    }
}

/// looking for the `query` string out of `contents` and returns
/// vector of line which contains the `query` string.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
