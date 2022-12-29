use yew::Properties;

use super::{input::InputLoop, manager::TileManager};

#[derive(Clone, PartialEq, Properties)]
pub struct Game {
    pub word_length: usize,
    pub guesses: Vec<Vec<char>>,
    pub current_guess: usize,
    pub tile_manager: TileManager,
}

impl Game {
    pub fn new() -> Self {
        Self {
            word_length: 5,
            guesses: std::iter::repeat(Vec::with_capacity(5))
                        .take(6)
                        .collect::<Vec<_>>(),
            current_guess: 0,
            tile_manager: TileManager::new(),
        }
    }
    pub fn update_guesses(&mut self, input_handler: &InputLoop) -> &Game {
        let index = self.current_guess;
        self.guesses.remove(index);
        self.guesses.insert(index, input_handler.current.clone());
        self
    }
}
