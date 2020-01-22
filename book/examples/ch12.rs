//! An I/O Project: Building a Command Line Program
use std::{env, io, process};

use the_book::ch09::Error;
use the_book::ch12::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args).unwrap_or_else(|err| usage(err));
    if let Err(err) = ch12::run(cfg) {
        usage(err);
    }
}

fn usage(err: Error) -> ! {
    match err {
        Error::Io(err) => match err.kind() {
            io::ErrorKind::InvalidInput => eprintln!("{:?}", err),
            io::ErrorKind::NotFound => eprintln!("{:?}", err),
            _ => eprintln!("oops {:?}", err),
        },
        _ => eprintln!("unexpected error"),
    }
    process::exit(1);
}
