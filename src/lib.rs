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
    pub fn all_at(hints: &[Hint]) -> bool {
        hints.iter()
            .filter(|h| {
                matches!(h.spot, Spot::At(_))
            })
            .count() == hints.len()
    }
}

pub trait Helper {
    fn suggest(&self) -> &Vec<String>;
    fn add_hint(&mut self, word: &str, hints: &[Hint]);
    fn remained_words_length(&self) -> usize;
}

pub trait Dictionary {
    fn extract_words(&self, word_length: usize) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_at_true() {
        assert!(Hint::all_at(&[
            Hint::new('e', Spot::At(0)),
            Hint::new('a', Spot::At(1)),
            Hint::new('r', Spot::At(2)),
            Hint::new('l', Spot::At(3)),
            Hint::new('y', Spot::At(4)),
        ]))
    }

    #[test]
    fn all_at_false() {
        assert!(!Hint::all_at(&[
            Hint::new('e', Spot::At(0)),
            Hint::new('a', Spot::At(1)),
            Hint::new('r', Spot::At(2)),
            Hint::new('l', Spot::At(3)),
            Hint::new('y', Spot::None()),
        ]))
    }
}