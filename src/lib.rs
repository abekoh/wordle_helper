#[derive(Clone)]
enum Spot {
    At(i32),
    InWithout(Vec<i32>),
    None(),
}

#[derive(Clone)]
struct Hint {
    letter: char,
    spot: Spot,
}

trait Resolver {
    fn add_hint(&mut self, results: &[Hint]);
    fn guess(&self) -> Vec<String>;
}

struct SimpleResolver {
    dict_words: Vec<String>,
    hints: Vec<Hint>,
}

impl SimpleResolver {
    fn new(dict_words: Vec<&str>) -> SimpleResolver {
        SimpleResolver {
            dict_words: dict_words.iter()
                .map(|word| {
                    word.to_string()
                }).collect(),
            hints: vec![],
        }
    }
}

impl Resolver for SimpleResolver {
    fn add_hint(&mut self, results: &[Hint]) {
        results.iter().for_each(|result| {
            self.hints.push(result.clone());
        })
    }

    fn guess(&self) -> Vec<String> {
        todo!()
    }
}

mod tests {
    use super::*;

    #[test]
    fn new() {
        let actual = SimpleResolver::new(vec!["hello", "early"]);
        assert_eq!(actual.dict_words, vec!["hello", "early"]);
        assert_eq!(actual.hints.len(), 0);
    }
}