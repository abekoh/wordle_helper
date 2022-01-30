use std::iter::zip;

use ansi_term::{ANSIGenericString, Colour, Style};
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

    #[clap(short, long, default_value_t = 6, help = "number of answer you can guess")]
    max_guess_count: usize,

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
    println!("{}", Cyan.paint(format!("number of answer you can guess: {}", config.max_guess_count)));

    let mut solver: Box<dyn Solver> = Box::new(SimpleSolver::new(config.word_length, &dictionary.extract_words(config.word_length)));
    let mut states: InputStates = InputStates::new(config.word_length, config.max_guess_count);

    loop {
        let remained_words_length = solver.remained_words_length();
        if remained_words_length == 0 {
            println!();
            eprintln!("Sorry, there are no matched words. quit.");
            std::process::exit(1);
        }

        loop {
            println!();
            println!("{}", Style::new().bold().paint(format!("ROUND {}/{}", states.round_count + 1, config.max_guess_count)));
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
                .with_prompt(format!("Is \"{}\" corrected answer?", state.word().unwrap().to_uppercase()))
                .interact()
                .unwrap()
            {
                state.correct();
                println!("{}", Style::new().bold().paint("Congratulation!!"));
                println!("{}", states.preview(&state).unwrap());
                std::process::exit(0);
            }

            if states.is_final_round() {
                println!("{}", Style::new().bold().paint(format!("X/{} GAME OVER!!", config.max_guess_count)));
                println!("{}", states.preview(&state).unwrap());
                std::process::exit(1);
            }
            states.increment_round();

            loop {
                println!(r#"Input {} numbers in order as hint;
· nowhere   -> {}
· somewhere -> {}
· just      -> {}"#,
                         config.word_length,
                         colorize(&HintInputType::Nowhere, "0"),
                         colorize(&HintInputType::Somewhere, "1"),
                         colorize(&HintInputType::Just, "2"),
                );
                if states.round_count == 1 {
                    println!("{}{}{}{}{}{}",
                             Style::new().fg(RGB(128, 128, 128)).paint("e.g.) SOLVE + 10221 => "),
                             colorize(&HintInputType::Somewhere, "S"),
                             colorize(&HintInputType::Nowhere, "O"),
                             colorize(&HintInputType::Just, "L"),
                             colorize(&HintInputType::Just, "V"),
                             colorize(&HintInputType::Somewhere, "E"),
                    );
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
                    .with_prompt("Is this hint OK?")
                    .default(true)
                    .interact()
                    .unwrap()
                {
                    let (word, hints) = state.get().unwrap();
                    if Hint::all_at(hints) {
                        state.correct();
                        println!("{}", Style::new().bold().paint("Wow, It's correct! Congrats!"));
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
}

const BACK_GREEN: Colour = RGB(83, 141, 78);
const BACK_YELLOW: Colour = RGB(180, 159, 58);
const BACK_GRAY: Colour = RGB(58, 58, 60);

enum HintInputType {
    Nowhere,
    Somewhere,
    Just,
}

fn colorize<'a>(hint_type: &'a HintInputType, text: &'a str) -> ANSIGenericString<'a, str> {
    return match hint_type {
        HintInputType::Nowhere => Style::new().on(BACK_GRAY).fg(White).bold().paint(text.to_string()),
        HintInputType::Somewhere => Style::new().on(BACK_YELLOW).fg(White).bold().paint(text.to_string()),
        HintInputType::Just => Style::new().on(BACK_GREEN).fg(White).bold().paint(text.to_string()),
    };
}

struct InputState {
    word_length: usize,
    word: Option<String>,
    hint: Vec<Hint>,
    is_correct: bool,
}

impl InputState {
    pub fn new(word_length: usize) -> Self {
        InputState {
            word_length,
            word: None,
            hint: vec![],
            is_correct: false,
        }
    }

    pub fn add_word(&mut self, input: &str) -> Result<(), &'static str> {
        if input.trim().len() != self.word_length {
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
        if trimmed.len() != self.word_length {
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

    pub fn word(&self) -> Option<&String> {
        match &self.word {
            Some(w) => Some(w),
            None => None,
        }
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
            return Result::Ok(format!("{}", colorize(&HintInputType::Just, &self.word.as_ref().unwrap().to_uppercase())));
        }
        if self.hint.is_empty() {
            return Result::Err("hints are empty");
        }
        let mut chars: Vec<String> = Vec::new();
        for (c, hint) in zip(self.word.as_ref().unwrap().chars(), &self.hint) {
            let res = match hint.spot {
                Spot::None() => format!("{}", colorize(&HintInputType::Nowhere, &c.to_string().to_uppercase())),
                Spot::InWithout(_) => format!("{}", colorize(&HintInputType::Somewhere, &c.to_string().to_uppercase())),
                Spot::At(_) => format!("{}", colorize(&HintInputType::Just, &c.to_string().to_uppercase())),
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
    word_length: usize,
    max_guess_count: usize,
    states: Vec<InputState>,
    pub round_count: i32,
}

impl InputStates {
    pub fn new(word_length: usize, max_guess_count: usize) -> Self {
        InputStates { states: Vec::new(), word_length, max_guess_count, round_count: 0 }
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
        let header_footer: String = format!("+{}+", "-".repeat(self.word_length));
        let mut results: Vec<String> = vec![header_footer.clone()];
        for i in 0..self.max_guess_count {
            if i < word_strs.len() {
                results.push(format!("|{}|", word_strs[i]));
            } else {
                results.push(format!("|{}|", " ".repeat(self.word_length)));
            }
        }
        results.push(header_footer);
        results.join("\n")
    }

    fn increment_round(&mut self) {
        self.round_count += 1;
    }

    fn is_final_round(&self) -> bool {
        (self.round_count + 1) == self.max_guess_count as i32
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
            assert_eq!(actual.word_length, 5);
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