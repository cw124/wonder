//! A set of algorithms (including humans) that can play 7 Wonders.

use std::fmt::Debug;
use crate::game::Action;
use crate::player::Player;

pub mod human;
pub mod random;


/// An algorithm that can play 7 Wonders.
pub trait PlayingAlgorithm: Debug {

    /// Returns the action that should be performed by the given player.
    ///
    /// `player_index` is the zero-indexed position of the player in the game (eg. 0-4 for a five player game).
    fn get_next_action(&self, player: &Player, player_index: u32) -> Action;
}