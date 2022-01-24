use std::fs::File;
use std::io;
use std::io::BufRead;

use std::iter::zip;
use ansi_term::Color::{Green, Yellow};

use ansi_term::Colour::Red;
use ansi_term::Style;
use clap::Parser;
use num_format::{Locale, ToFormattedString};

use wordle_solver::{Hint, Solver, Spot};
use wordle_solver::simple::SimpleSolver;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Config {
    #[clap(short, long, default_value_t = 5)]
    word_length: usize,

    #[clap(short, long, default_value = "data/words_alpha.txt")]
    dict_path: String,
}

fn main() {
    let config = Config::parse();

    let mut solver: Box<dyn Solver> = Box::new(SimpleSolver::new(config.word_length, &get_words(&config.dict_path)));

    println!("{}\n", Style::new().bold().paint("Welcome to WORDLE SOLVER"));

    loop {
        println!("There are {} words are remained.\n", solver.remining_words_length().to_formatted_string(&Locale::en));

        let mut state = InputState::new(config.word_length);

        loop {
            println!("Please input you guessed word:");
            let mut guessed_word = String::new();
            match io::stdin().read_line(&mut guessed_word) {
                Ok(_) => {
                    match state.add_word(&guessed_word) {
                        Ok(_) => break,
                        Err(e) => {
                            eprintln!("{}\n", Red.paint(e));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}\n", Red.paint(format!("failed to input word: {}", e)));
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
                            eprintln!("{}\n", Red.paint(e));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}\n", Red.paint(format!("failed to input word: {}", e)));
                }
            }
        }

        println!("{}", state.colorized_input().unwrap());

        let (word, hints) = state.get().unwrap();

        solver.add_hint(word, hints);

        for guessed in solver.guess() {
            println!("{}", guessed);
        }
    }
}

fn get_words(dict_path: &str) -> Vec<String> {
    let file = match File::open(dict_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to load '{}': {}", dict_path, e);
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
                    return Result::Err("input must be 0,1,2");
                }
            }
        }
        Result::Ok(())
    }

    pub fn colorized_input(&self) -> Result<String, &'static str> {
        if self.word.is_none() {
            return Result::Err("word are empty");
        }
        if self.hint.is_empty() {
            return Result::Err("hints are empty");
        }
        let mut chars: Vec<String> = Vec::new();
        for (c, hint) in zip(self.word.as_ref().unwrap().chars(), &self.hint) {
            let res = match hint.spot {
                Spot::None() => format!("{}", Style::new().bold().paint(c.to_string())),
                Spot::InWithout(_) => format!("{}", Style::new().on(Yellow).bold().paint(c.to_string())),
                Spot::At(_) => format!("{}", Style::new().on(Green).bold().paint(c.to_string())),
            };
            chars.push(res);
        }
        return Result::Ok(chars.join(""));
    }

    pub fn get(&self) -> Result<(&str, &Vec<Hint>), &'static str> {
        if self.word.is_none() {
            return Result::Err("word are empty");
        }
        if self.hint.is_empty() {
            return Result::Err("hints are empty");
        }
        return Result::Ok((
            self.word.as_ref().unwrap(),
            &self.hint,
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