mod information {
    use std::collections::HashMap;

    fn word_probability(remaining: &[String]) -> f64 {
        // all are equally likely
        1_f64 / remaining.len() as f64
    }

    fn information_entropy(probability: f64) -> f64 {
        if probability <= 0.0 {
            panic!("probability can't be 0, or negative");
        }
        -probability.log2()
    }

    fn remaining_information(remaining: &[String]) -> f64 {
        (remaining.len() as f64).log2()
    }
}
