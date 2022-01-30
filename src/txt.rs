use std::fs::{File};
use std::io;
use std::io::{BufRead, copy};
use std::env;
use std::path::Path;
use crate::Dictionary;

const DEFAULT_CACHE_DIR: &str = "wordle-solver";
const DEFAULT_FILENAME: &str = "words_alpha.txt";

fn default_dict_path() -> Box<Path> {
    let buf = match env::var("XDG_CACHE_HOME") {
        Ok(v) => Path::new(v.as_str()).join(DEFAULT_CACHE_DIR).join(DEFAULT_FILENAME),
        Err(_) => match env::var("HOME") {
            Ok(v) => {
                Path::new(v.as_str()).join(".config").join(DEFAULT_CACHE_DIR).join(DEFAULT_FILENAME)
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
    println!("download {} into {}", ENGLISH_WORDS_URL, path.to_str().unwrap());
    let resp = reqwest::blocking::get(ENGLISH_WORDS_URL)?;
    let mut dest = File::create(path)?;
    let content = resp.text()?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}

pub struct TxtDictionary {
    file: File,
}

impl TxtDictionary {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if path == "" {
            let default_path = default_dict_path();
            if !default_path.exists() {
                fetch_from_english_words(&default_path)?
            }
            let file = File::open(default_path)?;
            Ok(TxtDictionary { file })
        } else {
            let file = File::open(path)?;
            Ok(TxtDictionary { file })
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
