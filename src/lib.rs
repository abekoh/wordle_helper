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