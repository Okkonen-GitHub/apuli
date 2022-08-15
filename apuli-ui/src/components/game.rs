use yew::Properties;

use super::{input::InputLoop, manager::TileManager};

#[derive(Clone, PartialEq, Properties)]
pub struct Game {
    pub word_length: usize,
    pub guesses: Vec<Vec<char>>,
    pub current_guess: usize,
    pub tile_manager: TileManager,
    pub is_ready: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            word_length: 5,
            guesses: vec![[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec()],
            current_guess: 0,
            tile_manager: TileManager::new(),
            is_ready: false,
        }
    }
    pub fn update_guesses(&mut self, input_handler: &InputLoop) -> &Game {
        let index = self.current_guess;
        self.guesses.remove(index);
        self.guesses.insert(index, input_handler.current.clone());
        self

    }
}

