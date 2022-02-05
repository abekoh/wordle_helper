use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, copy};
use std::path::Path;

use ansi_term::Color::Yellow;
use ansi_term::Style;
use dialoguer::Confirm;
use dialoguer::theme::ColorfulTheme;

use crate::Dictionary;

const DEFAULT_CACHE_DIR: &str = "wordle-helper";
const DEFAULT_FILENAME: &str = "words_alpha.txt";

fn default_dict_path() -> Box<Path> {
    let buf = match env::var("XDG_CACHE_HOME") {
        Ok(v) => Path::new(v.as_str()).join(DEFAULT_CACHE_DIR).join(DEFAULT_FILENAME),
        Err(_) => match env::var("HOME") {
            Ok(v) => {
                Path::new(v.as_str()).join(".cache").join(DEFAULT_CACHE_DIR).join(DEFAULT_FILENAME)
            }
            Err(_) => {
                Path::new("/tmp").join(DEFAULT_CACHE_DIR).join(DEFAULT_FILENAME)
            }
        }
    };
    buf.into_boxed_path()
}

const ENGLISH_WORDS_URL: &str = "https://raw.githubusercontent.com/dwyl/english-words/master/words_alpha.txt";

fn fetch_from_english_words(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}",
             Style::new().fg(Yellow).paint(
                 format!("Downloading {}", ENGLISH_WORDS_URL)
             )
    );
    fs::create_dir_all(path.parent().unwrap())?;
    let resp = reqwest::blocking::get(ENGLISH_WORDS_URL)?;
    let mut dest = File::create(path)?;
    let content = resp.text()?;
    copy(&mut content.as_bytes(), &mut dest)?;
    println!("{}",
             Style::new().fg(Yellow).paint(
                 "Complete!"
             )
    );
    Ok(())
}

pub struct TxtDictionary {
    file: File,
}

impl TxtDictionary {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if path.is_empty() {
            let default_path = default_dict_path();
            if !default_path.exists() {
                println!();
                println!("{}",
                         Style::new().fg(Yellow).paint(
                             format!("Default dictionary is not found at {}", default_path.to_str().unwrap())
                         )
                );
                println!("{}",
                         Style::new().fg(Yellow).paint(
                             "So I should download dictionary from https://github.com/dwyl/english-words (about 4.04MB)"
                         )
                );
                if Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Do you want to continue?")
                    .interact()
                    .unwrap()
                {
                    fetch_from_english_words(&default_path)?
                }
            }
            let file = File::open(default_path)?;
            Ok(TxtDictionary { file })
        } else {
            let file = File::open(path)?;
            Ok(TxtDictionary { file })
        }
    }

    fn new_for_debug(path: &str) -> Self {
        Self {
            file: File::open(path).unwrap()
        }
    }
}

impl Dictionary for TxtDictionary {
    fn extract_words(&self, word_length: usize) -> Vec<String> {
        let dict: Vec<String> = io::BufReader::new(&self.file)
            .lines()
            .filter_map(|e| {
                e.ok()
            })
            .filter(|w| {
                w.len() == word_length
            })
            .map(|line| {
                String::from(line.trim())
            })
            .collect();
        dict
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_words() {
        let target = TxtDictionary::new_for_debug("src/testdata/english.txt");
        let actual = target.extract_words(5);
        assert_eq!(actual, vec![
            String::from("apple"),
            String::from("early"),
            String::from("asset"),
        ])
    }
}