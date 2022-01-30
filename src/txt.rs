use std::fs::File;
use std::io;
use std::io::BufRead;
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

pub struct TxtDictionary {
    file: File,
}

impl TxtDictionary {
    pub fn new(path: &str) -> io::Result<Self> {
        if path == "" {
            let file = File::open(default_dict_path())?;
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
