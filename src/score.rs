use app::Scorer;

pub struct DummyScorer {}

impl DummyScorer {
    pub fn new() -> DummyScorer {
        DummyScorer{}
    }
}

impl Scorer for DummyScorer {
    fn calculate(&self, _: &str) -> f64 {
        return 0.0
    }
}

