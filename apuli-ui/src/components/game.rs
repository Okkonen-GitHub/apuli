use yew::Properties;

#[derive(Clone, PartialEq, Properties)]
pub struct Game {
    pub word_length: usize,
    pub guesses: Vec<Vec<char>>

}

impl Game {
    pub fn new() -> Self {
        Self {
            word_length: 5,
            guesses: vec![[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec(),[' '; 5].to_vec()]
        }
    }
}

