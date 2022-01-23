use std::io;
use std::fs::File;
use std::io::BufRead;

use wordle_resolver::{Hint, Resolver, SimpleResolver, Spot};

const DICT_PATH: &str = "data/words_alpha.txt";

fn main() {
    let mut resolver = SimpleResolver::new(5, &get_words());

    loop {
        println!("\nPlease input you guessed word:");
        let mut guessed_word = String::new();
        match io::stdin().read_line(&mut guessed_word) {
            Ok(_) => {
                if guessed_word.trim().len() == 5 {
                    break;
                }
                eprintln!("input word of length must be {}", 5);
            }
            Err(e) => {
                eprintln!("failed to input word: {}", e);
            }
        }
    }

    resolver.add_hint(vec![Hint::new('a', Spot::None())]);
    resolver.add_hint(vec![Hint::new('t', Spot::At(vec![1]))]);
    for guessed in resolver.guess() {
        println!("{}", guessed);
    }
}

fn get_words() -> Vec<String> {
    let file = match File::open(DICT_PATH) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to load '{}': {}", DICT_PATH, e);
            std::process::exit(1);
        }
    };
    let dict: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|e| {
            e.ok()
        })
        .map(|line| {
            String::from(line.trim())
        })
        .collect();
    dict
}
