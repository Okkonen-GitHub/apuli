use std::fs;
use std::env::current_dir;
use std::path::PathBuf;


fn read_words(path: PathBuf) -> (Vec<String>, Vec<String>) {
    let file = fs::read_to_string(path);
    let mut words_5 = Vec::new();
    let mut words_6 = Vec::new();
    if let Ok(file) = file {
        for line in file.lines() {
            if line.chars().count() == 5 {
                words_5.push(line.to_owned());
            } else if line.chars().count() == 6 {
                words_6.push(line.to_owned());
            }       
        }
    }
    println!("bruh");
    (words_5, words_6)
}

fn write_files(path: PathBuf) {
    
    let (words_5, words_6) = read_words(path.clone());
    
    let path = current_dir().unwrap();
    let mut final_5 = String::new();
    for word in words_5.iter() {
        final_5.push_str(word);
        final_5.push_str("\n")
    }
    let mut final_6 = String::new();
    for word in words_6.iter() {
        final_6.push_str(word);
        final_6.push_str("\n")
    }

    let word_file_6 = fs::write(path.join("5_letter_words.txt"), final_5);
    let word_file_6 = fs::write(path.join("6_letter_words.txt"), final_6);
    
}

fn main() {
    let pwd = current_dir().expect("lol mitä");
    let pwd = pwd.join("full-words-generated.txt");
    println!("{:?}", pwd);
    write_files(pwd);
    println!("Hello, world!");
}
