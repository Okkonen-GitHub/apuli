#[derive(Clone, Debug)]
pub(crate) struct Game {
    guesses: Vec<String>,
    target: String,
}

impl Game {
    fn is_success(&self) -> bool {
        *self.guesses.last().expect("No words were played") == self.target
    }
}
