// pub mod information {
use std::collections::HashMap;

use crate::util::cache_insert;

#[derive(Default, Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum State {
    #[default]
    Absent,
    Present,
    Correct,
}

fn word_probability(words_in_pattern: &[String], remaining: &[String]) -> f64 {
    // all are equally likely
    words_in_pattern.len() as f64 / remaining.len() as f64
}

fn information_entropy(probability: f64) -> f64 {
    if probability <= 0.0 {
        panic!("probability can't be 0, or negative");
    }
    -probability.log2()
}

pub fn remaining_information(remaining: &[String]) -> f64 {
    (remaining.len() as f64).log2()
}

pub fn generate_patterns(
    guess: String,
    remaining_words: &[String],
) -> HashMap<Vec<State>, Vec<String>> {
    let mut patterns = HashMap::new();
    for a_word in remaining_words {
        let mut word = a_word.clone();
        let mut pattern: Vec<State> = std::iter::repeat(State::Absent).take(guess.len()).collect();
        for (i, letter) in guess.clone().chars().enumerate() {
            if word.chars().nth(i).unwrap() == letter {
                pattern[i] = State::Correct;
            } else if word.contains(letter) {
                pattern[i] = State::Present;
                word = word.replacen(letter, " ", 1);
            } else {
                pattern[i] = State::Absent;
            }
        }
        cache_insert(&mut patterns, pattern, a_word.clone());
    }
    patterns
}

// avg entropy for a given word
fn expected_information(guess: String, remaining_words: &[String]) -> f64 {
    let word_patterns = generate_patterns(guess, remaining_words);
    let probabilities: Vec<f64> = word_patterns
        .iter()
        .map(|(_pattern, matching_words)| word_probability(&matching_words, &remaining_words))
        .collect();
    let probaility_entropy_pairs: Vec<(f64, f64)> = probabilities
        .iter()
        .map(|p| (*p, information_entropy(*p)))
        .collect();
    probaility_entropy_pairs.iter().map(|(p, e)| e * p).sum()
}

pub fn rank_entropy(remaining_words: &[String]) -> Vec<(f64, String)> {
    let mut result: Vec<(f64, String)> = remaining_words
        .iter()
        .map(|word| {
            (
                expected_information(word.clone(), remaining_words),
                word.clone(),
            )
        })
        .collect();
    result.sort_unstable_by(|a, b| b.partial_cmp(a).expect("Entropy to be not NaN"));
    result
}
// }
