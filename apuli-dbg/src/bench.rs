// A subprogram to benchmark the library
// in terms of speed and accuracy.
// What matters is minimizing the amount of guesses required to finding the correct aswer
// And the speed of the algorithm (mainly that it is implemented in a reasonable way)
//
// And obviously that the program doesn't give wrong answers

#[cfg(test)]
mod apuli_bench {

    use apuli_lib::{apuli::*, util::cache_insert};
    use std::collections::HashMap;

    // Predetermined first guesses
    const FIRST_5_GUESS: &str = "KASTI";
    const FIRST_6_GUESS: &str = "KARSTI";

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
        fn gen_blues(words: &[String], target: &str) -> Option<Vec<Letter>> {
            let mut blues: Vec<Letter> = vec![];
            let mut cache: HashMap<char, Vec<usize>> = HashMap::new();
            for word in words {
                for (i, letter) in word.chars().enumerate() {
                    if letter != target.chars().nth(i).unwrap() && target.contains(letter) {
                        // dbg!(words, letter, target);
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
        fn gen_all(
            words: &[String],
            mut target: String,
        ) -> (Vec<Letter>, Vec<Letter>, Vec<Letter>) {
            let mut oranges: Vec<Letter> = vec![];
            let mut blues: Vec<Letter> = vec![];
            let mut grays: Vec<Letter> = vec![];
            let mut orange_cache: HashMap<char, Vec<usize>> = HashMap::new();
            let mut blue_cache: HashMap<char, Vec<usize>> = HashMap::new();
            let mut gray_cache: HashMap<char, Vec<usize>> = HashMap::new();
            for word in words {
                for (i, letter) in word.chars().enumerate() {
                    if letter == target.chars().nth(i).unwrap() {
                        cache_insert(&mut orange_cache, letter, i);
                    } else if word.contains(letter) {
                        cache_insert(&mut blue_cache, letter, i);
                        target = target.replacen(letter, " ", 1);
                    } else {
                        cache_insert(&mut gray_cache, letter, i);
                    }
                }
            }
            for (k, v) in orange_cache {
                oranges.push(Letter {
                    letter: k,
                    positions: Some(v),
                })
            }
            for (k, v) in blue_cache {
                blues.push(Letter {
                    letter: k,
                    positions: Some(v),
                })
            }
            for (k, v) in gray_cache {
                grays.push(Letter {
                    letter: k,
                    positions: Some(v),
                })
            }
            (grays, blues, oranges)
        }
        for l in 5..=6 {
            let mut data = HashMap::new();
            let mut scores: Vec<usize> = vec![];

            let all_n_words = all_words(l);
            for word in &all_n_words {
                let mut guesses: Vec<String> = vec![];
                let mut words = all_n_words.clone();
                let mut next_guess: String;
                while guesses.len() < 6 && words.len() > 1 {
                    dbg!(&guesses, &words);
                    if guesses.is_empty() {
                        // scout first?
                        // next_guess = match rank_scout(all_words(l), l).first() {
                        //     Some((_, g_word)) => g_word.to_owned(),
                        //     None => panic!("No word remaining in possible words"),
                        // };
                        next_guess = match l {
                            5 => FIRST_5_GUESS.to_owned(),
                            6 => FIRST_6_GUESS.to_owned(),
                            _ => unreachable!(),
                        };
                    } else if guesses.len() < 5 {
                        // scout
                        let ranked: Vec<String> =
                            rank(words).iter().map(|(_, x)| x.to_owned()).collect();
                        next_guess = match rank_scout(ranked.clone(), l).first() {
                            Some((_, g_word)) => g_word.to_owned(),
                            None => ranked.first().unwrap().to_owned(),
                        };
                    } else {
                        next_guess = match rank(words).first() {
                            Some((_, g_word)) => g_word.to_owned(),
                            None => panic!("No words remaining in possible words"),
                        };
                    }
                    guesses.push(next_guess);
                    // let grays = gen_grays(&guesses, word);
                    // let blues = gen_blues(&guesses, word);
                    // let oranges = gen_oranges(&guesses, word);
                    let (oranges, blues, grays) = gen_all(&guesses, word.clone());
                    words = query(&grays, Some(blues), Some(oranges), l);
                }
                {
                    let result = rank(words.clone());
                    data.insert("remaining", format!("{}", result.len()));
                    data.insert("Best guess", format!("{:?}", result.first()));
                    data.insert("TARGET", word.to_owned());
                    data.insert("GUESS COUNT", guesses.len().to_string());
                    data.insert("Guesses", format!("{:?}", guesses));
                    // assert_eq!(*word, result.first().unwrap().1);
                    dbg!(word);
                    if *word != result.first().unwrap().1 {
                        dbg!(&data);
                    }
                    scores.push(guesses.len() + 1);
                    // if guesses.len() >= 5 {
                    //     dbg!(guesses, word, result);
                    // }
                    // dbg!(&data);
                }
                // break;
            }
            let avg: f64 = scores.iter().sum::<usize>() as f64 / scores.len() as f64;
            let not_solved: usize = scores
                .iter()
                .filter(|&x| *x >= 6)
                .collect::<Vec<&usize>>()
                .len();
            dbg!(avg, not_solved, scores.len());
        }
    }
    #[test]
    fn pattern_generation() {
        use apuli_lib::information::{generate_patterns, State};
        let remaining = vec![
            "RYHMÄ", "MYÖHÄ", "NÖYRÄ", "HÖYNÄ", "RYÖNÄ", "MYÖDÄ", "MYYRÄ", "MÖNJÄ", "HYHMÄ",
            "HYRRÄ", "MÄYRÄ", "MYYJÄ", "MYYDÄ", "JÄYHÄ", "RÄHMÄ", "HÄRMÄ", "JÄYNÄ", "JÄÄHY",
            "RÄHJÄ", "HÄÄYÖ", "RYHMY", "MÖYHY", "ÄRJYÄ", "HÖYRY", "RÖYHY", "MÄÄRÄ", "NÄHDÄ",
            "RUGBY", "MURJU", "JÄÄRÄ", "JÄÄMÄ", "CURRY", "JÄNNÄ", "HUURU", "JÄÄDÄ", "HÖRHÖ",
            "JUNNU", "NYNNY", "MÖMMÖ", "MUMMU",
        ];
        let remaining = remaining
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let result = generate_patterns("RYHMÄ".to_owned(), &remaining);
        let key = vec![
            State::Absent,
            State::Present,
            State::Present,
            State::Absent,
            State::Present,
        ];
        dbg!(result.get(&key));
        // dbg!(result);
        let remaining = vec![
            "RYHMÄ", "MYÖHÄ", "NÖYRÄ", "HÖYNÄ", "RYÖNÄ", "MYÖDÄ", "MYYRÄ", "MÖNJÄ", "HYHMÄ",
            "HYRRÄ", "MÄYRÄ", "MYYJÄ", "MYYDÄ", "JÄYHÄ", "RÄHMÄ", "HÄRMÄ", "JÄYNÄ", "JÄÄHY",
            "RÄHJÄ", "HÄÄYÖ", "RYHMY", "MÖYHY", "ÄRJYÄ", "HÖYRY", "RÖYHY", "MÄÄRÄ", "NÄHDÄ",
            "RUGBY", "MURJU", "JÄÄRÄ", "JÄÄMÄ", "CURRY", "JÄNNÄ", "HUURU", "JÄÄDÄ", "HÖRHÖ",
            "JUNNU", "NYNNY", "MÖMMÖ", "MUMMU",
        ];
        let remaining = remaining
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let result = generate_patterns("ÄÄMMÄ".to_owned(), &remaining);
        let key = vec![
            State::Correct,
            State::Absent,
            State::Absent,
            State::Absent,
            State::Correct,
        ];
        dbg!(result.get(&key));
    }
}
