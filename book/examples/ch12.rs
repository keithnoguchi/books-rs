//! An I/O Project: Building a Command Line Program
use std::{env, process};

use the_book::ch12::{self, Config};
use the_book::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args).unwrap_or_else(|err| usage(err));
    if let Err(err) = ch12::run(cfg) {
        usage(err);
    }
    Ok(())
}

fn usage(err: Error) -> ! {
    const PROGNAME: &str = "the-book";
    use std::io;
    match err {
        Error::Io(err) => match err.kind() {
            io::ErrorKind::InvalidInput => {
                eprintln!("{}: {}", PROGNAME, err.to_string());
            }
            io::ErrorKind::NotFound => {
                eprintln!("{}: {}", PROGNAME, err.to_string());
            }
            _ => eprintln!("{}: oops {:?}", PROGNAME, err),
        },
        _ => eprintln!("{}: unexpected error", PROGNAME),
    }
    process::exit(1);
}
