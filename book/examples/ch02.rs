use rand::Rng;
use std::{cmp::Ordering, error, io};

const MIN: u32 = 1;
const MAX: u32 = 100;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Guess the number between {} and {}", MIN, MAX);
    let secret = rand::thread_rng().gen_range(MIN, MAX + 1);
    let mut guess = String::new();
    while let Ok(n) = io::stdin().read_line(&mut guess) {
        if n == 0 {
            break;
        }
        match guess.trim().parse::<u32>() {
            Err(_) => println!("type number, try again"),
            Ok(n) => match n.cmp(&secret) {
                Ordering::Less => println!("Higher"),
                Ordering::Greater => println!("Lower"),
                Ordering::Equal => {
                    println!("Bingo");
                    break;
                }
            },
        }
        guess.clear();
    }
    Ok(())
}
