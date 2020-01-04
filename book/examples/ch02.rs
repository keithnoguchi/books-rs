// SPDX-License-Identifier: GPL-2.0
use std::{error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Guess the number!");
    println!("Please input your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)?;
    println!("You guessed: {}", guess);
    Ok(())
}
