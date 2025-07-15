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
    pub mode: GameMode,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameMode {
    Sanuli,
    Neluli,
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
    pub fn new(word_len: usize, current_theme: Theme, game_mode: GameMode) -> Self {
        Self {
            word_length: word_len,
            guesses: std::iter::repeat(Vec::with_capacity(word_len))
                .take(6)
                .collect::<Vec<_>>(),
            current_guess: 0,
            tile_manager: TileManager::new(),
            theme: current_theme,
            mode: game_mode
        }
    }
    pub fn update_guesses(&mut self, input_handler: &InputLoop) -> &Self {
        let index = self.current_guess;
        self.guesses.remove(index);
        self.guesses.insert(index, input_handler.current.clone());
        self
    }
    pub fn max_guesses(&self) -> usize {
        match self.mode {
            GameMode::Sanuli => 6,
            GameMode::Neluli => 9,
        }
    }
}
