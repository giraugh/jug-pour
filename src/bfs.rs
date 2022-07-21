use crate::jugs::State;
use crate::search::{Node, Search};
use std::collections::VecDeque;

#[derive(Default)]
pub struct BreadthFirstSearch {
    open: VecDeque<Node>,
    closed: Vec<State>,
}

impl Search for BreadthFirstSearch {
    fn search(&mut self, initial_state: State) -> Option<Node> {
        // Add initial state to open set
        self.open.push_back(Node::from(initial_state));

        // Search...
        while !self.open.is_empty() {
            let node = self.open.pop_front().expect("Already checked size");
            for child in node.state.expand() {
                if child.is_goal() {
                    return Some(Node::new(child, node));
                }
                if !self.closed.contains(&child) {
                    self.closed.push(child);
                    self.open.push_back(Node::new(child, node.clone()));
                }
            }
        }

        None
    }
}
