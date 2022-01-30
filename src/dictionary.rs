use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn get_words(dict_path: &str) -> Vec<String> {
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
