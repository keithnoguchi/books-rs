/// Programming a [Guessing Game]
///
/// [guessing game]: https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html

/// Guesser guess the number multiple times.
pub struct Guesser {
    prompt: String,
    guess: String,
}

impl std::fmt::Display for Guesser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.guess)
    }
}

impl Guesser {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.into(),
            guess: String::new(),
        }
    }
    pub fn prompt(&self) {
        eprint!("{}: ", self.prompt);
    }
    pub fn read(&mut self) -> Result<usize, Box<dyn std::error::Error>> {
        Ok(std::io::stdin()
            .read_line(&mut self.guess)?)
    }
}
