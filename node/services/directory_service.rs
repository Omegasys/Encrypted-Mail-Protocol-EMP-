use crate::onion::node::Node;

pub struct DirectoryService {
    pub nodes: Vec<Node>,
}

impl DirectoryService {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn register_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn list_nodes(&self) -> &[Node] {
        &self.nodes
    }
}
