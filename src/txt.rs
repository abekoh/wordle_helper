use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::Dictionary;

pub struct TxtDictionary {
    file: File,
}

impl TxtDictionary {
    pub fn new(path: &str) -> io::Result<Self> {
        let file = File::open(path.to_string())?;
        Ok(TxtDictionary { file })
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
