use std::iter::zip;

use ansi_term::{Colour, Style};
use ansi_term::Color::{RGB, White};
use ansi_term::Colour::Cyan;
use clap::Parser;
use dialoguer::{Confirm, FuzzySelect, Input, Select};
use dialoguer::theme::ColorfulTheme;
use num_format::{Locale, ToFormattedString};

use wordle_solver::{Dictionary, Hint, Solver, Spot};
use wordle_solver::simple::SimpleSolver;
use wordle_solver::txt::TxtDictionary;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Config {
    #[clap(short, long, default_value_t = 5, help = "length of one word")]
    word_length: usize,

    #[clap(short, long, default_value_t = 6, help = "how many you can answer")]
    answer_length: usize,

    #[clap(short, long, hide_default_value = true, default_value = "", help = "dictionary path")]
    dict_path: String,
}

fn main() {
    let config = Config::parse();

    println!("{}", Style::new().bold().paint("Welcome to WORDLE SOLVER"));

    let dictionary: Box<dyn Dictionary> = Box::new(match TxtDictionary::new(&config.dict_path) {
        Ok(d) => d,
        Err(e) => {
            println!();
            eprintln!("failed to load dictionary: {}", e);
            std::process::exit(1);
        }
    });
    println!("{}", Cyan.paint(format!("word length: {}", config.word_length)));
    println!("{}", Cyan.paint(format!("# of answer you can guess: {}", config.answer_length)));

    let mut solver: Box<dyn Solver> = Box::new(SimpleSolver::new(config.word_length, &dictionary.extract_words(config.word_length)));
    let mut states: InputStates = InputStates::new(config.word_length, config.answer_length);

    loop {
        let remained_words_length = solver.remained_words_length();
        if remained_words_length == 0 {
            println!();
            eprintln!("Remained words are empty, so I can't solve this. quit.");
            std::process::exit(1);
        }

        loop {
            println!();
            println!("There are {} words are remained.", remained_words_length.to_formatted_string(&Locale::en));

            let mut state = InputState::new(config.word_length);

            let guess_types = &[
                "Use suggestions",
                "Input manually",
            ];
            let selected_type_idx = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select guessing type")
                .default(0)
                .items(&guess_types[..])
                .interact()
                .unwrap();
            match selected_type_idx {
                0 => {
                    let suggested = solver.suggest();
                    let selected = FuzzySelect::with_theme(&ColorfulTheme::default())
                        .with_prompt("Guess")
                        .default(0)
                        .items(suggested)
                        .interact()
                        .unwrap();
                    state.add_word(&suggested[selected]).unwrap();
                }
                1 => {
                    let input: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Guess")
                        .validate_with({
                            move |input: &String| -> Result<(), &str> {
                                if input.len() != config.word_length {
                                    return Err("invalid length");
                                }
                                Ok(())
                            }
                        })
                        .interact_text()
                        .unwrap();
                    state.add_word(&input).unwrap();
                }
                _ => {
                    eprintln!("failed to recognize selection");
                    std::process::exit(1);
                }
            }

            println!("Input answer like this:");
            println!("{}", states.preview(&state).unwrap());

            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Is corrected?")
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
                if Hint::all_at(hints) {
                    state.correct();
                    println!("{}\n", Style::new().bold().paint("Wow, It's correct! Congrats!"));
                    println!("{}", states.preview(&state).unwrap());
                    std::process::exit(0);
                }
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

    pub fn is_draft(&self) -> bool {
        if self.is_correct {
            return false;
        }
        self.word.is_some() && self.hint.is_empty()
    }

    pub fn plain(&self) -> Result<String, &'static str> {
        if self.word.is_none() {
            return Result::Err("word are empty");
        }
        Result::Ok(format!("{}", Style::new().fg(White).bold().paint(self.word.as_ref().unwrap().to_uppercase())))
    }

    pub fn colorized(&self) -> Result<String, &'static str> {
        if self.word.is_none() {
            return Result::Err("word are empty");
        }
        if self.is_correct {
            return Result::Ok(format!("{}", Style::new().on(BACK_GREEN).fg(White).bold().paint(self.word.as_ref().unwrap().to_uppercase())));
        }
        if self.hint.is_empty() {
            return Result::Err("hints are empty");
        }
        let mut chars: Vec<String> = Vec::new();
        for (c, hint) in zip(self.word.as_ref().unwrap().chars(), &self.hint) {
            let res = match hint.spot {
                Spot::None() => format!("{}", Style::new().on(BACK_GRAY).fg(White).bold().paint(c.to_string().to_uppercase())),
                Spot::InWithout(_) => format!("{}", Style::new().on(BACK_YELLOW).fg(White).bold().paint(c.to_string().to_uppercase())),
                Spot::At(_) => format!("{}", Style::new().on(BACK_GREEN).fg(White).bold().paint(c.to_string().to_uppercase())),
            };
            chars.push(res);
        }
        Result::Ok(chars.join(""))
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
    word_width: usize,
    answer_width: usize,
    states: Vec<InputState>,
}

impl InputStates {
    pub fn new(word_width: usize, answer_width: usize) -> Self {
        InputStates { states: Vec::new(), word_width, answer_width }
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

        let staged_text = match staged_state.is_draft() {
            true => staged_state.plain(),
            false => staged_state.colorized(),
        };
        match staged_text {
            Ok(s) => results.push(s),
            Err(e) => {
                return Err(e);
            }
        }
        Ok(InputStates::pretty_preview(self, &results))
    }

    fn pretty_preview(&self, word_strs: &[String]) -> String {
        let header_footer: String = format!("+{}+", "-".repeat(self.word_width));

        let mut results: Vec<String> = Vec::new();
        results.push(header_footer.clone());
        for i in 0..self.answer_width {
            if i < word_strs.len() {
                results.push(format!("|{}|", word_strs[i]));
            } else {
                results.push(format!("|{}|", " ".repeat(self.word_width)));
            }
        }
        results.push(header_footer);
        results.join("\n")
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
                assert!(actual.is_err());
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
                    assert!(actual.is_err());
                }
            }

            #[test]
            fn invalid_no_word() {
                let mut state = InputState::new(5);
                let actual = state.add_hint("00120");
                assert!(actual.is_err());
            }
        }
    }

    #[cfg(test)]
    mod input_states {
        use super::*;

        #[test]
        fn pretty_preview() {
            let target = InputStates::new(5, 6);
            let actual = target.pretty_preview(&["ABCDE".to_string(),
                "FGHIJ".to_string(),
                "KLMNO".to_string()]);
            assert_eq!(actual, r#"+-----+
|ABCDE|
|FGHIJ|
|KLMNO|
|     |
|     |
|     |
+-----+"#)
        }
    }
}