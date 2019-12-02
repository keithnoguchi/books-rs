// SPDX-License-Identifier: GPL-2.0
// https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
