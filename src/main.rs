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
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
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
    hint: Vec<Hint>,
}

impl InputState {
    pub fn new(width: usize) -> Self {
        InputState {
            width,
            word: None,
            hint: vec![],
        }
    }

    pub fn add_word(&mut self, input: &str) -> Result<(), &'static str> {
        if input.trim().len() != self.width {
            return Result::Err("invalid word length");
        }
        self.word = Option::from(input.trim().to_string());
        Result::Ok(())
    }

    pub fn add_hint(&mut self, input: &str) -> Result<(), &'static str> {
        if self.word.is_none() {
            return Result::Err("add word before");
        }
        let trimmed = input.trim();
        if trimmed.len() != self.width {
            return Result::Err("invalid length");
        }
        for c in trimmed.chars() {
            match c {
                '0' | '1' | '2' => {}
                _ => {
                    return Result::Err("input must be 1,2,3");
                }
            }
        }
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


    #[cfg(test)]
    mod add_word {
        use super::*;

        #[test]
        fn valid() {
            let mut state = InputState::new(5);
            let actual = state.add_word("apple");
            assert!(actual.is_ok());
        }

        #[test]
        fn invalid() {
            let mut state = InputState::new(5);
            let actual = state.add_word("banana");
            assert!(!actual.is_ok());
        }
    }

    #[cfg(test)]
    mod add_hint {
        use super::*;

        #[test]
        #[ignore]
        fn valid() {
            let mut state = InputState::new(5);
            state.add_word("bound").unwrap();
            let actual = state.add_hint("00120");
            assert!(actual.is_ok());
            assert_eq!(state.hint, vec![
                Hint::new('b', Spot::None()),
                Hint::new('o', Spot::None()),
                Hint::new('u', Spot::InWithout(vec![2])),
                Hint::new('n', Spot::At(3)),
                Hint::new('d', Spot::None()),
            ]);
        }

        #[test]
        fn invalid_nums() {
            let inputs = vec!["30120", "a0120", "001201"];
            for input in inputs {
                let mut state = InputState::new(5);
                state.add_word("apple").unwrap();
                let actual = state.add_hint(input);
                assert!(!actual.is_ok());
            }
        }

        #[test]
        fn invalid_no_word() {
            let mut state = InputState::new(5);
            let actual = state.add_hint("00120");
            assert!(!actual.is_ok());
        }
    }
}