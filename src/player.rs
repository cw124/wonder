use std::collections::HashMap;

use crate::card::{Card, Colour};
use crate::power::Power;
use crate::power::ScienceItem;
use crate::resources::{ProducedResources, Resources};
use crate::wonder::{WonderBoard, WonderSide, WonderType};
use std::collections::hash_map::RandomState;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Player {
    pub wonder: WonderBoard,
    built_structures: Vec<Card>,
    built_wonder_stages: Vec<Option<Card>>,
    // TODO: how to represent this?
    pub coins: u32,
    pub hand: Vec<Card>,
}

#[allow(dead_code)]
impl Player {
    pub(crate) fn build_structure(&mut self, structure: Card) -> bool {
        if self.can_play(structure) {
            self.built_structures.push(structure);
            true
        } else {
            false
        }
    }

    fn evaluate_green(&self, colour_cards: &Vec<Card>) -> f32 {
        let mut science_items_count: HashMap<ScienceItem, i32, RandomState> = HashMap::new();

        science_items_count.insert(ScienceItem::Compass, 0);
        science_items_count.insert(ScienceItem::Cog, 0);
        science_items_count.insert(ScienceItem::Tablet, 0);


        for card in colour_cards.iter() {
            match card.power() {
                Power::Science(science_items) =>
                    for science_item in science_items.iter() {
                        let count = science_items_count.entry(*science_item).or_insert(0);
                        *count += 1;
                    },
                _ => {}
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

        return score_for_all_symbol_groups + score_for_sets_of_identical_symbols;
    }

    fn evaluate_colour(&self, cards_of_given_colour: &Vec<Card>) -> f32 {
        let colour = cards_of_given_colour.get(0).unwrap().colour();

        match colour {
            Colour::Green => self.evaluate_green(cards_of_given_colour),
            _ => cards_of_given_colour.iter().map(|card| card.immediate_strength()).sum(),
        }
    }

    pub fn strength(&self) -> f32 {
        let mut colour_to_structure = HashMap::new();
        for structure in self.built_structures.iter() {
            let colour_structures = colour_to_structure.entry(structure.colour()).or_insert(vec![]);
            colour_structures.push(*structure)
        }

        colour_to_structure.iter()
            .map(|colour_entry|
                {
                    let x = self.evaluate_colour(colour_entry.1);
                    println!("Evaluated color: {:#?}", x);
                    x
                }
            )
            .sum()
    }

    pub fn new(wonder_type: WonderType, wonder_side: WonderSide, hand: Vec<Card>) -> Player {
        Player {
            wonder: WonderBoard { wonder_type, wonder_side },
            built_structures: vec![],
            built_wonder_stages: vec![],
            coins: 3,
            hand,
        }
    }

    /// Returns `true` if the user can afford to play the given card, given the resources the player
    /// has access to.
    ///
    /// TODO: doesn't currently deal with borrowing resources from neighbours.
    pub fn can_play(&self, card: Card) -> bool {
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

    use crate::card::Card;
    use crate::player::Player;
    use crate::wonder::{WonderSide, WonderType};

    #[test]
    fn can_play_returns_true_when_player_can_afford_card() {
        // TODO: @Before etc
        let player = create_player();
        assert_eq!(true, player.can_play(Card::LumberYard));
    }

    #[test]
    fn can_play_returns_true_after_player_builds_required_resources() {
        let mut player = create_player();
        player.build_structure(StonePit);
        assert_eq!(false, player.can_play(Aqueduct));
        assert_eq!(true, player.build_structure(Card::Quarry));
        assert_eq!(true, player.can_play(Aqueduct));
    }

    #[test]
    fn strength_returns_sum_of_card_strengths() {
        assert_strength_after_playing_cards(0.0, vec![StonePit]);
        assert_strength_after_playing_cards(5.0, vec![StonePit, Quarry, Aqueduct]);
        assert_strength_after_playing_cards(
            6.0,
            vec![StonePit, Quarry, Aqueduct, Loom1, Apothecary],
        );
    }

    #[test]
    fn strength_returns_correct_strength_of_green_structures() {
        assert_strength_after_playing_cards(
            1.0,
            add_prerequisites(vec![Lodge]),
        );

        assert_strength_after_playing_cards(
            4.0,
            add_prerequisites(vec![Lodge, Apothecary]),
        );

        assert_strength_after_playing_cards(
            9.0,
            add_prerequisites(vec![Lodge, Apothecary, Dispensary]),
        );

        assert_strength_after_playing_cards(
            10.0,
            add_prerequisites(vec![Lodge, Workshop, Library]),
        );

        assert_strength_after_playing_cards(  // rulebook example
            21.0,
            add_prerequisites(vec![Lodge, Apothecary, Dispensary, Laboratory, Workshop, Library]),
        );
    }

    fn add_prerequisites(structures: Vec<Card>) -> Vec<Card> {
        // clay: 2, loom: 1, papyrus: 1
        let prerequisites = vec![Quarry, Sawmill, Foundry, Brickyard, Press1, Glassworks1, Loom1];
        prerequisites.iter().copied().chain(structures.iter().copied()).collect()
    }

    fn assert_strength_after_playing_cards(strength: f32, cards: Vec<Card>) {
        let mut player = create_player();
        player.coins = 100;
        for card in cards.iter() {
            println!("Building card: {:#?}", card);
            assert_eq!(true, player.build_structure(*card));
        };
        assert_eq!(strength, player.strength());
    }

    #[test]
    fn can_play_returns_false_when_player_cannot_pay() {
        let mut player = Player::new(WonderType::ColossusOfRhodes, WonderSide::A, vec![]);
        player.coins = 0; //TODO introduce a Bank type to allow for double-entry bookkeeping instead of this
        assert_eq!(false, player.can_play(Card::TreeFarm));
    }

    #[test]
    fn can_play_returns_false_when_both_choice_resources_needed() {
        // TODO implement
    }

    fn create_player() -> Player {
        Player::new(WonderType::ColossusOfRhodes, WonderSide::A, vec![])
    }
}
