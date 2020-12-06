use crate::card::Card;
use crate::power::Power;
use crate::resources::{ProducedResources, Resources};
use crate::wonder::{WonderBoard, WonderType, WonderSide};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Player {
    wonder: WonderBoard,
    built_structures: Vec<Card>,
    built_wonder_stages: Vec<Option<Card>>, // TODO: how to represent this?
    coins: u32,
    hand: Vec<Card>,
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

    pub fn strength(self) -> f32 {
        return self.built_structures.iter()
            .map(|structure| structure.strength())
            .sum();
    }

    pub fn new(wonder_type: WonderType, wonder_side: WonderSide, hand: Vec<Card>) -> Player {
        Player {
            wonder: WonderBoard { wonder_type, wonder_side },
            built_structures: vec![],
            built_wonder_stages: vec![],
            coins: 3,
            hand
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
    use crate::card::Card;
    use crate::player::Player;
    use crate::wonder::{WonderType, WonderSide};
    use Card::*;

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
            vec![StonePit, Quarry, Aqueduct, Loom1, Apothecary]
        );
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
