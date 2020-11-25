use crate::power::CountableGameItem;
use crate::power::PerGameItemReward;
use crate::power::Power;
use crate::resources::{ProducedResources, Resources};

#[derive(Debug)]
pub enum Card {

    // Age 1
    // =====

    // Brown
    LumberYard,
    TreeFarm,

    // Grey
    Loom,

    // Blue
    Baths,

    // Yellow
    Tavern,
    EastTradingPost,


    // Age 2
    // =====

    // Blue
    Aqueduct,

    // Yellow
    Forum,
    Vineyard,
}

#[derive(PartialEq)]
enum Colour {
    Brown,
    Grey,
    Blue,
    Yellow,
    Red,
    Green,
    Purple
}

struct CardInfo<'a> {
    name: &'a str,
    players_needed: Vec<u32>,
    cost: Resources,
    chains_to: Vec<Card>,
    colour: Colour,
    power: Power,
}

impl Card {
    fn info(&self) -> CardInfo {
        match self {
            Card::LumberYard => CardInfo {
                name: "Lumber Yard",
                players_needed: vec![3, 4],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::wood(1))),
            },

            Card::TreeFarm => CardInfo {
                name: "Tree Farm",
                players_needed: vec![6],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Choice(vec![
                    Resources::wood(1),
                    Resources::clay(1)])),
            },

            Card::Loom => CardInfo {
                name: "Loom",
                players_needed: vec![3, 6],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::loom(1))),
            },

            Card::Baths => CardInfo {
                name: "Baths",
                players_needed: vec![3, 7],
                cost: Resources::stone(1),
                chains_to: vec![Card::Aqueduct],
                colour: Colour::Blue,
                power: Power::VictoryPoints(3),
            },

            Card::Tavern => CardInfo {
                name: "Tavern",
                players_needed: vec![4, 5, 7],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Yellow,
                power: Power::Coins(5),
            },

            Card::EastTradingPost => CardInfo {
                name: "East Trading Post",
                players_needed: vec![3, 7],
                cost: Resources::free(),
                chains_to: vec![Card::Forum],
                colour: Colour::Yellow,
                power: Power::BuyAntiClockwise,
            },

            Card::Aqueduct => CardInfo {
                name: "Aqueduct",
                players_needed: vec![3, 7],
                cost: Resources::stone(3),
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(5),
            },

            Card::Forum => CardInfo {
                name: "Forum",
                players_needed: vec![3, 6, 7],
                cost: Resources::clay(2),
                chains_to: vec![],
                colour: Colour::Yellow,
                power: Power::Producer(ProducedResources::Choice(vec![
                    Resources::wood(1),
                    Resources::stone(1),
                    Resources::ore(1),
                    Resources::clay(1)])),
            },

            Card::Vineyard => CardInfo {
                name: "Vineyard",
                players_needed: vec![3, 6],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Yellow,
                power: Power::PerGameItemRewards(vec![PerGameItemReward {
                    game_item: |game_item| match game_item {
                        CountableGameItem::CountableCard(card) if card.info().colour == Colour::Brown => true,
                        _ => false
                    },
                    me: true,
                    neighbours: true,
                    coins_per_thing: 1,
                    points_per_thing: 0
                }])
            },
        }
    }

    pub fn players_needed(&self) -> Vec<u32> {
        self.info().players_needed
    }

    pub fn cost(&self) -> Resources {
        self.info().cost
    }

    pub fn chains_to(&self) -> Vec<Card> {
        self.info().chains_to
    }

    pub fn colour(&self) -> Colour {
        self.info().colour
    }

    pub fn power(&self) -> Power {
        self.info().power
    }
}
