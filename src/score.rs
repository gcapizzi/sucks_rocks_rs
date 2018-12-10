pub struct DummyScorer {}

impl DummyScorer {
    pub fn new() -> DummyScorer {
        DummyScorer{}
    }

    pub fn calculate(&self, _: &str) -> f64 {
        return 0.0
    }
}

