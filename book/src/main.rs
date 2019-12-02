// SPDX-License-Identifier: GPL-2.0
// https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html
use book::Error;
use std::{env, io::ErrorKind};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Error::from(ErrorKind::InvalidInput));
    }
    let filename = &args[1];
    println!("{}", filename);
    Ok(())
}
