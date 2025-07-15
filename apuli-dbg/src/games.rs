#[derive(Clone, Debug, Default)]
pub(crate) struct Game {
    pub guesses: Vec<String>,
    pub target: String,
}

impl Game {
    fn is_success(&self) -> bool {
        *self.guesses.last().expect("No words were played") == self.target
    }
}
