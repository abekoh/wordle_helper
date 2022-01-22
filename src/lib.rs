enum Spot {
    At(i32),
    InWithout(Vec<i32>),
    None(),
}

struct Hint {
    letter: char,
    spot: Spot,
}

trait Resolver {
    fn add_hint(results: Vec<Hint>);
    fn guess() -> Vec<String>;
}

struct SimpleResolver {
    dict_words: Vec<String>;
}

impl SimpleResolver {
    fn new(dict_words: Vec<&str>) -> SimpleResolver {
        SimpleResolver {
            dict_words: dict_words.iter()
                .map(|word| {
                    word.to_string()
                }).collect()
        }
    }
}

impl Resolver for SimpleResolver {
    fn add_hint(results: Vec<Hint>) {
        todo!()
    }

    fn guess() -> Vec<String> {
        todo!()
    }
}