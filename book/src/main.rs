// SPDX-License-Identifier: GPL-2.0
// https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html
use book::{ch12::Config, Error};
use std::{env, fs, process};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args).unwrap_or_else(|err| usage(err));
    let contents = fs::read_to_string(cfg.filename())?;
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
