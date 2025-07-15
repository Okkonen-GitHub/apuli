use std::fs;
use std::env::current_dir;
use std::path::PathBuf;

struct Letter {
    letter: char,
    color: usize, // 0 = Gray, 1 = Blue, 2 = Orange
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
            if guess.get(*pos..*pos+1).unwrap() != orange.letter.to_string() {
                return false;
            }
        }
    }
    true
}

fn remove_grey(mut words: Vec<String>, grays: Vec<Letter>) -> Vec<String> {
    for gray in grays.iter() {
        let mut pos = 0;
        for word in words.clone().iter() {
            if word.contains(gray.letter) {
                println!("sanat: {:?}, sana: {}, pos: {pos}", words, word);
                words.remove(pos);
                println!("sanat: {:?}, sana: {}, pos: {pos}", words, word);
                // pos -= 1;
            } else {
                pos += 1;
            }
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

fn query() {
    let oranges = vec![
        Letter {
            letter: 'A',
            color: 2,
            positions: Some(vec![0,1]),
        },
        Letter {
            letter: 'O',
            color: 2,
            positions: Some(vec![4]),
        },
    ];
    let guess = "AALTO".to_string();
    println!("{}",check_oranges(&oranges, &guess));
    // first remove the greys
    // then check oranges, then blues
    let words = vec!["PULLO".to_string(), "JÄNIS".to_string(), "AALTO".to_string(), "AMMUU".to_string(), "ÄÄLIÖ".to_string(), "VÄÄRÄ".to_string()];
    let grays = vec![
        Letter {
            letter: 'A',
            color: 0,
            positions: None
        },
        Letter {
            letter: 'O',
            color: 0,
            positions: None,
        },
    ];
    let words = remove_grey(words, grays);
    println!("{:?}", words);
}

fn main() {
    let path = current_dir().unwrap();
    let words = all_words(path, 5);
    println!("{:?}", words.last().unwrap());
    // get input and then query
    query();
}
