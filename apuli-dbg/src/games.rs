pub(crate) struct Game {
    guesses: Vec<String>,
    target: String,
}

impl Game {
    fn is_success(&self) -> bool {
        *self.guesses.last().unwrap_or(&"".to_owned()) == self.target
    }
}
