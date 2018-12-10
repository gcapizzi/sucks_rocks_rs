use std::io;
use std::io::Write;

mod score;

fn main() {
    let query = prompt_and_read("Enter a query: ");

    let scorer = score::DummyScorer::new();
    let score = scorer.calculate(&query);

    println!("The score for '{}' is {}!", query, score);
}

fn prompt_and_read(prompt: &str) -> String {
    print_string(prompt);
    return read_string()
}

fn print_string(string: &str) {
    print!("{}", string);
    let _ = io::stdout().flush();
}

fn read_string() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read string");
    return String::from(buf.trim())
}
