use std::cmp::min;

const JUG_SIZES: (usize, usize) = (3, 5);

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum State {
    Jugs(usize, usize),
}

#[derive(PartialEq)]
pub enum Side {
    L,
    R,
}
use Side::{L, R};

pub enum Action {
    Fill(Side),
    Empty(Side),
    Pour(Side, Side),
}

impl State {
    pub fn expand(&self) -> Vec<State> {
        self.possible_actions()
            .into_iter()
            .map(|action| self.apply_action(&action))
            .collect()
    }

    // Apply an action from a given state to get the next state
    pub fn apply_action(&self, action: &Action) -> State {
        match *self {
            State::Jugs(left, right) => match action {
                Action::Empty(L) => State::Jugs(0, right),
                Action::Empty(R) => State::Jugs(left, 0),
                Action::Fill(L) => State::Jugs(JUG_SIZES.0, right),
                Action::Fill(R) => State::Jugs(left, JUG_SIZES.1),
                Action::Pour(L, R) => {
                    let amount = min(JUG_SIZES.1 - right, left);
                    State::Jugs(left - amount, right + amount)
                }
                Action::Pour(R, L) => {
                    let amount = min(JUG_SIZES.0 - left, right);
                    State::Jugs(left + amount, right - amount)
                }
                _ => *self,
            },
        }
    }

    // Determine whether this state is a goal state
    pub fn is_goal(&self) -> bool {
        matches!(self, State::Jugs(1, _) | State::Jugs(_, 1))
    }

    // Get a vec of actions possible from a given state
    fn possible_actions(&self) -> Vec<Action> {
        vec![
            Action::Fill(L),
            Action::Fill(R),
            Action::Empty(L),
            Action::Empty(R),
            Action::Pour(L, R),
            Action::Pour(R, L),
        ]
        .into_iter()
        .filter(|action| self.action_is_possible(action))
        .collect()
    }

    // Can a given action be taken from this state?
    fn action_is_possible(&self, action: &Action) -> bool {
        match *self {
            State::Jugs(left, right) => match action {
                Action::Empty(L) => left > 0,
                Action::Empty(R) => right > 0,
                Action::Fill(L) => left < JUG_SIZES.0,
                Action::Fill(R) => right < JUG_SIZES.1,
                Action::Pour(L, R) => left > 0 && right < JUG_SIZES.1,
                Action::Pour(R, L) => right > 0 && left < JUG_SIZES.0,
                _ => false,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_do_fill() {
        assert!(State::Jugs(1, 0).action_is_possible(&Action::Fill(L)));
        assert!(State::Jugs(1, 0).action_is_possible(&Action::Fill(R)));
        assert!(State::Jugs(0, 1).action_is_possible(&Action::Fill(L)));
        assert!(State::Jugs(0, 1).action_is_possible(&Action::Fill(R)));

        assert!(!State::Jugs(JUG_SIZES.0, 0).action_is_possible(&Action::Fill(L)));
        assert!(!State::Jugs(0, JUG_SIZES.1).action_is_possible(&Action::Fill(R)));
    }

    #[test]
    fn can_do_empty() {
        assert!(State::Jugs(1, 0).action_is_possible(&Action::Empty(L)));
        assert!(State::Jugs(0, 1).action_is_possible(&Action::Empty(R)));

        assert!(!State::Jugs(0, 0).action_is_possible(&Action::Empty(L)));
        assert!(!State::Jugs(0, 0).action_is_possible(&Action::Empty(R)));
        assert!(!State::Jugs(0, 2).action_is_possible(&Action::Empty(L)));
        assert!(!State::Jugs(2, 0).action_is_possible(&Action::Empty(R)));
    }

    #[test]
    fn can_do_pour() {
        assert!(State::Jugs(3, 0).action_is_possible(&Action::Pour(L, R)));
        assert!(State::Jugs(0, 5).action_is_possible(&Action::Pour(R, L)));
        assert!(State::Jugs(3, 4).action_is_possible(&Action::Pour(L, R)));
        assert!(State::Jugs(2, 5).action_is_possible(&Action::Pour(R, L)));

        assert!(!State::Jugs(0, 0).action_is_possible(&Action::Pour(L, R)));
        assert!(!State::Jugs(3, 5).action_is_possible(&Action::Pour(L, R)));
        assert!(!State::Jugs(0, 0).action_is_possible(&Action::Pour(R, L)));
        assert!(!State::Jugs(3, 5).action_is_possible(&Action::Pour(R, L)));
        assert!(!State::Jugs(0, 1).action_is_possible(&Action::Pour(L, R)));
        assert!(!State::Jugs(1, 0).action_is_possible(&Action::Pour(R, L)));
    }

    #[test]
    fn after_fill() {
        assert_eq!(
            State::Jugs(1, 0).apply_action(&Action::Fill(L)),
            State::Jugs(JUG_SIZES.0, 0),
        );
        assert_eq!(
            State::Jugs(1, 0).apply_action(&Action::Fill(R)),
            State::Jugs(1, JUG_SIZES.1),
        );
        assert_eq!(
            State::Jugs(0, 1).apply_action(&Action::Fill(L)),
            State::Jugs(JUG_SIZES.0, 1),
        );
        assert_eq!(
            State::Jugs(0, 1).apply_action(&Action::Fill(R)),
            State::Jugs(0, JUG_SIZES.1),
        );
    }

    #[test]
    fn after_empty() {
        assert_eq!(
            State::Jugs(1, 0).apply_action(&Action::Empty(L)),
            State::Jugs(0, 0),
        );
        assert_eq!(
            State::Jugs(0, 1).apply_action(&Action::Empty(R)),
            State::Jugs(0, 0),
        );
    }

    #[test]
    fn after_pour() {
        assert_eq!(
            State::Jugs(3, 0).apply_action(&Action::Pour(L, R)),
            State::Jugs(0, 3)
        );
        assert_eq!(
            State::Jugs(0, 5).apply_action(&Action::Pour(R, L)),
            State::Jugs(3, 2)
        );
        assert_eq!(
            State::Jugs(3, 4).apply_action(&Action::Pour(L, R)),
            State::Jugs(2, 5)
        );
        assert_eq!(
            State::Jugs(2, 5).apply_action(&Action::Pour(R, L)),
            State::Jugs(3, 4)
        );
    }

    #[test]
    fn goal_state() {
        assert!(State::Jugs(1, 0).is_goal());
        assert!(State::Jugs(0, 1).is_goal());
        assert!(State::Jugs(1, 2).is_goal());
        assert!(State::Jugs(2, 1).is_goal());
        assert!(!State::Jugs(2, 2).is_goal());
        assert!(!State::Jugs(10, 3).is_goal());
    }
}
