//! An action defines what a user does each turn (builds a structure, builds a wonder stage, discards a card).

use crate::card::Card;
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::game::VisibleGame;
use std::collections::HashSet;
use std::iter::FromIterator;
use crate::power::Power;
use crate::player::PublicPlayer;

/// Represents an action.
#[allow(dead_code)]
#[derive(Debug)]
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

/// Represents resources borrowed from left and right neighbours as part of an action.
#[derive(Debug, Eq, PartialEq)]
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
}

/// Represents the borrowing of a specific resource.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Borrow {
    /// The card the resource is on.
    card: Card,
    /// The index of the resource on the card that is being borrowed. Always 0 except for resources that generate two
    /// resources in one go (ie. [`Sawmill`], [`Quarry`], [`Brickyard`], and [`Foundry`]), where the first resource is 0
    /// and the second resource is 1.
    index: u32,
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