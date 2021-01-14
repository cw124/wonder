//! A computer algorithm for player 7 Wonders. Simply picks a random action each turn.

use rand::prelude::*;

use crate::algorithms::PlayingAlgorithm;
use crate::game::Action;
use crate::player::Player;

#[derive(Debug)]
pub struct Random {
}

impl PlayingAlgorithm for Random {
    fn get_next_action(&self, player: &Player, _player_index: u32) -> Action {
        Action::Build(*player.hand().iter()
            .filter(|card| player.can_play(&Action::Build(**card)))
            .choose(&mut thread_rng())
            .unwrap())
        // TODO: also randomly choose to build a Wonder stage or discard.
        // TODO: need to discard if no other option is possible (rather than panic!)
    }
}