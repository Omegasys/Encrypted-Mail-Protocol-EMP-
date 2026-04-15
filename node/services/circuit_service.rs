use crate::onion::circuit::Circuit;
use crate::onion::directory::Directory;

pub struct CircuitService;

impl CircuitService {
    pub fn build_circuit(directory: &Directory, hops: usize) -> Option<Circuit> {
        let nodes = directory.get_random_nodes(hops);

        if nodes.len() < hops {
            return None;
        }

        Some(Circuit::new(nodes))
    }
}
