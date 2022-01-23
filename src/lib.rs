#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Spot {
    At(Vec<usize>),
    InWithout(Vec<usize>),
    None(),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hint {
    letter: char,
    spot: Spot,
}

impl Hint {
    pub fn new(letter: char, spot: Spot) -> Self {
        Hint { letter, spot }
    }
}

pub trait Resolver {
    fn add_hint(&mut self, hints: Vec<Hint>);
    fn remove_word(&mut self, word: &str);
    fn guess(&self) -> Vec<&String>;
}

pub struct SimpleResolver {
    width: i32,
    dict_words: Vec<String>,
    hints: Vec<Hint>,
}

impl SimpleResolver {
    pub fn new(width: i32, dict_words: Vec<&str>) -> SimpleResolver {
        SimpleResolver {
            width: width,
            dict_words: dict_words.iter()
                .map(|word| {
                    word.to_string()
                }).collect(),
            hints: vec![],
        }
    }
}

impl Resolver for SimpleResolver {
    fn add_hint(&mut self, hints: Vec<Hint>) {
        hints.iter().for_each(|hint| {
            self.hints.push(hint.clone());
        });
    }

    fn remove_word(&mut self, word: &str) {
        match self.dict_words.iter().position(|r| { r == word }) {
            Some(index) => {
                self.dict_words.swap_remove(index);
                ()
            }
            _ => (),
        }
    }

    fn guess(&self) -> Vec<&String> {
        self.dict_words.iter()
            .filter(|word| {
                for hint in &self.hints {
                    let res = match &hint.spot {
                        Spot::None() => {
                            !word.contains(hint.letter)
                        }
                        Spot::InWithout(without_spots) => {
                            if !word.contains(hint.letter) {
                                return false;
                            }
                            let match_spots = without_spots.iter()
                                .filter(|spot| {
                                    word.as_bytes()[*spot.clone()] as char != hint.letter
                                }).count();
                            return match_spots > 0;
                        }
                        Spot::At(at_spots) => {
                            for at_spot in at_spots {
                                if word.as_bytes()[at_spot.clone()] as char == hint.letter {
                                    return true;
                                }
                            }
                            return false;
                        }
                    };
                    if !res {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod new {
        use super::*;

        #[test]
        fn asset_fields() {
            let actual = SimpleResolver::new(5, vec!["hello", "early"]);
            assert_eq!(actual.width, 5);
            assert_eq!(actual.dict_words, vec!["hello", "early"]);
            assert_eq!(actual.hints.len(), 0);
        }
    }


    #[test]
    fn add_hint() {
        let mut actual = SimpleResolver::new(5, vec!["hello", "early"]);
        actual.add_hint(vec![Hint { letter: 'a', spot: Spot::None() }]);
        assert_eq!(actual.hints, vec![Hint { letter: 'a', spot: Spot::None() }]);
    }

    #[test]
    fn remove_word() {
        let mut actual = SimpleResolver::new(5, vec!["hello", "early"]);
        actual.remove_word("hello");
        assert_eq!(actual.dict_words, vec!["early"]);
    }

    #[cfg(test)]
    mod guess {
        use super::*;

        #[test]
        fn remove_including_a() {
            let mut actual = SimpleResolver::new(5, vec!["hello", "early", "asset"]);
            actual.add_hint(vec![Hint { letter: 'a', spot: Spot::None() }]);
            assert_eq!(actual.guess(), vec![&String::from("hello")]);
        }

        #[test]
        fn only_including_l() {
            let mut actual = SimpleResolver::new(5, vec!["hello", "early", "asset"]);
            actual.add_hint(vec![Hint { letter: 'l', spot: Spot::InWithout(vec![2]) }]);
            assert_eq!(actual.guess(), vec![&String::from("early")]);
        }

        #[test]
        fn at_t() {
            let mut actual = SimpleResolver::new(5, vec!["hello", "early", "asset"]);
            actual.add_hint(vec![Hint { letter: 't', spot: Spot::At(vec![4]) }]);
            assert_eq!(actual.guess(), vec![&String::from("asset")]);
        }
    }
}