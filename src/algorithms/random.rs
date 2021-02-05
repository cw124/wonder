//! A computer algorithm for player 7 Wonders. Randomly picks a card to build or a wonder stage to complete, discarding
//! a card if neither is possible.

use rand::prelude::*;

use crate::algorithms::PlayingAlgorithm;
use crate::game::{VisibleGame};
use crate::player::Player;
use crate::action::Action;

#[derive(Debug)]
pub struct Random;

impl PlayingAlgorithm for Random {
    fn get_next_action(&self, player: &Player, visible_game: &VisibleGame) -> Action {
        let action_to_take = player.hand().iter()
            .map(|card| player.options_for_card(card, visible_game))
            .filter(|actions| actions.possible())
            .map(|mut actions| actions.actions.swap_remove(thread_rng().gen_range(0, actions.actions.len())))
            .choose(&mut thread_rng());

        match action_to_take {
            Some(action) => action,
            None => Action::Discard(*player.hand().iter()
                .choose(&mut thread_rng())
                .unwrap())
        }
        // TODO: also randomly choose to build a Wonder stage, when doing so is supported.
    }
}