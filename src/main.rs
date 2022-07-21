pub mod jugs;
pub mod search;

use jugs::State;
use search::{BreadthFirstSearch, Search};

fn main() {
    let initial_state = State::Jugs(0, 0);
    let mut search = BreadthFirstSearch::default();
    let final_node = search.search(initial_state).unwrap();
    println!("{:?}", final_node.back_trace());
}
