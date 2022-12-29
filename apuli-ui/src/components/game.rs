use yew::Properties;

use std::fmt;

use super::{input::InputLoop, manager::TileManager};

#[derive(Clone, PartialEq, Properties)]
pub struct Game {
    pub word_length: usize,
    pub guesses: Vec<Vec<char>>,
    pub current_guess: usize,
    pub tile_manager: TileManager,
    pub theme: Theme,
}


#[derive(Clone, PartialEq, Copy)]
pub enum Theme {
    Dark,
    Colorblind,
}

impl fmt::Display for Theme {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Theme::Dark => write!(f, "dark"),
            Theme::Colorblind => write!(f, "colorblind"),
        }
    }
}

impl Game {
    pub fn new(word_len: usize) -> Self {
        Self {
            word_length: word_len,
            guesses: std::iter::repeat(Vec::with_capacity(word_len))
                .take(6)
                .collect::<Vec<_>>(),
            current_guess: 0,
            tile_manager: TileManager::new(),
            theme: Theme::Dark,
        }
    }
    pub fn update_guesses(&mut self, input_handler: &InputLoop) -> &Game {
        let index = self.current_guess;
        self.guesses.remove(index);
        self.guesses.insert(index, input_handler.current.clone());
        self
    }
}
