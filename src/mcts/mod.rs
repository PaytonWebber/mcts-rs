pub mod arena;
pub mod node;

use crate::state::State;
use arena::Arena;
use node::Node;

use rand::seq::SliceRandom;

pub struct Mcts<S: State> {
    pub arena: Arena<S>,
    pub root_id: usize,
    c: f64,
}

impl<S: State + std::fmt::Debug + std::clone::Clone> Mcts<S> {
    pub fn new(state: S, c: f64) -> Self {
        let mut arena: Arena<S> = Arena::new();
        let root: Node<S> = Node::new(state.clone(), S::default_action(), None);
        let root_id: usize = arena.add_node(root);
        Mcts { arena, root_id, c }
    }

    pub fn search(&mut self, n: usize) -> S::Action {
        for _ in 0..n {
            let mut selected_id: usize = self.select();
            let selected_node: &Node<S> = self.arena.get_node(selected_id);
            if !selected_node.state.is_terminal() {
                self.expand(selected_id);
                let children: &Vec<usize> = &self.arena.get_node(selected_id).children;
                let random_child: usize = children.choose(&mut rand::thread_rng()).unwrap().clone();
                selected_id = random_child;
            }
            let reward: f64 = self.simulate(selected_id);
            self.backprop(selected_id, reward);
        }
        let root_node: &Node<S> = self.arena.get_node(self.root_id);
        let best_child: usize = root_node
            .children
            .iter()
            .max_by(|&a, &b| {
                let node_a_score = self.arena.get_node(*a).q;
                let node_b_score = self.arena.get_node(*b).q;
                node_a_score.partial_cmp(&node_b_score).unwrap()
            })
            .unwrap()
            .clone();

        let best_action: S::Action = self.arena.get_node(best_child).action;
        best_action
    }

    fn select(&mut self) -> usize {
        let mut current: usize = 0;
        loop {
            let node = &self.arena.get_node(current);
            if node.is_leaf() || node.state.is_terminal() {
                return current;
            }
            let best_child = node.get_best_child(&self.arena, self.c);
            current = best_child;
        }
    }

    fn expand(&mut self, id: usize) {
        let parent: &Node<S> = self.arena.get_node_mut(id);
        let legal_actions: Vec<S::Action> = parent.state.get_legal_actions();
        let parent_state: S = parent.state.clone();
        for action in legal_actions {
            let state = parent_state.step(action);
            let new_node = Node::new(state, action, Some(id));
            let new_id = self.arena.add_node(new_node);
            self.arena.get_node_mut(id).children.push(new_id);
        }
    }

    fn simulate(&self, id: usize) -> f64 {
        let node: &Node<S> = self.arena.get_node(id);
        let mut state: S = node.state.clone();
        while !state.is_terminal() {
            let legal_actions = state.get_legal_actions();
            let action = legal_actions
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone();
            state = state.step(action);
        }
        let reward: f64 = state.reward(node.state.to_play()) as f64;
        reward
    }

    fn backprop(&mut self, id: usize, mut reward: f64) {
        let mut current: usize = id;
        loop {
            let node = self.arena.get_node_mut(current);
            node.reward_sum += reward;
            node.n += 1;
            node.q = node.reward_sum / node.n as f64;
            if let Some(parent_id) = node.parent {
                current = parent_id;
            } else {
                break;
            }
            reward = -reward;
        }
    }
}
