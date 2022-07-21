use std::collections::VecDeque;

use crate::jugs::State;

#[derive(Clone, Debug)]
pub struct Node {
    state: State,
    parent: Option<Box<Node>>,
}

impl Node {
    pub fn new(state: State, parent: Node) -> Self {
        Self {
            state,
            parent: Some(Box::new(parent)),
        }
    }

    pub fn back_trace(self) -> Vec<State> {
        let mut states = Vec::new();
        let mut node = self;
        while let Some(next_node) = node.parent {
            states.push(node.state);
            node = *next_node;
        }
        states.push(node.state);
        states.reverse();
        states
    }
}

impl From<State> for Node {
    fn from(state: State) -> Self {
        Self {
            state,
            parent: None,
        }
    }
}

pub trait Search {
    fn search(&mut self, initial_state: State) -> Option<Node>;
}

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
