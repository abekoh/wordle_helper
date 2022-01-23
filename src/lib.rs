#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Spot {
    At(usize),
    InWithout(usize),
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
    fn guess(&self) -> &Vec<String>;
    fn add_hint(&mut self, word: &str, hints: &Vec<Hint>);
    fn remining_words_length(&self) -> usize;
}

#[derive(Debug)]
pub struct SimpleResolver {
    width: i32,
    dict_words: Vec<String>,
}

impl SimpleResolver {
    pub fn new(width: i32, dict_words: &Vec<String>) -> SimpleResolver {
        SimpleResolver {
            width,
            dict_words: dict_words.iter()
                .filter(|word| {
                    word.len() == width as usize
                })
                .map(|word| {
                    word.to_string()
                }).collect(),
        }
    }

    fn update_with_hints(&mut self, hints: &Vec<Hint>) {
        self.dict_words = self.dict_words.iter()
            .filter(|word| {
                for hint in hints {
                    let res = match &hint.spot {
                        Spot::None() => {
                            !word.contains(hint.letter)
                        }
                        Spot::InWithout(spot) => {
                            if !word.contains(hint.letter) {
                                return false;
                            }
                            word.as_bytes()[*spot as usize] as char != hint.letter
                        }
                        Spot::At(at_spot) => {
                            word.as_bytes()[at_spot.clone()] as char == hint.letter
                        }
                    };
                    if !res {
                        return false;
                    }
                }
                true
            })
            .map(|w| {
                w.clone()
            })
            .collect();
    }

    fn remove_word(&mut self, word: &str) {
        if word.len() != self.width as usize {
            return ();
        }
        match self.dict_words.iter().position(|r| { r == word }) {
            Some(index) => {
                self.dict_words.swap_remove(index);
                ()
            }
            _ => (),
        }
    }
}

impl Resolver for SimpleResolver {
    fn guess(&self) -> &Vec<String> {
        &self.dict_words
    }

    fn add_hint(&mut self, word: &str, hints: &Vec<Hint>) {
        self.remove_word(word);
        self.update_with_hints(hints);
    }

    fn remining_words_length(&self) -> usize {
        self.dict_words.len()
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
            let actual = SimpleResolver::new(5, &vec!["hello".to_string(), "early".to_string()]);
            assert_eq!(actual.width, 5);
            assert_eq!(actual.dict_words, vec!["hello", "early"]);
        }

        #[test]
        fn filter_word_only_length_is_5() {
            let actual = SimpleResolver::new(5, &vec!["hello".to_string(), "dog".to_string(), "in".to_string(), "early".to_string(), "difference".to_string()]);
            assert_eq!(actual.dict_words, vec!["hello", "early"]);
        }
    }

    #[cfg(test)]
    mod guess {
        use super::*;

        fn preset_words() -> Vec<String> {
            vec!["hello".to_string(), "early".to_string(), "asset".to_string()]
        }


        #[test]
        fn remove_word() {
            let mut actual = SimpleResolver::new(5, &preset_words());
            actual.add_hint("hello", &vec![]);
            assert_eq!(actual.guess(), &vec![String::from("asset"), String::from("early")]);
        }

        #[cfg(test)]
        mod remove_including {
            use super::*;

            #[test]
            fn a() {
                let mut actual = SimpleResolver::new(5, &preset_words());
                actual.add_hint("dummy", &vec![Hint { letter: 'a', spot: Spot::None() }]);
                assert_eq!(actual.guess(), &vec![String::from("hello")]);
            }

            #[test]
            fn l() {
                let mut actual = SimpleResolver::new(5, &preset_words());
                actual.add_hint("dummy", &vec![Hint { letter: 'l', spot: Spot::None() }]);
                assert_eq!(actual.guess(), &vec![String::from("asset")]);
            }
        }

        #[cfg(test)]
        mod only_including {
            use super::*;

            #[test]
            fn l_2() {
                let mut actual = SimpleResolver::new(5, &preset_words());
                actual.add_hint("dummy", &vec![Hint { letter: 'l', spot: Spot::InWithout(2) }]);
                assert_eq!(actual.guess(), &vec![String::from("early")]);
            }

            #[test]
            fn e_0() {
                let mut actual = SimpleResolver::new(5, &preset_words());
                actual.add_hint("dummy", &vec![Hint { letter: 'e', spot: Spot::InWithout(0) }]);
                assert_eq!(actual.guess(), &vec![String::from("hello"), String::from("asset")]);
            }
        }


        #[test]
        fn at_t() {
            let mut actual = SimpleResolver::new(5, &preset_words());
            actual.add_hint("dummy", &vec![Hint { letter: 't', spot: Spot::At(4) }]);
            assert_eq!(actual.guess(), &vec![String::from("asset")]);
        }
    }
}