use crate::mcts::arena::Arena;
use crate::state::State;

#[derive(Debug)]
pub struct Node<S: State> {
    pub state: S,
    pub action: S::Action,
    pub parent: Option<usize>,
    pub reward_sum: f64,
    pub n: usize, // number of visits
    pub q: f64,   // average reward
    pub children: Vec<usize>,
}

impl<S: State> Node<S> {
    pub fn new(state: S, action: S::Action, parent: Option<usize>) -> Self {
        Node {
            state,
            action,
            parent,
            reward_sum: 0.0,
            n: 0,
            q: 0.0,
            children: Vec::new(),
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn ucb(&self, arena: &Arena<S>, c: f64) -> f64 {
        if self.n == 0 {
            return f64::INFINITY;
        }
        let parent_n = arena.get_node(self.parent.unwrap()).n as f64;
        self.q + c * (parent_n.ln() / self.n as f64).sqrt()
    }

    pub fn get_best_child(&self, arena: &Arena<S>, c: f64) -> usize {
        if self.is_leaf() {
            panic!("get_best_child called on leaf node");
        }
        let best_child = self
            .children
            .iter()
            .max_by(|&a, &b| {
                let ucb_a = arena.get_node(*a).ucb(arena, c);
                let ucb_b = arena.get_node(*b).ucb(arena, c);
                ucb_a.partial_cmp(&ucb_b).unwrap()
            })
            .unwrap();
        *best_child
    }
}
