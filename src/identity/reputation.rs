use std::collections::HashMap;

pub struct ReputationStore {
    scores: HashMap<String, i32>,
}

impl ReputationStore {
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
        }
    }

    pub fn increase(&mut self, id: &str) {
        *self.scores.entry(id.to_string()).or_insert(0) += 1;
    }

    pub fn decrease(&mut self, id: &str) {
        *self.scores.entry(id.to_string()).or_insert(0) -= 1;
    }

    pub fn get(&self, id: &str) -> i32 {
        *self.scores.get(id).unwrap_or(&0)
    }
}
