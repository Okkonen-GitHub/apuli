// A subprogram to benchmark the library
// in terms of speed and accuracy.
// What matters is minimizing the amount of guesses required to finding the correct aswer
// And the speed of the algorithm (mainly that it is implemented in a reasonable way)
//
// And obviously that the program doesn't give wrong answers

#[cfg(test)]
mod apuli_bench {

    use crate::apuli::*;
    use std::collections::HashMap;

    // 1: Get the target word
    // 2: Generate the next guess (first is rank() with no args (it doesn't make sense to scout without
    //    information) then after that each guess' information needs to be added and word list
    //    filtered with removals)
    // 3: Add the information the guess would give us given the target word
    // 3.5: If we want to bench scouting, and if the number of guesses is in the range [2, 4]
    // 3.9: If 5 bencing scouting and reach 5 guesses without an answer, take the best word given
    //   by rank()
    // 4: repeat 2-3 until 6 guesses is reached or only one word remains
    #[test]
    fn guesses_to_win() {
        fn gen_grays(words: &[String], target: &str) -> Vec<Letter> {
            let mut grays: Vec<Letter> = vec![];
            let mut cache: HashMap<char, Vec<usize>> = HashMap::new();
            for word in words {
                for (i, letter) in word.chars().enumerate() {
                    if !target.contains(letter) {
                        let positions = cache.get_mut(&letter).cloned();
                        if let Some(mut positions) = positions {
                            positions.push(i);
                            cache.insert(letter, positions.to_vec());
                        } else {
                            cache.insert(letter, vec![i]);
                        }
                    }
                }
            }
            for (k, v) in cache {
                grays.push(Letter {
                    letter: k,
                    positions: Some(v),
                })
            }
            grays
        }
        fn gen_blues(words: &[String], target: &String) -> Option<Vec<Letter>> {
            let mut blues: Vec<Letter> = vec![];
            let mut cache: HashMap<char, Vec<usize>> = HashMap::new();
            for word in words {
                for (i, letter) in word.chars().enumerate() {
                    if letter != target.chars().nth(i).unwrap() && target.contains(letter) {
                        let positions = cache.get_mut(&letter).cloned();
                        if let Some(mut positions) = positions {
                            positions.push(i);
                            cache.insert(letter, positions.to_vec());
                        } else {
                            cache.insert(letter, vec![i]);
                        }
                    }
                }
            }
            for (k, v) in cache {
                blues.push(Letter {
                    letter: k,
                    positions: Some(v),
                })
            }

            if !blues.is_empty() {
                Some(blues)
            } else {
                None
            }
        }
        fn gen_oranges(words: &[String], target: &str) -> Option<Vec<Letter>> {
            let mut oranges: Vec<Letter> = vec![];
            let mut cache: HashMap<char, Vec<usize>> = HashMap::new();
            for word in words {
                for (i, letter) in word.chars().enumerate() {
                    if letter == target.chars().nth(i).unwrap() {
                        let positions = cache.get_mut(&letter).cloned();
                        if let Some(mut positions) = positions {
                            positions.push(i);
                            cache.insert(letter, positions.to_vec());
                        } else {
                            cache.insert(letter, vec![i]);
                        }
                    }
                }
            }
            for (k, v) in cache {
                oranges.push(Letter {
                    letter: k,
                    positions: Some(v),
                })
            }

            if !oranges.is_empty() {
                Some(oranges)
            } else {
                None
            }
        }

        let words_5 = all_words(5);
        for word in &words_5 {
            let mut guesses: Vec<String> = vec![];
            let mut words = words_5.clone();
            let mut next_guess: String = Default::default();
            while guesses.len() < 6 {
                if guesses.is_empty() {
                    next_guess = match rank(all_words(5)).first() {
                        Some((_, g_word)) => g_word.to_owned(),
                        None => panic!("No word remaining in possible words"),
                    };
                    guesses.push(next_guess.clone());
                    let grays = gen_grays(&guesses, word);
                    let blues = gen_blues(&guesses, word);
                    let oranges = gen_oranges(&guesses, word);
                    words = query(&grays, blues, oranges, 5)
                } else {
                    rank(words);
                    todo!()
                }
            }
        }
    }
}
