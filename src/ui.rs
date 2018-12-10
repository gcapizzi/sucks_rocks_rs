use std::io;
use std::io::Write;

pub struct TerminalUI {}

impl TerminalUI {
    pub fn new() -> TerminalUI {
        TerminalUI{}
    }

    pub fn read_query(&self) -> String {
        self.print_string("Enter a query: ");
        return self.read_string()
    }

    pub fn show_score(&self, score: f64) {
        println!("The score is {}!", score);
    }

    fn print_string(&self, string: &str) {
        print!("{}", string);
        let _ = io::stdout().flush();
    }

    fn read_string(&self) -> String {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Failed to read string");
        return String::from(buf.trim())
    }
}
