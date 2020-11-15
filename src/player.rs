use crate::card::Card;
use crate::power::Power;
use crate::resources::{ProducedResources, Resources};
use crate::wonder::Wonder;

pub struct PlayerBoard {
    wonder: Wonder,
    built_structures: Vec<Card>,
    built_wonder_stages: Vec<Option<Card>>,  // TODO: how to represent this?
    coins: u32,
}

impl PlayerBoard {
    pub fn new(wonder: Wonder) -> PlayerBoard {
        return PlayerBoard {
            wonder,
            built_structures: vec![],
            built_wonder_stages: vec![],
            coins: 3
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
                },
                Power::Producer(ProducedResources::Single(resources)) => {
                    available_resources += &resources;
                },

                Power::PurchasableProducer(ProducedResources::Choice(choice)) => {
                    choices.push(choice);
                },
                Power::Producer(ProducedResources::Choice(choice)) => {
                    choices.push(choice);
                },

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