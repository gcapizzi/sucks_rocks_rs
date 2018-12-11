pub trait Scorer {
    fn calculate(&self, &str) -> f64;
}

pub trait UI {
    fn read_query(&self) -> String;
    fn show_score(&self, f64);
}

pub struct App<S, U> {
    scorer: S,
    ui: U,
}

impl<S: Scorer, U: UI> App<S, U> {
    pub fn new(scorer: S, ui: U) -> App<S, U> {
        App { scorer, ui }
    }

    pub fn run(&self) {
        let query = self.ui.read_query();
        let score = self.scorer.calculate(&query);
        self.ui.show_score(score)
    }
}
