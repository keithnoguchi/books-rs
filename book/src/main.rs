// SPDX-License-Identifier: GPL-2.0
// https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html
use book::ch12::{self, Config};
use book::Error;
use std::{env, process};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args).unwrap_or_else(|err| usage(err));
    let contents = ch12::run(cfg).unwrap_or_else(|err| usage(err));
    println!("With text:\n{}", contents);
    Ok(())
}

fn usage(err: Error) -> ! {
    const PROGNAME: &str = "book";
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
