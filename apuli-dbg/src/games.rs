use apuli_lib::{
    apuli::{query, rank},
    information::remaining_information,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct Game {
    pub guesses: Vec<String>,
    pub target: String,
}

impl Game {
    fn is_success(&self) -> bool {
        *self.guesses.last().expect("No words were played") == self.target
    }
    pub fn remaining_words(&self, word_lenght: usize) -> Vec<(u16, String)> {
        let result = query(&[], None, None, word_lenght);
        rank(result)
    }
    pub fn remaining_information(&self, word_lenght: usize) -> f64 {
        let result = query(&[], None, None, word_lenght);
        remaining_information(&result)
    }
}
