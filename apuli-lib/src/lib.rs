

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod apuli {
    use std::fs;
    use std::path::PathBuf;


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

    trait Removal {
        fn remove_grey(&mut self, grays: Vec<Letter>) -> Self;
        fn remove_others(&mut self, blues: Option<&Vec<Letter>>, oranges: Option<&Vec<Letter>>) -> Self;
    }

    impl Removal for Vec<String> {
        fn remove_grey(&mut self, grays: Vec<Letter>) -> Self {
            for gray in grays.iter() {
                for word in self.clone().iter() {
                    if word.contains(gray.letter) {
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
        // one blue must be in the word though
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

    fn all_words(base_path: PathBuf, word_len: usize) -> Vec<String> {
        let file = fs::read_to_string(base_path.join(format!("{}_letter_words.txt", word_len)));
        let mut words = Vec::new();
        if let Ok(file) = file {
            for line in file.lines() {
                words.push(line.to_owned())
            }
        }
        words
    }

    pub fn query(grays: Vec<Letter>, blues: Option<&Vec<Letter>>, oranges: Option<&Vec<Letter>>, word_lenght: usize) -> Vec<String> {
        let path = PathBuf::from("../../apuli-lib/");
        println!("{:?}", path);
        let mut words = all_words(path, word_lenght);
        
        words.remove_grey(grays);
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
