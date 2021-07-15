/// Programming a [Guessing Game]
///
/// [guessing game]: https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html
use book::ch02::Guesser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut guess = Guesser::new("Guess your number");
    guess.prompt();
    guess.read()?;
    println!("Your guess is {}", guess);
    Ok(())
}
