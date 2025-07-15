

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod apuli {
    use std::ops::Deref;


    const WORDS_6: &str = include_str!("../6_letter_words.txt");
    const WORDS_5: &str = include_str!("../5_letter_words.txt");


    pub const ALLOWED_KEYS: [char; 28] = [
        'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K',
        'L', 'Ö', 'Ä', 'Z', 'X', 'C', 'V', 'B', 'N', 'M',
    ];
    pub const ALLOWED_NUMS : [char; 6] = ['0', '1', '2', '3', '4', '5'];
    #[derive(Debug)]
    pub struct Letter {
        pub letter: char,
        pub positions: Option<Vec<usize>>,
    }

    trait ContainsN {
        // Returns true if string contains n amount of specified letter
        fn contains_n(&self, letter: &char, n: usize) -> bool;
        // Returns true if string contains AT LEAST n amount of specified letter
        fn contains_atleast_n(&self, letter: &char, n: usize) -> bool;
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
    }

    trait Removal {
        fn remove_grey(&mut self, grays: &Vec<Letter>, blues: Option<&Vec<Letter>>, oranges: Option<&Vec<Letter>>) -> Self;
        fn remove_others(&mut self, blues: Option<&Vec<Letter>>, oranges: Option<&Vec<Letter>>) -> Self;
    }

    impl Removal for Vec<String> {
        fn remove_grey(&mut self, grays: &Vec<Letter>, blues: Option<&Vec<Letter>>, oranges: Option<&Vec<Letter>>) -> Self {
            for gray in grays.iter() {
                let mut is_ominous = false;
                for word in self.clone().iter() {
                    if let Some(blues) = blues {
                        let mut known_count = 0;
                        for blue in blues {
                            if let Some(oranges) = oranges {
                                // is_ominous = true;
                                for orange in oranges {
                                    if orange.letter == gray.letter {
                                        is_ominous = true;
                                        known_count += orange.positions.as_ref().unwrap().len();
                                    }
                                    if blue.letter == gray.letter {
                                        is_ominous = true;
                                        known_count += 1;
                                    }
                                    if known_count != 0 && !word.contains_atleast_n(&blue.letter, known_count) {
                                        dbg!(word, known_count, blue.letter);
                                        if let Some(index) = self.iter().position(|x| x == word) {
                                            self.remove(index); // the word might have already been
                                        // removed earlier so we have to check in this (latter)
                                        // case
                                        }

                                        //self.remove(self.iter().position(|x| x == word).unwrap());
                                    }
                                    known_count = 0;
                                }
                            } else {
                                if blue.letter == gray.letter {
                                    is_ominous = true;
                                    known_count += 1;
                                }
                                if known_count != 0 && !word.contains_atleast_n(&blue.letter, known_count) {
                                    dbg!(word, known_count, blue.letter);
                                    self.remove(self.iter().position(|x| x == word).unwrap());
                                }
                            }
                            known_count = 0;
                        }

                    } else if let Some(oranges) = oranges {
                        for orange in oranges {
                            if orange.letter == gray.letter {
                                is_ominous = true;
                                if !word.contains_n(&orange.letter, orange.positions.as_ref().unwrap().len()) {
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
                        // println!("sanat: {:?}, sana: {}, pos: {pos}", words, word);
                        self.remove(self.iter().position(|x| x == word).unwrap());
                        // println!("sanat: {:?}, sana: {}, pos: {pos}", words, word);
                        // pos -= 1;
                    }
                }
            }
            self.to_vec()
        }
        fn remove_others(&mut self, oranges: Option<&Vec<Letter>>, blues: Option<&Vec<Letter>>) -> Self {
            match oranges {
                Some(oranges) => {
                    for word in self.clone().iter() {
                        if !check_oranges(oranges, word) {
                            self.remove(self.iter().position(|x| x == word).unwrap());
                        }
                    }
                },
                None => {}
            };
            match blues {
                Some(blues) => {
                    for word in self.clone().iter() {
                        if !check_blues(blues, word) {
                            self.remove(self.iter().position(|x| x == word).unwrap());
                        }
                    }
                },
                None => {
                    return self.to_vec();
                }
            }
            self.to_vec()
        }
    }

    fn check_blues(blues: &Vec<Letter>, guess: &String) -> bool {
        /* 
        *Returns true if some blue is found in the correct position
        Input: "SYÖPÄ", vec![Letter { letter: 'S', color: 1, positions: vec![3,4]}] --> true (sana kelpaa)
        Input: "SYÖPÄ", vec![Letter { letter: 'S', color: 1, positions: vec![0,2]] --> false (sana ei kelpaa)
        */
        let mut pos = 0; 
        for c in guess.chars() {

            for blue in blues.iter() {
                if blue.letter == c && blue.positions.as_ref().unwrap().contains(&pos) {
                    return false;
                }
            }
            pos += 1;
        }
        // at least n number of blues must be in the word though
        for blue in blues.iter() {
            if !guess.contains(blue.letter) {
                return false;
            }
        }
        true
    }

    fn check_oranges(oranges: &Vec<Letter>, guess: &String) -> bool {
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

    // fn remove_grey(mut words: Vec<String>, grays: Vec<Letter>) -> Vec<String> {
    //     for gray in grays.iter() {
    //         for word in words.clone().iter() {
    //             if word.contains(gray.letter) {
    //                 // println!("sanat: {:?}, sana: {}, pos: {pos}", words, word);
    //                 words.remove(words.iter().position(|x| x == word).unwrap());
    //                 // println!("sanat: {:?}, sana: {}, pos: {pos}", words, word);
    //                 // pos -= 1;
    //             }
    //         }
    //     }
    //     words
    // }

    // // removes all of the other invalid words
    // fn remove_others(mut words: Vec<String>, oranges: Option<&Vec<Letter>>, blues: Option<&Vec<Letter>>) -> Vec<String> {
    //     match oranges {
    //         Some(oranges) => {
    //             for word in words.clone().iter() {
    //                 if !check_oranges(oranges, word) {
    //                     words.remove(words.iter().position(|x| x == word).unwrap());
    //                 }
    //             }
    //         },
    //         None => {}
    //     };
    //     match blues {
    //         Some(blues) => {
    //             for word in words.clone().iter() {
    //                 if !check_blues(blues, word) {
    //                     words.remove(words.iter().position(|x| x == word).unwrap());
    //                 }
    //             }
    //         },
    //         None => {
    //             return words;
    //         }
    //     }
    //     words
    // }

    fn all_words(word_len: usize) -> Vec<String> {
        let mut words = Vec::new();
        match word_len {
            5 => {
                for word in WORDS_5.split("\n") {
                    words.push(word.to_owned())
                }
            },
            6 => {
                for word in WORDS_6.split("\n") {
                    words.push(word.to_owned())
                }
            },
            _ => {unreachable!()}
        }
        words
    }

    pub fn query(grays: &Vec<Letter>, blues: Option<&Vec<Letter>>, oranges: Option<&Vec<Letter>>, word_lenght: usize) -> Vec<String> {
        // let path = PathBuf::from("../apuli-lib/");
        // println!("{:?}", path);
        let mut words = all_words(word_lenght);

        words.remove_grey(grays, blues, oranges);
        match oranges {
            Some(oranges) => {
                words.remove_others(Some(&oranges), None);
            },
            None => {

            }
        };
        match blues {
            Some(blues) => {
                words.remove_others(None, Some(&blues));
            },
            None => {

            }
        };
        words
        // println!("Määrä: {}", words.len());
    }
}
