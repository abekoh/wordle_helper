use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::Dictionary;

pub struct TxtDictionary {
    dict_path: String,
}

impl TxtDictionary {
    pub fn new(path: &str) -> Self {
        TxtDictionary { dict_path: path.to_string() }
    }
}

impl Dictionary for TxtDictionary {
    fn extract_words(&self, word_length: usize) -> Result<Vec<String>, &str> {
        let file = match File::open(self.dict_path.to_string()) {
            Ok(v) => v,
            Err(_) => {
                return Err("failed to open file");
            }
        };
        let dict: Vec<String> = io::BufReader::new(file)
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
        Ok(dict)
    }
}
