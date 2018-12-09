use std::io;
use std::io::Write;

struct SucksRocks {}

impl SucksRocks {
    fn new() -> SucksRocks {
        SucksRocks{}
    }

    fn score(&self, _: &str) -> f64 {
        return 0.0
    }
}

fn main() {
    let query = read_string("Enter a query: ");

    let sucks_rocks = SucksRocks::new();
    let score = sucks_rocks.score(&query);

    println!("The score for '{}' is {}!", query, score);
}

fn read_string(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read query");

    return String::from(buf.trim())
}
