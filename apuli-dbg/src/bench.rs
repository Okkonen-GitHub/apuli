use std::collections::VecDeque;

use crate::games::Game;
use apuli_lib::apuli::all_words;

// for everything and single mode
pub(crate) fn init_all_games(word_lenght: usize) -> VecDeque<Game> {
    all_words(word_lenght)
        .iter()
        .map(|goal| Game {
            target: goal.to_owned(),
            guesses: vec![],
        })
        .collect()
}

// for cherrypicking
pub(crate) fn init_selected(selected: &[String]) -> VecDeque<Game> {
    selected
        .iter()
        .map(|word| Game {
            target: word.to_string(),
            guesses: vec![],
        })
        .collect()
}
