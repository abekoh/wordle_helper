use wordle_resolver::{Hint, Resolver, SimpleResolver, Spot};

fn main() {
    let mut resolver = SimpleResolver::new(vec!["story", "hello"]);
    resolver.add_hint(&vec![Hint::new('a', Spot::None())]);
    resolver.add_hint(&vec![Hint::new('t', Spot::At(vec![1]))]);
    for guessed in resolver.guess() {
        println!("{}", guessed);
    }
}
