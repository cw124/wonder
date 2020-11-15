use crate::card::Card;
use crate::power::Power;
use crate::resources::{ProducedResources, Resources};
use crate::wonder::Wonder;

pub struct PlayerBoard {
    wonder: Wonder,
    built_structures: Vec<Card>,
    built_wonder_stages: Vec<Option<Card>>,
    // TODO: how to represent this?
    coins: u32,
}

impl PlayerBoard {
    pub(crate) fn build_structure(&mut self, structure: Card) -> bool {
        return if self.can_play(structure) {
            self.built_structures.push(structure);
            true
        } else {
            false
        };
    }
}

impl PlayerBoard {
    pub fn new(wonder: Wonder) -> PlayerBoard {
        return PlayerBoard {
            wonder,
            built_structures: vec![],
            built_wonder_stages: vec![],
            coins: 3,
        };
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

        if choices.len() > 0 {
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

        return false;
    }
}

#[cfg(test)]
mod tests {
    use std::intrinsics::transmute;

    use crate::card::Card;
    // This is how you write unit tests in rust, embedded in module file.
    use crate::player::PlayerBoard;
    use crate::wonder::Wonder;

    #[test]
    fn should_can_play_return_true_when_player_can_afford_card() {
        // TODO: @Before etc
        let player = create_player();
        assert_eq!(true, player.can_play(Card::LumberYard));
    }

    #[test]
    fn should_can_play_return_true_after_player_builds_required_resources() {
        let mut player = create_player();
        player.build_structure(Card::StonePit);
        assert_eq!(false, player.can_play(Card::Aqueduct));
        assert_eq!(true, player.build_structure(Card::Quarry));
        assert_eq!(true, player.can_play(Card::Aqueduct));
    }


    #[test]
    fn should_can_play_return_false_when_player_cannot_pay() {
        let mut player = PlayerBoard::new(Wonder::ColossusOfRhodesA);
        player.coins = 0; //TODO introduce a Bank type to allow for double-entry bookkeeping instead of this
        assert_eq!(false, player.can_play(Card::TreeFarm));
    }

    #[test]
    fn should_can_play_return_false_when_both_choice_resources_needed() {
        // TODO implement
    }

    fn create_player() -> PlayerBoard {
        let mut player = PlayerBoard::new(Wonder::ColossusOfRhodesA);
        player
    }
}