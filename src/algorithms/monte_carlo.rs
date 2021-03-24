//! A computer algorithm for playing 7 Wonders. Uses Monte Carlo tree search to determine which action to take.

use crate::action::Action;
use crate::algorithms::random::Random;
use crate::algorithms::{random, PlayingAlgorithm};
use crate::card;
use crate::card::Card;
use crate::game::{Game, OutputMode, SentientPlayer, VisibleGame};
use crate::player::Player;
use std::collections::HashMap;

#[derive(Debug)]
pub struct MonteCarlo;

impl PlayingAlgorithm for MonteCarlo {
    fn get_next_action(&mut self, player: &Player, visible_game: &VisibleGame) -> Action {
        // TODO: this isn't classic Monte Carlo tree search yet. We just evaluate each possible immediate next action
        //  and then play the game randomly until the end as many times as possible, then pick the action that won most.
        //  No tree is actually built, and there's no expansion/exploration tradeoffs.

        // Build a vector of possible actions. We'll evaluate the strength of each and pick the best.
        let mut action_options = Vec::new();
        for card in player.hand() {
            let mut options = player.options_for_card(card, visible_game, false);
            if options.possible() {
                // TODO: for now, just take one option. This will be the only option if the card can be played without
                //  borrowing; otherwise in many cases it will be the one-and-only borrow option. Sometimes, though,
                //  there can be tens of ways to borrow the required resources, which increases our search space
                //  greatly, so just pick one. Long term, we should search all of them, but collapse borrowing that
                //  results in the same coin transfers, as these are equivalent in terms of the strength of the action.
                action_options.push(options.actions.swap_remove(0));
            }
            action_options.push(Action::Discard(*card));
        }

        // Cards we know about: those in our hand, and those played by ourselves and the other players. We'll invent
        // random hands for the other players based on the remaining cards valid for the given number of players.
        // TODO: we can do better here. When we pass our hand to our neighbour, we know the cards they have. We should
        //  write a separate piece of code (that can be reused by multiple algorithms) that tracks this information.
        let mut known_cards: HashMap<Card, u32> = HashMap::new();
        for card in player.hand() {
            *known_cards.entry(*card).or_insert(0) += 1;
        }
        for public_player in visible_game.public_players {
            for card in &public_player.built_structures {
                *known_cards.entry(*card).or_insert(0) += 1;
            }
        }

        // Try 10 full games for each possible action, and choose the action where we win the most.
        // TODO: we need to run way more than 10 games! But everything is far too slow at the moment. Need to optimise
        //  first. Also, we need to change Game so that we run while the other algorithms (eg. humans) are thinking
        //  about their action, giving us more time. Also, we should obviously eventually multi-thread this.
        let mut strength = vec![0; action_options.len()];
        for _ in 0..10 {
            for (option_index, action) in action_options.iter().enumerate() {
                let mut deck = card::new_deck_without(
                    &visible_game.age(),
                    visible_game.public_players.len() as u32,
                    &known_cards,
                );
                let mut sentient_players: Vec<SentientPlayer> = Vec::with_capacity(visible_game.public_players.len());
                for (i, public_player) in visible_game.public_players.iter().enumerate() {
                    if i == visible_game.player_index {
                        // Us. Use our hand and an algorithm that will play the chosen card followed by random cards
                        // thereafter.
                        sentient_players.push(SentientPlayer {
                            player: Player::new_from_public(&public_player, player.hand().clone()),
                            algorithm: Box::new(MonteCarloAlg::new(action.clone())),
                        });
                    } else {
                        // Everyone else. Deal a random hand (since we don't know their actual hand) and play randomly
                        // throughout.
                        sentient_players.push(SentientPlayer {
                            player: Player::new_from_public(
                                &public_player,
                                deck.drain(0..player.hand().len()).collect(),
                            ),
                            algorithm: Box::new(Random {}),
                        });
                    }
                }

                // Play the game to the end and increment the strength of this action if we win.
                let mut game = Game::new_with_players(sentient_players, visible_game.turn, OutputMode::NoOutput);
                let scores = game.play();
                if scores.iter().enumerate().max_by_key(|(_, score)| *score).unwrap().0 == visible_game.player_index {
                    strength[option_index] += 1;
                }
            }
        }

        action_options
            .iter()
            .zip(strength.iter())
            .max_by_key(|(_, strength)| *strength)
            .unwrap()
            .0
            .clone()
    }
}

#[derive(Debug)]
struct MonteCarloAlg {
    action: Option<Action>,
}

impl MonteCarloAlg {
    fn new(action: Action) -> MonteCarloAlg {
        MonteCarloAlg { action: Some(action) }
    }
}

impl PlayingAlgorithm for MonteCarloAlg {
    fn get_next_action(&mut self, player: &Player, visible_game: &VisibleGame) -> Action {
        if let Some(action) = self.action.take() {
            if player.can_play(&action, visible_game) {
                return action;
            }
        }
        random::get_next_action(player, visible_game)
    }
}
