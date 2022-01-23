use std::io;
use std::fs::File;
use std::io::BufRead;

use wordle_resolver::{Hint, Resolver, SimpleResolver, Spot};

const DICT_PATH: &str = "data/words_alpha.txt";

fn main() {
    // let mut resolver = SimpleResolver::new(5, &get_words());

    let mut state = InputState::new(5);

    loop {
        println!("\nPlease input you guessed word:");
        let mut guessed_word = String::new();
        match io::stdin().read_line(&mut guessed_word) {
            Ok(_) => {
                match state.add_word(&guessed_word) {
                    Ok(_) => break,
                    Err(_) => (),
                }
                eprintln!("input word of length must be {}", 5);
            }
            Err(e) => {
                eprintln!("failed to input word: {}", e);
            }
        }
    }

    loop {
        println!("\nPlease input result (0=not matched, 1=any, 2=exact):");
        let mut hint_input = String::new();
        match io::stdin().read_line(&mut hint_input) {
            Ok(_) => {
                eprintln!("input word of length must be {}", 5);
            }
            Err(e) => {
                eprintln!("failed to input word: {}", e);
            }
        }
    }

    // resolver.add_hint(vec![Hint::new('a', Spot::None())]);
    // resolver.add_hint(vec![Hint::new('t', Spot::At(vec![1]))]);
    // for guessed in resolver.guess() {
    //     println!("{}", guessed);
    // }
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

struct InputState {
    width: usize,
    word: Option<String>,
    hint_input: Option<String>,
}

impl InputState {
    pub fn new(width: usize) -> Self {
        InputState {
            width,
            word: None,
            hint_input: None,
        }
    }

    pub fn add_word(&mut self, input: &str) -> Result<(), &'static str> {
        if input.trim().len() != self.width {
            return Result::Err("invalid word length");
        }
        self.word = Option::from(input.trim().to_string());
        Result::Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let actual = InputState::new(5);
        assert_eq!(actual.width, 5);
    }
}