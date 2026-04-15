use crate::onion::node::Node;
use rand::seq::SliceRandom;

pub struct PathSelector;

impl PathSelector {
    pub fn select_path(nodes: &[Node], hops: usize) -> Vec<Node> {
        let mut rng = rand::thread_rng();
        let mut pool = nodes.to_vec();

        pool.shuffle(&mut rng);

        pool.into_iter().take(hops).collect()
    }
}
