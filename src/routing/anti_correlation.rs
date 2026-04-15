use crate::onion::node::Node;
use rand::seq::SliceRandom;
use std::collections::HashSet;

pub struct AntiCorrelation;

impl AntiCorrelation {
    pub fn filter_unique(nodes: &[Node]) -> Vec<Node> {
        let mut seen = HashSet::new();

        nodes.iter()
            .filter(|n| seen.insert(n.address.clone()))
            .cloned()
            .collect()
    }

    pub fn randomize_timing(base_delay_ms: u64) -> u64 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        base_delay_ms + rng.gen_range(0..base_delay_ms)
    }

    pub fn diversify_path(nodes: &[Node], hops: usize) -> Vec<Node> {
        let filtered = Self::filter_unique(nodes);

        let mut rng = rand::thread_rng();
        let mut pool = filtered.clone();

        pool.shuffle(&mut rng);

        pool.into_iter().take(hops).collect()
    }
}
