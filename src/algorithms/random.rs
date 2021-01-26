//! A computer algorithm for player 7 Wonders. Randomly picks a card to build or a wonder stage to complete, discarding
//! a card if neither is possible.

use rand::prelude::*;

use crate::algorithms::PlayingAlgorithm;
use crate::game::{Action, VisibleGame};
use crate::player::Player;

#[derive(Debug)]
pub struct Random;

impl PlayingAlgorithm for Random {
    fn get_next_action(&self, player: &Player, _visible_game: &VisibleGame) -> Action {
        let card_to_build = player.hand().iter()
            .filter(|card| player.can_play(&Action::Build(**card), _visible_game))
            .choose(&mut thread_rng());

        match card_to_build {
            Some(card) => Action::Build(*card),
            None => Action::Discard(*player.hand().iter()
                .choose(&mut thread_rng())
                .unwrap())
        }
        // TODO: also randomly choose to build a Wonder stage, when doing so is supported.
    }
}