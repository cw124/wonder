//! An action defines what a user does each turn (builds a structure, builds a wonder stage, discards a card).

use std::fmt;
use std::fmt::{Display, Formatter};

use crate::card::Card;
use crate::resources::Resource;

/// Represents an action.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Action {
    Build(Card, Borrowing),
    Wonder(Card, Borrowing),
    Discard(Card),
}

impl Display for Action {
    /// Formats the action, returning only public information. For example, if the action is to discard a card, the
    /// card in question is not revealed.
    /// TODO: Handle borrowing. In which case we probably need a different function that can take the player index and
    ///  return things like "Build Baths by paying 2 coins to player 3 to borrow stone".
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Action::Build(card, _) => write!(f, "Build {}", card.to_string()),
            Action::Wonder(_, _) => write!(f, "Build a wonder stage"),
            Action::Discard(_) => write!(f, "Discard"),
        }
    }
}

/// Represents the possible actions a player can take in order to lay a particular card.
pub struct ActionOptions {
    pub actions: Vec<Action>,
}

impl ActionOptions {
    /// Returns `true` if it's possible to lay the card, `false` otherwise.
    pub fn possible(&self) -> bool {
        !self.actions.is_empty()
    }

    /// Returns `true` if the card can be laid using the player's own built structures only (no borrowing).
    pub fn own_cards_only(&self) -> bool {
        if self.actions.len() != 1 {
            return false;
        }
        if let Action::Build(_, borrowing) = &self.actions[0] {
            if !borrowing.has_borrowing() {
                return true;
            }
        }
        false
    }
}

/// Represents resources borrowed from left and right neighbours as part of an action.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Borrowing {
    pub left: Vec<Borrow>,
    pub right: Vec<Borrow>,
}

impl Borrowing {
    pub fn new(left: Vec<Borrow>, right: Vec<Borrow>) -> Borrowing {
        Borrowing { left, right }
    }

    /// Convenience constructor when no borrowing is required.
    pub fn no_borrowing() -> Borrowing {
        Self::new(vec![], vec![])
    }

    /// Returns true if this borrowing plan has not borrowing (ie. the player can use their own cards exclusively).
    pub fn has_borrowing(&self) -> bool {
        !self.left.is_empty() || !self.right.is_empty()
    }
}

/// Represents the borrowing of a specific resource.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Borrow {
    /// The card the resource is on.
    pub card: Card,
    /// The resource being borrowed.
    pub resource: Resource,
}

impl Borrow {
    pub fn new(card: Card, resource: Resource) -> Borrow {
        Borrow { card, resource }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn possible_with_no_options() {
        let options = ActionOptions { actions: vec![] };
        assert_eq!(false, options.possible());
    }

    #[test]
    fn possible_with_options() {
        let options = ActionOptions {
            actions: vec![Action::Build(Card::LumberYard, Borrowing::no_borrowing())],
        };
        assert_eq!(true, options.possible());
    }

    #[test]
    fn own_cards_only_with_no_borrowing() {
        let options = ActionOptions {
            actions: vec![Action::Build(Card::LumberYard, Borrowing::no_borrowing())],
        };
        assert_eq!(true, options.own_cards_only());
    }

    #[test]
    fn own_cards_only_with_borrowing() {
        let options = ActionOptions {
            actions: vec![Action::Build(
                Card::Stockade,
                Borrowing::new(vec![Borrow::new(Card::LumberYard, Resource::Wood)], vec![]),
            )],
        };
        assert_eq!(false, options.own_cards_only());
    }
}
