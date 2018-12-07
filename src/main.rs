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

    let sucks_rocks = SucksRocks::new();
    let query = String::new();
    read_string("Enter a query: ", &mut query);
    let score = sucks_rocks.score(&query);

    println!("The score for '{}' is {}!", query, score);
}

fn read_string(prompt: &str, buf: &mut String) {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    io::stdin().read_line(buf).expect("Failed to read query");
    buf = &mut String::from(buf.trim())
}
