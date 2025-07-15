mod bench;
pub mod information;
pub mod util;
//
// #[cfg(test)]
// pub mod bench;
//
// #[cfg(feature = "information")]
// pub mod information;
//
// pub mod util;

pub mod apuli {
    use std::collections::HashMap;

    const WORDS_6: &str = include_str!("../6_letter_words.txt");
    const WORDS_5: &str = include_str!("../5_letter_words.txt");

    pub const ALLOWED_KEYS: [char; 28] = [
        'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K',
        'L', 'Ö', 'Ä', 'Z', 'X', 'C', 'V', 'B', 'N', 'M',
    ];
    pub const ALLOWED_NUMS: [char; 6] = ['0', '1', '2', '3', '4', '5'];
    #[derive(Debug, Clone)]
    pub struct Letter {
        pub letter: char,
        pub positions: Option<Vec<usize>>,
    }

    trait ContainsN {
        // Returns true if string contains n amount of specified letter
        fn contains_n(&self, letter: &char, n: usize) -> bool;
        // Returns true if string contains AT LEAST n amount of specified letter
        fn contains_atleast_n(&self, letter: &char, n: usize) -> bool;

        // returns how many times a letter appears in a string
        fn appearances(&self, letter: &char) -> usize;
    }

    impl ContainsN for String {
        fn contains_n(&self, letter: &char, n: usize) -> bool {
            let mut count = 0;
            for ltr in self.chars() {
                if &ltr == letter {
                    count += 1;
                }
            }
            count == n
        }

        fn contains_atleast_n(&self, letter: &char, n: usize) -> bool {
            let mut count = 0;
            for ltr in self.chars() {
                if &ltr == letter {
                    count += 1;
                }
            }
            count >= n
        }

        fn appearances(&self, letter: &char) -> usize {
            let mut count = 0;
            for ltr in self.chars() {
                if &ltr == letter {
                    count += 1;
                }
            }
            count
        }
    }

    trait Removal {
        fn remove_grey(
            &mut self,
            grays: &[Letter],
            blues: Option<&[Letter]>,
            oranges: Option<&[Letter]>,
        ) -> Self;
        fn remove_others(&mut self, blues: Option<&[Letter]>, oranges: Option<&[Letter]>) -> Self;
        fn appearances(&self, guess: &str) -> usize;
    }

    impl Removal for Vec<String> {
        fn remove_grey(
            &mut self,
            grays: &[Letter],
            blues: Option<&[Letter]>,
            oranges: Option<&[Letter]>,
        ) -> Self {
            for gray in grays.iter() {
                let mut is_ominous = false;
                for word in self.clone().iter() {
                    if let Some(blues) = blues {
                        let mut known_count = 0;
                        for blue in blues {
                            if let Some(oranges) = oranges {
                                for orange in oranges {
                                    let mut is_exact = false;
                                    if orange.letter == gray.letter {
                                        is_exact = true;
                                        is_ominous = true;
                                        known_count += orange.positions.as_ref().unwrap().len();
                                    }
                                    // if we know the count based on oranges, it is already
                                    // correct, except when it isn't.
                                    // But to fix this we would need to add row info to the
                                    // positions for the algorithm.
                                    else if blue.letter == gray.letter {
                                        is_ominous = true;
                                        known_count += 1;
                                    }
                                    if known_count != 0
                                        && is_exact
                                        && !word.contains_atleast_n(&orange.letter, known_count)
                                    {
                                        if let Some(index) = self.iter().position(|x| x == word) {
                                            self.remove(index);
                                        }
                                    } else if known_count != 0
                                        && !word.contains_atleast_n(&blue.letter, known_count)
                                    {
                                        if let Some(index) = self.iter().position(|x| x == word) {
                                            self.remove(index); // the word might have already been
                                                                // removed earlier so we have to check in this (latter)
                                                                // case
                                        }
                                    }
                                    known_count = 0;
                                }
                            } else {
                                if blue.letter == gray.letter {
                                    is_ominous = true;
                                    known_count += 1;
                                }
                                if known_count != 0
                                    && !word.contains_atleast_n(&blue.letter, known_count)
                                {
                                    self.remove(self.iter().position(|x| x == word).unwrap());
                                }
                            }
                            known_count = 0;
                        }
                    } else if let Some(oranges) = oranges {
                        for orange in oranges {
                            if orange.letter == gray.letter {
                                is_ominous = true;
                                if !word.contains_n(
                                    &orange.letter,
                                    orange.positions.as_ref().unwrap().len(),
                                ) {
                                    if let Some(index) = self.iter().position(|x| x == word) {
                                        self.remove(index); // the word might have already been
                                                            // removed earlier so we have to check in this (latter)
                                                            // case
                                    }
                                }
                            }
                        }
                    }
                    if !is_ominous && word.contains(gray.letter) {
                        self.remove(self.iter().position(|x| x == word).unwrap());
                    }
                }
            }
            self.to_vec()
        }
        fn remove_others(&mut self, oranges: Option<&[Letter]>, blues: Option<&[Letter]>) -> Self {
            if let Some(oranges) = oranges {
                for word in self.clone().iter() {
                    if !check_oranges(oranges, word) {
                        self.remove(self.iter().position(|x| x == word).unwrap());
                    }
                }
            };
            if let Some(blues) = blues {
                for word in self.clone().iter() {
                    if !check_blues(blues, word) {
                        self.remove(self.iter().position(|x| x == word).unwrap());
                    }
                }
            }
            self.to_vec()
        }
        fn appearances(&self, guess: &str) -> usize {
            let mut count = 0;
            for word in self.iter() {
                if word == guess {
                    count += 1;
                }
            }
            count
        }
    }

    fn check_blues(blues: &[Letter], guess: &str) -> bool {
        /*
        *Returns true if some blue is found in the correct position
        Input: "SYÖPÄ", vec![Letter { letter: 'S', color: 1, positions: vec![3,4]}] --> true (sana kelpaa)
        Input: "SYÖPÄ", vec![Letter { letter: 'S', color: 1, positions: vec![0,2]] --> false (sana ei kelpaa)
        */
        for (pos, c) in guess.chars().enumerate() {
            for blue in blues.iter() {
                if blue.letter == c && blue.positions.as_ref().unwrap().contains(&pos) {
                    return false;
                }
            }
        }
        // at least n number of blues must be in the word though
        for blue in blues.iter() {
            if !guess.contains(blue.letter) {
                return false;
            }
        }
        true
    }

    fn check_oranges(oranges: &[Letter], guess: &str) -> bool {
        /*
        * Checks if the guess contains the correct oranges (in the right positions)
        let oranges = vec![
            Letter {
                letter: 'A',
                color: 2,
                positions: vec![0,1],
            },
        ];
        let guess = "AMMUU".to_string();
        println!("{}",check_oranges(&oranges, &guess));
        -> false
        let guess = "AALTO".to_string();
        println!("{}",check_oranges(&oranges, &guess));
        -> true
        */
        for orange in oranges.iter() {
            if !guess.contains(orange.letter) {
                return false;
            }
            for pos in orange.positions.as_ref().unwrap().iter() {
                if guess.chars().nth(*pos).unwrap() != orange.letter {
                    return false;
                }
            }
        }
        true
    }

    pub(crate) fn all_words(word_len: usize) -> Vec<String> {
        let mut words = Vec::new();
        match word_len {
            5 => {
                for word in WORDS_5.split('\n') {
                    if !word.is_empty() {
                        words.push(word.to_owned())
                    }
                }
            }
            6 => {
                for word in WORDS_6.split('\n') {
                    if !word.is_empty() {
                        words.push(word.to_owned())
                    }
                }
            }
            _ => {
                unreachable!()
            }
        }
        words
    }

    pub fn query(
        grays: &[Letter],
        blues: Option<Vec<Letter>>,
        oranges: Option<Vec<Letter>>,
        word_lenght: usize,
    ) -> Vec<String> {
        let mut words = all_words(word_lenght);

        words.remove_grey(grays, blues.as_deref(), oranges.as_deref());
        words.remove_others(oranges.as_deref(), blues.as_deref());

        words
    }

    // basically a sorting function
    // Returns a sorted list of strings and scores for each word
    pub fn rank(words: Vec<String>) -> Vec<(u16, String)> {
        let mut letter_frequency: HashMap<char, Vec<u16>> = HashMap::new();
        if words.is_empty() {
            return vec![];
        }
        let mut result = Vec::new();
        let word_len = words[0].len();
        for word in &words {
            for (position, letter) in word.chars().enumerate() {
                let key = &letter;
                if letter_frequency.contains_key(key) {
                    let mut positions: Vec<u16> = letter_frequency.get(key).unwrap().clone();
                    positions[position] += 1;

                    letter_frequency.insert(*key, positions.to_vec());
                } else {
                    let mut positions = vec![0; word_len];
                    positions[position] = 1;
                    letter_frequency.insert(*key, positions);
                }
            }
        }

        for word in words {
            let mut score = 0;
            for (ch, val) in &letter_frequency {
                // doesn't reward words having duplicate letters as much
                match word.appearances(ch) {
                    0 => {}
                    n => {
                        // this increases score for potential orange letters
                        for (index, ltr) in word.chars().enumerate() {
                            if ltr == *ch {
                                score += val[index] / (n as u16);
                            }
                        }
                        // then we need to increase score for blues
                        if word.contains(*ch) {
                            score += val.iter().sum::<u16>() / (n as u16);
                        }
                    }
                }
            }
            result.push((score, word));
        }

        // sort the vec based on score..
        // somehow
        result.sort_unstable_by(|a, b| b.cmp(a));

        result
    }
    // same as normal rank, but for neluli, so some additional information is used to rank the
    // words
    pub fn rank_combined(
        all_grays: &Vec<Vec<Letter>>,
        all_blues: Vec<Option<Vec<Letter>>>,
        all_oranges: &[Option<Vec<Letter>>],
        words: Vec<String>,
    ) -> Vec<(i32, String)> {
        let mut letter_frequency: HashMap<char, Vec<i32>> = HashMap::new();
        if words.is_empty() {
            return vec![];
        }
        let mut result = Vec::new();
        let word_len = words[0].len();
        for word in &words {
            for (position, letter) in word.chars().enumerate() {
                let key = &letter;
                if letter_frequency.contains_key(key) {
                    let mut positions: Vec<i32> = letter_frequency.get(key).unwrap().clone();
                    positions[position] += 1;

                    letter_frequency.insert(*key, positions.to_vec());
                } else {
                    let mut positions = vec![0; word_len];
                    positions[position] = 1;
                    letter_frequency.insert(*key, positions);
                }
            }
        }

        for word in words {
            let mut score = 0;
            for (ch, val) in &letter_frequency {
                // doesn't reward words having duplicate letters as much
                match word.appearances(ch) {
                    0 => {}
                    n => {
                        // this increases score for potential orange letters
                        for (index, ltr) in word.chars().enumerate() {
                            if ltr == *ch {
                                score += val[index] / (n as i32);
                            }
                        }
                        // then we need to increase score for blues
                        if word.contains(*ch) {
                            score += val.iter().sum::<i32>() / (n as i32);
                        }
                    }
                }
            }
            result.push((score, word));
        }

        // then reduce score for words that are not benefitial for all boards
        // remove duplicates but increase their score based on how many times the word appears
        // let mut index = 0;
        // let mut new = Vec::new();
        let mut i = 0;
        let mut words = result
            .iter()
            .map(|(_score, word)| word.clone())
            .collect::<Vec<String>>();
        while !words.is_empty() {
            // idk
            let word = &result.clone()[i].1;
            let score = result.clone()[i].0;
            let app_count = words.appearances(word);
            for _ in 0..app_count {
                let idx = result.iter().position(|(_, x)| x == word);
                if let Some(index) = idx {
                    result.remove(index);
                }
                let idx = words.iter().position(|x| x == word);
                if let Some(index) = idx {
                    words.remove(index);
                }
            }
            result.insert(0, (score * (app_count as i32), word.to_string()));
            i += 1;
        }
        // handle blues
        for (score, word) in &mut result {
            // goes through all the boards
            for blues in all_blues.iter().flatten() {
                // goes through all the blues in the board
                for blue in blues {
                    for (i, ltr) in word.chars().enumerate() {
                        if ltr == blue.letter && blue.positions.as_ref().unwrap().contains(&i) {
                            *score -= 1; // maybe change how much it should be reduced or even
                                         // divided
                        }
                    }
                }
            }
        }

        // then oranges
        for (score, word) in &mut result {
            for oranges in all_oranges.iter().flatten() {
                for orange in oranges {
                    for (i, ltr) in word.chars().enumerate() {
                        if ltr == orange.letter && orange.positions.as_ref().unwrap().contains(&i) {
                            *score -= 1;
                        }
                    }
                }
            }
        }
        // and finally grays
        for (score, word) in &mut result {
            for grays in all_grays {
                for gray in grays {
                    for ltr in word.chars() {
                        if ltr == gray.letter {
                            *score -= 1; // 1 might be too low for these
                        }
                    }
                }
            }
        }

        // sort the vec based on score..
        // somehow
        result.sort_unstable_by(|a, b| b.cmp(a));

        result
    }

    pub fn rank_scout(words: Vec<String>, word_len: usize) -> Vec<(u32, String)> {
        let all_words = all_words(word_len);
        let threshold = (words.len() as f64 * 0.7) as u32; // letter in ~70% of words get filtered
        let mut letters: Vec<(u32, char)> = Vec::new();
        let mut result = Vec::new();
        for word in &words {
            for ltr in word.chars() {
                let mut app_count = 0;
                for word in &words {
                    if word.contains(ltr) {
                        app_count += 1;
                    }
                }
                if app_count < threshold {
                    letters.push((app_count, ltr));
                }
            }
        }
        for word in all_words {
            let mut score = 0;
            for (freq, ltr) in &letters {
                match word.appearances(ltr) {
                    0 => {}
                    _ => score += freq,
                }
            }
            if score > 1 {
                result.push((score, word));
            }
        }
        result.sort_unstable_by(|a, b| b.cmp(a));
        result
    }
}
