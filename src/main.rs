use std::fs;
use std::env::{current_dir, args};
use std::path::PathBuf;


const ALLOWED_KEYS: [char; 28] = [
    'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K',
    'L', 'Ö', 'Ä', 'Z', 'X', 'C', 'V', 'B', 'N', 'M',
];
const ALLOWED_NUMS : [char; 6] = ['0', '1', '2', '3', '4', '5'];
#[derive(Debug)]
struct Letter {
    letter: char,
    positions: Option<Vec<usize>>,
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

fn remove_grey(mut words: Vec<String>, grays: Vec<Letter>) -> Vec<String> {
    for gray in grays.iter() {
        for word in words.clone().iter() {
            if word.contains(gray.letter) {
                // println!("sanat: {:?}, sana: {}, pos: {pos}", words, word);
                words.remove(words.iter().position(|x| x == word).unwrap());
                // println!("sanat: {:?}, sana: {}, pos: {pos}", words, word);
                // pos -= 1;
            }
        }
    }
    words
}

// removes all of the other invalid words
fn remove_others(mut words: Vec<String>, oranges: Option<&Vec<Letter>>, blues: Option<&Vec<Letter>>) -> Vec<String> {
    match oranges {
        Some(oranges) => {
            for word in words.clone().iter() {
                if !check_oranges(oranges, word) {
                    words.remove(words.iter().position(|x| x == word).unwrap());
                }
            }
        },
        None => {}
    };
    match blues {
        Some(blues) => {
            for word in words.clone().iter() {
                if !check_blues(blues, word) {
                    words.remove(words.iter().position(|x| x == word).unwrap());
                }
            }
        },
        None => {
            return words;
        }
    }
    words
}

fn all_words(base_path: PathBuf, word_len: usize) -> Vec<String> {
    let file = fs::read_to_string(base_path.join(format!("{}_letter_words.txt", word_len)));
    let mut words = Vec::new();
    if let Ok(file) = file {
        for line in file.lines() {
            words.push(line.to_owned())
        }
    }
    println!("bruh");
    words
}

fn query(path: PathBuf, grays: Vec<Letter>, blues: Option<&Vec<Letter>>, oranges: Option<&Vec<Letter>>, word_lenght: usize) {
    let mut words = all_words(path, word_lenght);
    
    words = remove_grey(words, grays);
    match oranges {
        Some(oranges) => {
            words = remove_others(words, Some(&oranges), None);
        },
        None => {

        }
    };
    match blues {
        Some(blues) => {
            words = remove_others(words.clone(), None, Some(&blues));
        },
        None => {

        }
    };
    for word in &words {
        println!("{}", word);
    }
    println!("Määrä: {}", words.len());
}

fn main() {

    let mut grays = Vec::new();
    let mut blues = Some(Vec::new());
    let mut oranges = Some(Vec::new());

    let mut args = args().skip(1);
    let word_length;
    match args.next() {
        Some(n) => {
            if n == "-5".to_string() {
                word_length = 5;
            } else if n == "-6".to_string() {
                word_length = 6;
            } else {
                println!("Invalid word length");
                return;
            }
        },
        _ => {panic!("Invalid word length")}
    }

    if args.next() == Some("-g".to_string()) {
        let grays_str: String = args.next().expect("No grays given");
        for c in grays_str.chars() {
            if !ALLOWED_KEYS.contains(&c.to_uppercase().next().unwrap()) {
                panic!("Invalid argument: {}", c);
            } else {
                grays.push(Letter {
                    letter: c.to_uppercase().next().unwrap(),
                    positions: None,
                });
            }
        }
    }
    if args.next() == Some("-b".to_string()) {
        let blues_str: String = args.next().expect("No blues given");
        for c in blues_str.chars() {
            if !ALLOWED_KEYS.contains(&c.to_uppercase().next().unwrap()) && c != ':' && !ALLOWED_NUMS.contains(&c) {
                panic!("Invalid argument: {}", c);
            }
        }
        let mut element = blues_str.split(":");
        loop {
            let mut positions: Vec<usize> = Vec::new();
            
            // println!("{:?}, {:?}", element.next(), element.next());
            
            let ltr = element.next();
            let nums = element.next();

            match ltr {
                Some(ltr) => {
                    match nums {
                        Some(nums) => {
                            for c in nums.chars() {
                                if !c.is_numeric() {
                                    panic!("Invalid argument: {}", c);
                                } else {
                                    positions.push(c.to_digit(10).unwrap() as usize);
                                }
                            }
                            blues.as_mut().unwrap().push(Letter {
                                letter: ltr.chars().next().unwrap().to_uppercase().next().unwrap(),
                                positions: Some(positions),
                            });
                        },
                        None => break,
                    }
                },
                None => break,
            }
        }
    }
    if args.next() == Some("-o".to_string()) {
        let oranges_str: String = args.next().expect("No blues given");
        for c in oranges_str.chars() {
            if !ALLOWED_KEYS.contains(&c.to_uppercase().next().unwrap()) && c != ':' && !ALLOWED_NUMS.contains(&c) {
                panic!("Invalid argument: {}", c);
            }
        }
        let mut element = oranges_str.split(":");
        loop {
            let mut positions: Vec<usize> = Vec::new();
            
            // println!("{:?}, {:?}", element.next(), element.next());
            
            let ltr = element.next();
            let nums = element.next();

            match ltr {
                Some(ltr) => {
                    match nums {
                        Some(nums) => {
                            for c in nums.chars() {
                                if !c.is_numeric() {
                                    panic!("Invalid argument: {}", c);
                                } else {
                                    positions.push(c.to_digit(10).unwrap() as usize);
                                }
                            }
                            oranges.as_mut().unwrap().push(Letter {
                                letter: ltr.chars().next().unwrap().to_uppercase().next().unwrap(),
                                positions: Some(positions),
                            });
                        },
                        None => break,
                    }
                },
                None => break,
            }
        }
    }
    // println!("grays {:#?}", grays);
    // println!("blues {:#?}", blues);
    // println!("oranges {:#?}", oranges);
    let path = current_dir().unwrap();
    // get input and then query
    if oranges.as_mut().unwrap().len() == 0 {
        oranges = None;
    }
    if blues.as_mut().unwrap().len() == 0 {
        blues = None;
    }
    match &oranges {
        Some(oranges) => {
            match &blues {
                Some(blues) => {
                    query(path, grays, Some(blues), Some(oranges), word_length);
                },
                None => {
                    query(path, grays, None, Some(oranges), word_length);
                }
            }
        }
        None => {
            match &blues {
                Some(blues) => {
                    query(path, grays, Some(&blues), None, word_length);
                },
                None => {
                    query(path, grays, None, None, word_length);
                }
            }
        }
    };

}
