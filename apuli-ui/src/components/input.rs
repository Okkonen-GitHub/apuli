#[derive(Debug)]
pub struct InputLoop {
    pub word_len: usize,
    pub current: Vec<char>,
}

impl InputLoop {
    pub fn new(word_len: usize, current: Vec<char>) -> Self {
        Self { word_len, current }
    }
    pub fn reset(&mut self) {
        self.current.clear();
    }

    pub fn insert_char(&mut self, key: char) -> &mut InputLoop {
        self.current.push(key);
        if self.current.len() > self.word_len {
            self.current.remove(0); // remove the first character
        }
        self
    }
    // removes the last character from the InputLoop (backspace)
    pub fn remove_char(&mut self) -> &mut InputLoop {
        if self.current.len() != 0 {
            self.current.pop();
        }
        self
    }
}
