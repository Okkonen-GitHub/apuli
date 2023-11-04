// A subprogram to benchmark the library
// in terms of speed and accuracy.
// What matters is minimizing the amount of guesses required to finding the correct aswer
// And the speed of the algorithm (mainly that it is implemented in a reasonable way)
//
// And obviously that the program doesn't give wrong answers

mod apuli_bench {

    use crate::apuli::*;

    // 1: Get the target word
    // 2: Generate the next guess (first is rank() with no args (it doesn't make sense to scout without
    //    information) then after that each guess' information needs to be added and word list
    //    filtered with removals)
    // 3: Add the information the guess would give us given the target word
    // 3.5: If we want to bench scouting, and if the number of guesses is in the range [2, 4]
    // 3.9: If 5 bencing scouting and reach 5 guesses without an answer, take the best word given
    //   by rank()
    // 4: repeat 2-3 until 6 guesses is reached or only one word remains
    fn guesses_to_win() {
        fn get_next_guess(prev: &[&str]) -> String {
            let next_guess: String = Default::default();
            if prev.is_empty() {
                rank(all_words(5));
            }
            next_guess
        }

        let words_5 = all_words(5);
        for word in &words_5 {
            let guesses: &[&str] = &[];
            let mut words = words_5.clone();
            get_next_guess(&guesses);
        }
    }
}
