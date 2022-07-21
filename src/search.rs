use crate::jugs::State;

#[derive(Clone, Debug)]
pub struct Node {
    pub state: State,
    pub parent: Option<Box<Node>>,
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
