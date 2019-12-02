// SPDX-License-Identifier: GPL-2.0
// https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html
use book::{ch12::Config, Error};
use std::{env, fs};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args)?;
    let contents = fs::read_to_string(cfg.filename())?;
    println!("With text:\n{}", contents);
    Ok(())
}
