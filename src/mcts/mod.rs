pub mod arena;
pub mod node;

use crate::state::State;
use arena::Arena;
use node::Node;

use rand::seq::SliceRandom;
use rayon::prelude::*;

use std::sync::Mutex;

pub struct Mcts<S: State> {
    pub arena: Mutex<Arena<S>>,
    pub root_id: usize,
    c: f64,
}

impl<S: State + std::fmt::Debug + std::clone::Clone> Mcts<S> {
    pub fn new(state: S, c: f64) -> Self {
        let mut arena: Arena<S> = Arena::new();
        let root: Node<S> = Node::new(state.clone(), S::default_action(), None);
        let root_id: usize = arena.add_node(root);
        Mcts {
            arena: Mutex::new(arena),
            root_id,
            c,
        }
    }

    pub fn search(&mut self, n: usize) -> S::Action {
        for _ in 0..n {
            let selected_id = self.select();

            // Lock only to check terminality
            {
                let arena = self.arena.lock().unwrap();
                let selected_node = arena.get_node(selected_id);
                if !selected_node.state.is_terminal() {
                    drop(arena);
                    self.expand(selected_id);
                    continue;
                }
            }

            // Simulate and backprop
            let state = {
                let arena = self.arena.lock().unwrap();
                arena.get_node(selected_id).state.clone()
            };
            let reward = self.simulate_from_state(state.clone(), state.to_play());
            self.backprop(selected_id, reward, 1);
        }

        let arena = self.arena.lock().unwrap();
        let root_node = arena.get_node(self.root_id);
        let best_child = root_node
            .children
            .iter()
            .max_by(|&a, &b| {
                let node_a_score = arena.get_node(*a).q;
                let node_b_score = arena.get_node(*b).q;
                node_a_score.partial_cmp(&node_b_score).unwrap()
            })
            .unwrap()
            .clone();
        arena.get_node(best_child).action.clone()
    }

    fn select(&mut self) -> usize {
        let mut current: usize = self.root_id;
        loop {
            let arena = self.arena.lock().unwrap();
            let node = arena.get_node(current);
            if node.is_leaf() || node.state.is_terminal() {
                return current;
            }
            let best_child = node.get_best_child(&arena, self.c);
            current = best_child;
        }
    }

    fn expand(&mut self, id: usize) {
        let children_info = {
            let mut arena = self.arena.lock().unwrap();
            let parent = arena.get_node_mut(id);

            let parent_state = parent.state.clone();
            let legal_actions = parent_state.get_legal_actions();

            // Create children nodes
            let mut children_info = Vec::new();
            for action in legal_actions {
                let child_state = parent_state.step(&action);
                let child_node = Node::new(child_state.clone(), action.clone(), Some(id));
                let child_id = arena.add_node(child_node);

                children_info.push((child_id, child_state));
            }
            children_info
        };

        // add children to parent
        let mut arena = self.arena.lock().unwrap();
        let parent = arena.get_node_mut(id);
        for (child_id, _) in &children_info {
            parent.children.push(*child_id);
        }
        drop(arena);

        // Step 2: Parallel simulations outside the lock
        let results: Vec<f64> = children_info
            .par_iter()
            .map(|(_, state)| self.simulate_from_state(state.clone(), state.to_play()))
            .collect();

        // Step 3: Aggregate results
        let total_reward: f64 = results.iter().sum();
        let total_visits: usize = results.len();

        // Step 4: Backprop once with aggregated results
        if total_visits > 0 {
            self.backprop(id, total_reward, total_visits);
        }
    }

    /// Simulate a rollout from a given state until terminal, without locking the arena.
    fn simulate_from_state(&self, mut state: S, to_play: usize) -> f64 {
        while !state.is_terminal() {
            let legal_actions = state.get_legal_actions();
            let action = legal_actions
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone();
            state = state.step(&action);
        }
        state.reward(to_play) as f64
    }

    fn backprop(&mut self, id: usize, mut reward: f64, total_n: usize) {
        let mut current = id;
        loop {
            let mut arena = self.arena.lock().unwrap();
            let node = arena.get_node_mut(current);
            node.reward_sum += reward;
            node.n += total_n;
            node.q = node.reward_sum / node.n as f64;
            if let Some(parent_id) = node.parent {
                current = parent_id;
            } else {
                break;
            }
            // Flip the reward for the parent
            reward = -reward;
        }
    }
}
