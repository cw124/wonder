//! A set of algorithms (including humans) that can play 7 Wonders.

use std::fmt::Debug;

use crate::action::Action;
use crate::game::VisibleGame;
use crate::player::Player;

pub mod human;
pub mod monte_carlo;
pub mod random;

/// An algorithm that can play 7 Wonders.
pub trait PlayingAlgorithm: Debug {
    /// Returns the action that should be performed by the given player.
    ///
    /// `visible_game` is a restricted view of the state of all players in the game.
    fn get_next_action(&mut self, player: &Player, visible_game: &VisibleGame) -> Action;
}
