use crate::onion::node::Node;

pub struct Directory {
    pub nodes: Vec<Node>,
}

impl Directory {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn get_random_nodes(&self, count: usize) -> Vec<Node> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();

        let mut nodes = self.nodes.clone();
        nodes.shuffle(&mut rng);

        nodes.into_iter().take(count).collect()
    }
}
