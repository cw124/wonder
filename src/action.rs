//! An action defines what a user does each turn (builds a structure, builds a wonder stage, discards a card).

use crate::card::Card;
use crate::game::VisibleGame;
use crate::player::PublicPlayer;
use crate::power::Power;
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;

/// Represents an action.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Action {
    Build(Card, Borrowing),
    Wonder(Card, Borrowing),
    Discard(Card),
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Action::Build(card, _) => write!(f, "Build {}", card.to_string()),
            Action::Wonder(card, _) => write!(f, "Use {} to build a wonder stage", card.to_string()),
            Action::Discard(card) => write!(f, "Discard {}", card.to_string()),
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

    /// Returns true if this borrowing plan is possible for the given game state.
    pub fn valid(&self, visible_game: &VisibleGame) -> bool {
        fn valid_on(borrows: &[Borrow], neighbour: &PublicPlayer) -> bool {
            let neighbour_structures = HashSet::<&Card>::from_iter(&neighbour.built_structures);
            borrows.iter().all(|borrow| neighbour_structures.contains(&borrow.card))
        }
        valid_on(&self.left, &visible_game.left_neighbour()) && valid_on(&self.right, &visible_game.right_neighbour())
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
    /// The index of the resource on the card that is being borrowed. Always 0 except for resources that generate two
    /// resources in one go (ie. [`Sawmill`], [`Quarry`], [`Brickyard`], and [`Foundry`]), where the first resource is 0
    /// and the second resource is 1.
    pub index: u32,
}

impl Borrow {
    pub fn new(card: Card, index: u32) -> Borrow {
        if index > 1 {
            panic!("index must be 0 or 1");
        }
        if let Power::Producer(resources) | Power::PurchasableProducer(resources) = card.power() {
            if index == 1 && (resources.len() != 1 || resources[0].max() != 2) {
                panic!("index can only be 1 if card is a double production card");
            }
        }
        Borrow { card, index }
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
                Borrowing::new(vec![Borrow::new(Card::LumberYard, 0)], vec![]),
            )],
        };
        assert_eq!(false, options.own_cards_only());
    }
}
