use crate::mcts::node::Node;
use crate::state::State;

pub struct Arena<S: State> {
    pub nodes: Vec<Node<S>>,
}

impl<S: State> Arena<S> {
    pub fn new() -> Self {
        Arena { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: Node<S>) -> usize {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }

    pub fn get_node_mut(&mut self, id: usize) -> &mut Node<S> {
        &mut self.nodes[id]
    }

    pub fn get_node(&self, id: usize) -> &Node<S> {
        &self.nodes[id]
    }
}
