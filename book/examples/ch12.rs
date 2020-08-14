//! An I/O Project: Building a Command Line Program
use std::{env, process};

use the_book::ch12;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = ch12::App::new(&args)
        .unwrap_or_else(|err| {
            eprintln!("invalid configuration: {}", err);
            process::exit(1);
        })
        .run()
        .unwrap_or_else(|err| {
            eprintln!("application error: {}", err);
            process::exit(1);
        });
    for line in &lines {
        println!("{}", line);
    }
}
