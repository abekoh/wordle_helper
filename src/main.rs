use std::fs::File;
use std::io;
use std::io::BufRead;

use wordle_resolver::{Hint, Resolver, SimpleResolver, Spot};

const DICT_PATH: &str = "data/words_alpha.txt";

fn main() {
    let mut resolver = SimpleResolver::new(5, &get_words());

    loop {
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
                    match state.add_hint(&hint_input) {
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

        let (word, hints) = state.get().unwrap();

        resolver.remove_word(&word);
        resolver.add_hint(hints);

        for guessed in resolver.guess() {
            println!("{}", guessed);
        }
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
        for (i, hint_c) in trimmed.chars().enumerate() {
            let word_c = self.word.as_ref().unwrap().chars().nth(i).unwrap();
            match hint_c {
                '0' => {
                    self.hint.push(Hint::new(word_c, Spot::None()));
                }
                '1' => {
                    self.hint.push(Hint::new(word_c, Spot::InWithout(i)));
                }
                '2' => {
                    self.hint.push(Hint::new(word_c, Spot::At(i)));
                }
                _ => {
                    return Result::Err("input must be 1,2,3");
                }
            }
        }
        Result::Ok(())
    }

    pub fn get(&self) -> Result<(String, Vec<Hint>), &'static str> {
        if self.word.is_none() {
            return Result::Err("word are empty");
        }
        if self.hint.is_empty() {
            return Result::Err("hints are empty");
        }
        return Result::Ok((
            self.word.clone().unwrap(),
            self.hint.clone(),
        ));
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
        fn valid() {
            let mut state = InputState::new(5);
            state.add_word("bound").unwrap();
            let actual = state.add_hint("00120");
            assert!(actual.is_ok());
            assert_eq!(state.hint, vec![
                Hint::new('b', Spot::None()),
                Hint::new('o', Spot::None()),
                Hint::new('u', Spot::InWithout(2)),
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