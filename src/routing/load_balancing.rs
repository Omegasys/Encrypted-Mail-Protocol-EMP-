use crate::onion::node::Node;
use std::collections::HashMap;

pub struct LoadBalancer {
    load_map: HashMap<String, usize>,
}

impl LoadBalancer {
    pub fn new() -> Self {
        Self {
            load_map: HashMap::new(),
        }
    }

    pub fn record_usage(&mut self, node: &Node) {
        *self.load_map.entry(node.id.clone()).or_insert(0) += 1;
    }

    pub fn least_loaded<'a>(&self, nodes: &'a [Node]) -> Option<&'a Node> {
        nodes.iter().min_by_key(|n| self.load_map.get(&n.id).unwrap_or(&0))
    }
}
