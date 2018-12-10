mod score;
mod ui;

fn main() {
    let ui = ui::TerminalUI::new();
    let scorer = score::DummyScorer::new();

    let query = ui.read_query();
    let score = scorer.calculate(&query);
    ui.show_score(score)
}
