use crate::onion::node::Node;

pub struct Circuit {
    pub nodes: Vec<Node>,
}

impl Circuit {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { nodes }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_valid(&self) -> bool {
        self.nodes.len() >= 3
    }
}
