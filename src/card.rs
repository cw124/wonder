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

impl Card {
    fn name(&self) -> &str {
        match self {
            Card::LumberYard => "Lumber Yard",
            Card::TreeFarm => "Tree Fram",
            Card::Loom => "Loom",
            Card::Baths => "Baths",
            Card::Tavern => "Tavern",
            Card::EastTradingPost => "East Trading Post",
            Card::Aqueduct => "Aqueduct",
            Card::Forum => "Forum",
            Card::Vineyard => "Vineyard",
        }
    }

    fn players_needed(&self) -> Vec<u32> {
        match self {
            Card::LumberYard => vec![3, 4],
            Card::TreeFarm => vec![6],
            Card::Loom => vec![3, 6],
            Card::Baths => vec![3, 7],
            Card::Tavern => vec![4, 5, 7],
            Card::EastTradingPost => vec![3, 7],
            Card::Aqueduct => vec![3, 7],
            Card::Forum => vec![3, 6, 7],
            Card::Vineyard => vec![3, 6],
        }
    }

    pub fn cost(&self) -> Resources {
        match self {
            Card::LumberYard => Resources::free(),
            Card::TreeFarm => Resources::coins(1),
            Card::Loom => Resources::free(),
            Card::Baths => Resources::stone(1),
            Card::Tavern => Resources::free(),
            Card::EastTradingPost => Resources::free(),
            Card::Aqueduct => Resources::stone(3),
            Card::Forum => Resources::clay(2),
            Card::Vineyard => Resources::free(),
        }
    }

    fn chains_to(&self) -> Vec<Card> {
        match self {
            Card::Baths => vec![Card::Aqueduct],
            Card::EastTradingPost => vec![Card::Forum],
            // Card::Forum => vec![Card::Haven],
            _ => vec![],
        }
    }

    fn colour(&self) -> Colour {
        match self {
            Card::LumberYard => Colour::Brown,
            Card::TreeFarm => Colour::Brown,
            Card::Loom => Colour::Blue,
            Card::Baths => Colour::Blue,
            Card::Tavern => Colour::Yellow,
            Card::EastTradingPost => Colour::Yellow,
            Card::Aqueduct => Colour::Blue,
            Card::Forum => Colour::Blue,
            Card::Vineyard => Colour::Yellow,
        }
    }

    pub fn power(&self) -> Power {
        match self {
            Card::LumberYard => Power::PurchasableProducer(ProducedResources::Single(Resources::wood(1))),
            Card::TreeFarm => Power::PurchasableProducer(ProducedResources::Choice(vec![Resources::wood(1), Resources::clay(1)])),
            Card::Loom => Power::PurchasableProducer(ProducedResources::Single(Resources::loom(1))),
            Card::Baths => Power::VictoryPoints(3),
            Card::Tavern => Power::Coins(5),
            Card::EastTradingPost => Power::BuyAntiClockwise,
            Card::Aqueduct => Power::VictoryPoints(5),
            Card::Forum => Power::Producer(ProducedResources::Choice(vec![Resources::wood(1), Resources::stone(1), Resources::ore(1), Resources::clay(1)])),
            Card::Vineyard => Power::PerGameItemRewards(vec![PerGameItemReward {
                game_item: |game_item| match game_item {
                    CountableGameItem::CountableCard(card) if card.colour() == Colour::Brown => true,
                    _ => false
                },
                me: true,
                neighbours: true,
                coins_per_thing: 1,
                points_per_thing: 0
            }])
        }
    }
}
