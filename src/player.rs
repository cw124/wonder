use std::collections::HashMap;

use crate::card::{Card, Colour};
use crate::game::Action;
use crate::power::Power;
use crate::power::ScienceItem;
use crate::resources::{ProducedResources, Resources};
use crate::wonder::{WonderBoard, WonderSide, WonderType};
use std::fmt::Debug;
use crate::algorithms::PlayingAlgorithm;
use std::mem;

#[derive(Debug)]
pub struct Player {
    algorithm: Box<dyn PlayingAlgorithm>,
    wonder: WonderBoard,
    built_structures: Vec<Card>,
    built_wonder_stages: Vec<Option<Card>>, // TODO: how to represent this?
    coins: u32,
    hand: Vec<Card>,
}

#[allow(dead_code)]
impl Player {
    pub fn new(
        wonder_type: WonderType,
        wonder_side: WonderSide,
        hand: Vec<Card>,
        algorithm: Box<dyn PlayingAlgorithm>) -> Player {

        Player {
            algorithm,
            wonder: WonderBoard { wonder_type, wonder_side },
            built_structures: vec![],
            built_wonder_stages: vec![],
            coins: 3,
            hand,
        }
    }

    pub fn algorithm(&self) -> &dyn PlayingAlgorithm {
        &*self.algorithm
    }

    pub fn wonder(&self) -> &WonderBoard {
        &self.wonder
    }

    pub fn built_structures(&self) -> &Vec<Card> {
        &self.built_structures
    }

    pub fn coins(&self) -> u32 {
        self.coins
    }

    pub fn hand(&self) -> &Vec<Card> {
        &self.hand
    }

    /// Performs the given [`Action`] on the current player, for example moving a card from the player's hand into the
    /// player's built structures. Returns `true` if the action is legal, `false` otherwise (in which case this function
    /// otherwise does nothing).
    pub fn do_action(&mut self, action: &Action) -> bool {
        if self.can_play(action) {
            match action {
                Action::Build(card) => {
                    let index = self.hand.iter().position(|c| c == card).unwrap();
                    let card_from_hand = self.hand.swap_remove(index);
                    self.built_structures.push(card_from_hand);
                    self.coins -= card_from_hand.cost().coins;
                    // TODO: deal with borrowed resources
                }
                Action::Wonder(_) => todo!(),
                Action::Discard => todo!(),
            }
            true
        } else {
            false
        }
    }

    /// Replaces this player's hand with the given cards, returning the hand the player had before the swap.
    pub fn swap_hand(&mut self, new_hand: Vec<Card>) -> Vec<Card> {
        mem::replace(&mut self.hand, new_hand)
    }

    fn evaluate_green(colour_cards: &[Card]) -> f32 {
        let mut science_items_count: HashMap<ScienceItem, i32> = HashMap::new();

        science_items_count.insert(ScienceItem::Compass, 0);
        science_items_count.insert(ScienceItem::Cog, 0);
        science_items_count.insert(ScienceItem::Tablet, 0);


        for card in colour_cards.iter() {
            if let Power::Science(science_items) = card.power() {
                for science_item in science_items.iter() {
                    let count = science_items_count.entry(*science_item).or_insert(0);
                    *count += 1;
                }
            }
        }

        let score_for_sets_of_identical_symbols: f32 = science_items_count.iter()
            .filter(|(_, count)| **count > 0)
            .map(|(_, count)| {
                (*count as f32).powf(2f32)
            })
            .sum();

        let score_for_all_symbol_groups: f32 =  7f32 * 
            *science_items_count.iter().min_by_key(|(_, count)| *count).unwrap().1 as f32;

        score_for_all_symbol_groups + score_for_sets_of_identical_symbols
    }

    fn evaluate_colour(cards_of_given_colour: &[Card]) -> f32 {
        let colour = cards_of_given_colour.get(0).unwrap().colour();

        match colour {
            Colour::Green => Self::evaluate_green(cards_of_given_colour),
            _ => cards_of_given_colour.iter().map(|card| card.immediate_strength()).sum(),
        }
    }

    fn strength_internal(cards: &[Card]) -> f32 {
        let mut colour_to_structure = HashMap::new();
        for structure in cards.iter() {
            let colour_structures = colour_to_structure.entry(structure.colour()).or_insert(vec![]);
            colour_structures.push(*structure)
        }

        colour_to_structure.iter()
            .map(|colour_entry| Self::evaluate_colour(colour_entry.1))
            .sum()
    }

    /// Returns this player's "strength" -- a number where a higher value means the player is doing better than a lower
    /// value.
    pub fn strength(&self) -> f32 {
        Self::strength_internal(&self.built_structures)
    }

    pub fn can_play(&self, action: &Action) -> bool {
        match action {
            Action::Build(card) => self.can_play_card(card),
            Action::Wonder(_) => todo!(),
            Action::Discard => todo!(),
        }
    }

    /// Returns `true` if the user can afford to play the given card, given the resources the player
    /// has access to.
    ///
    /// TODO: doesn't currently deal with borrowing resources from neighbours.
    fn can_play_card(&self, card: &Card) -> bool {
        if !self.hand.iter().any(|c| c == card) {
            return false;
        }

        // Initialise a Resources struct with the number of coins we have.
        let mut available_resources = Resources::coins(self.coins);

        // Add all the other resources we always have access to (ie. those that are not resource
        // "choice" cards. At the same time, make a vector of resources choices available to us.
        let mut choices = Vec::new();
        for card in &self.built_structures {
            match card.power() {
                // TODO: can we write these four options more succinctly?
                Power::PurchasableProducer(ProducedResources::Single(resources)) => {
                    available_resources += &resources;
                }
                Power::Producer(ProducedResources::Single(resources)) => {
                    available_resources += &resources;
                }

                Power::PurchasableProducer(ProducedResources::Choice(choice)) => {
                    choices.push(choice);
                }
                Power::Producer(ProducedResources::Choice(choice)) => {
                    choices.push(choice);
                }

                _ => {}
            }
        }

        // Add Wonder starting resources.
        available_resources += &self.wonder.starting_resource();

        if available_resources.can_afford(&card.cost()) {
            return true;
        }

        if !choices.is_empty() {
            // Iterate through all possible combinations of the choices we have. Use the iteration
            // index to work out which choice to make for each card.

            let combinations: u32 = choices.iter()
                .fold(1, |x, y| x * y.len() as u32);

            for combination in 0..combinations {
                let mut available_resources_option = available_resources.clone();
                let mut combination = combination;
                for choice in &choices {
                    let index = combination % choice.len() as u32;
                    available_resources_option += &choice[index as usize];
                    combination /= choice.len() as u32;
                }
                if available_resources_option.can_afford(&card.cost()) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use Card::*;

    use super::*;
    use crate::algorithms::random::Random;

    #[test]
    fn can_play_returns_true_when_player_can_afford_card() {
        // TODO: @Before etc
        let player = new_player(vec![LumberYard]);
        assert_eq!(true, player.can_play(&Action::Build(LumberYard)));
    }

    #[test]
    fn can_play_returns_true_after_player_builds_required_resources() {
        let mut player = new_player(vec![StonePit, Quarry, Aqueduct]);
        player.do_action(&Action::Build(StonePit));
        assert_eq!(false, player.can_play(&Action::Build(Aqueduct)));
        assert_eq!(true, player.do_action(&Action::Build(Quarry)));
        assert_eq!(true, player.can_play(&Action::Build(Aqueduct)));
    }

    #[test]
    fn strength_returns_sum_of_card_strengths() {
        assert_eq!(0.0, Player::strength_internal(&vec![StonePit]));
        assert_eq!(5.0, Player::strength_internal(&vec![StonePit, Quarry, Aqueduct]));
        assert_eq!(6.0, Player::strength_internal(&vec![StonePit, Quarry, Aqueduct, Loom1, Apothecary]));
    }

    #[test]
    fn strength_returns_correct_strength_of_green_structures() {
        assert_eq!(1.0, Player::strength_internal(&vec![Lodge]));
        assert_eq!(4.0, Player::strength_internal(&vec![Lodge, Apothecary]));
        assert_eq!(9.0, Player::strength_internal(&vec![Lodge, Apothecary, Dispensary]));
        assert_eq!(10.0, Player::strength_internal(&vec![Lodge, Workshop, Library]));
        assert_eq!(21.0, Player::strength_internal(&vec![Lodge, Apothecary, Dispensary, Laboratory, Workshop, Library]));  // rulebook example
    }

    #[test]
    fn can_play_returns_false_when_player_cannot_pay() {
        let mut player = new_player(vec![]);
        player.coins = 0; //TODO introduce a Bank type to allow for double-entry bookkeeping instead of this
        assert_eq!(false, player.can_play(&Action::Build(TreeFarm)));
    }

    #[test]
    fn can_play_returns_false_when_both_choice_resources_needed() {
        // TODO implement
    }

    #[test]
    fn do_action_returns_false_if_action_not_playable() {
        let mut player = new_player(vec![LumberYard]);
        assert_eq!(false, player.do_action(&Action::Build(StonePit)));
    }

    #[test]
    fn do_action_transfers_built_card_from_hand_to_built_structures() {
        let mut player = new_player(vec![LumberYard]);
        assert_eq!(0, player.built_structures.len());
        assert_eq!(1, player.hand.len());
        assert_eq!(true, player.do_action(&Action::Build(LumberYard)));
        assert_eq!(1, player.built_structures.len());
        assert_eq!(0, player.hand.len());
    }

    fn new_player(hand: Vec<Card>) -> Player {
        Player::new(WonderType::ColossusOfRhodes, WonderSide::A, hand, Box::new(Random {}))
    }
}
