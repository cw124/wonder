use crate::card::Card;
use crate::power::Power;
use crate::resources::{ProducedResources, Resources};
use crate::wonder::Wonder;

struct PlayerBoard {
    wonder: Wonder,
    built_structures: Vec<Card>,
    built_wonder_stages: Vec<Option<Card>>,  // TODO: how to represent this?
    coins: u32,
}

impl PlayerBoard {
    fn can_play(&self, card: Card) -> bool {
        todo!()
    }
}