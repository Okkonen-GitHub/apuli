use apuli_lib::apuli::*;
use std::env::args;

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
        }
        _ => {
            panic!("Invalid word length")
        }
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
            if !ALLOWED_KEYS.contains(&c.to_uppercase().next().unwrap())
                && c != ':'
                && !ALLOWED_NUMS.contains(&c)
            {
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
                Some(ltr) => match nums {
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
                    }
                    None => break,
                },
                None => break,
            }
        }
    }
    if args.next() == Some("-o".to_string()) {
        let oranges_str: String = args.next().expect("No blues given");
        for c in oranges_str.chars() {
            if !ALLOWED_KEYS.contains(&c.to_uppercase().next().unwrap())
                && c != ':'
                && !ALLOWED_NUMS.contains(&c)
            {
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
                Some(ltr) => match nums {
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
                    }
                    None => break,
                },
                None => break,
            }
        }
    }
    // println!("grays {:#?}", grays);
    // println!("blues {:#?}", blues);
    // println!("oranges {:#?}", oranges);
    // get input and then query
    if oranges.as_mut().unwrap().is_empty() {
        oranges = None;
    }
    if blues.as_mut().unwrap().is_empty() {
        blues = None;
    }

    let result = query(&grays, blues, oranges, word_length);
    let ranked = rank(result);

    let _ = ranked
        .iter()
        .enumerate()
        .for_each(|(index, (score, word))| {
            // println!("{index}.  {word}  @[{score}]")
        });

    println!("Määrä: {}", ranked.len());
}
