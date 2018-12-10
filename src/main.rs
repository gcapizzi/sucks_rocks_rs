mod score;
mod ui;
mod app;

fn main() {
    let scorer = score::DummyScorer::new();
    let ui = ui::TerminalUI::new();

    app::App::new(scorer, ui).run()
}
