pub mod simple;
pub mod txt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Spot {
    At(usize),
    InWithout(usize),
    None(),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hint {
    pub letter: char,
    pub spot: Spot,
}

impl Hint {
    pub fn new(letter: char, spot: Spot) -> Self {
        Hint { letter, spot }
    }
}

pub trait Solver {
    fn suggest(&self) -> &Vec<String>;
    fn add_hint(&mut self, word: &str, hints: &[Hint]);
    fn remained_words_length(&self) -> usize;
}

pub trait Dictionary {
    fn extract_words(&self, word_length: usize) -> Result<Vec<String>, &str>;
}
