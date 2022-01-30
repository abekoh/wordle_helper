use std::iter::zip;

use ansi_term::{Colour, Style};
use ansi_term::Color::{RGB, White};
use clap::Parser;
use dialoguer::{Confirm, FuzzySelect, Input};
use dialoguer::theme::ColorfulTheme;
use num_format::{Locale, ToFormattedString};

use wordle_solver::{Dictionary, Hint, Solver, Spot};
use wordle_solver::txt::TxtDictionary;
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

    let dictionary: Box<dyn Dictionary> = Box::new(match TxtDictionary::new(&config.dict_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("failed to load dictionary: {}", e);
            std::process::exit(1);
        }
    });

    let mut solver: Box<dyn Solver> = Box::new(SimpleSolver::new(config.word_length, &dictionary.extract_words(config.word_length)));
    let mut states: InputStates = InputStates::new();

    println!("{}\n", Style::new().bold().paint("Welcome to WORDLE SOLVER"));

    loop {
        println!("There are {} words are remained.\n", solver.remained_words_length().to_formatted_string(&Locale::en));

        loop {
            let mut state = InputState::new(config.word_length);

            let suggested = solver.suggest();
            let selected = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Guess")
                .default(0)
                .items(suggested)
                .interact()
                .unwrap();
            state.add_word(&suggested[selected]).unwrap();

            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Correct?")
                .interact()
                .unwrap()
            {
                state.correct();
                println!("{}\n", Style::new().bold().paint("Congratulation!!"));
                println!("{}", states.preview(&state).unwrap());
                std::process::exit(0);
            }

            let hint_input = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Hint")
                .validate_with({
                    move |input: &String| -> Result<(), &str> {
                        if input.len() != config.word_length {
                            return Err("invalid length");
                        }
                        if input.chars()
                            .filter(move |c| {
                                *c == '0' || *c == '1' || *c == '2'
                            })
                            .count() != config.word_length {
                            return Err("invalid number contains");
                        }
                        Ok(())
                    }
                })
                .interact_text()
                .unwrap();
            state.add_hint(&hint_input).unwrap();


            println!("{}", states.preview(&state).unwrap());

            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Is OK?")
                .default(true)
                .interact()
                .unwrap()
            {
                let (word, hints) = state.get().unwrap();
                solver.add_hint(word, hints);
                states.add(state);
                break;
            }
        }
    }
}

struct InputState {
    width: usize,
    word: Option<String>,
    hint: Vec<Hint>,
    is_correct: bool,
}

const BACK_GREEN: Colour = RGB(83, 141, 78);
const BACK_YELLOW: Colour = RGB(180, 159, 58);
const BACK_GRAY: Colour = RGB(58, 58, 60);

impl InputState {
    pub fn new(width: usize) -> Self {
        InputState {
            width,
            word: None,
            hint: vec![],
            is_correct: false,
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

    pub fn correct(&mut self) {
        self.is_correct = true;
    }

    pub fn colorized(&self) -> Result<String, &'static str> {
        if self.word.is_none() {
            return Result::Err("word are empty");
        }
        if self.is_correct {
            return Result::Ok(format!("{}", Style::new().on(BACK_GREEN).fg(White).bold().paint(self.word.as_ref().unwrap())));
        }
        if self.hint.is_empty() {
            return Result::Err("hints are empty");
        }
        let mut chars: Vec<String> = Vec::new();
        for (c, hint) in zip(self.word.as_ref().unwrap().chars(), &self.hint) {
            let res = match hint.spot {
                Spot::None() => format!("{}", Style::new().on(BACK_GRAY).fg(White).bold().paint(c.to_string())),
                Spot::InWithout(_) => format!("{}", Style::new().on(BACK_YELLOW).fg(White).bold().paint(c.to_string())),
                Spot::At(_) => format!("{}", Style::new().on(BACK_GREEN).fg(White).bold().paint(c.to_string())),
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

struct InputStates {
    states: Vec<InputState>,
}

impl InputStates {
    pub fn new() -> Self {
        InputStates { states: Vec::new() }
    }

    pub fn add(&mut self, state: InputState) {
        self.states.push(state)
    }

    pub fn preview(&self, staged_state: &InputState) -> Result<String, &'static str> {
        let mut results: Vec<String> = Vec::new();
        for state in &self.states {
            match state.colorized() {
                Ok(s) => results.push(s),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        match staged_state.colorized() {
            Ok(s) => results.push(s),
            Err(e) => {
                return Err(e);
            }
        }
        Ok(results.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod input_state {
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
}