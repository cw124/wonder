use std::borrow::Borrow;

enum CountableGameItem {
    CountableCard(Card),
}

#[derive(Default)]
#[derive(Debug)]
struct Cost {
    coins: u32,

    wood: u32,
    stone: u32,
    ore: u32,
    clay: u32,

    glass: u32,
    loom: u32,
    papyrus: u32,
}

impl Cost {
    fn free() -> Cost {
        return Cost { ..Default::default() }
    }

    fn coins(num: u32) -> Cost {
        return Cost { coins: num, ..Default::default() }
    }

    fn wood(num: u32) -> Cost {
        return Cost { wood: num, ..Default::default() }
    }

    fn stone(num: u32) -> Cost {
        return Cost { stone: num, ..Default::default() }
    }

    fn clay(num: u32) -> Cost {
        return Cost { clay: num, ..Default::default() }
    }
}

enum Power {
    PurchasableProducer(ProducedResources),
    Producer(ProducedResources),
    VictoryPoints(u32),
    Coins(u32),
    BuyAntiClockwise,
    PerGameItemRewards(Vec<PerGameItemReward>),
}

struct PerGameItemReward {
    game_item: fn(game_item: CountableGameItem) -> bool,
    me: bool,
    neighbours: bool,
    coins_per_thing: u32,
    points_per_thing: u32
}

enum ProducedResources {
    Single(Resource),
    Double(Resource),
    Choice(Vec<Resource>),
}

enum Resource {
    Wood,
    Stone,
    Ore,
    Clay,

    Glass,
    Loom,
    Papyrus,
}

#[derive(Debug)]
enum Card {

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

    fn cost(&self) -> Cost {
        match self {
            Card::LumberYard => Cost::free(),
            Card::TreeFarm => Cost::coins(1),
            Card::Loom => Cost::free(),
            Card::Baths => Cost::stone(1),
            Card::Tavern => Cost::free(),
            Card::EastTradingPost => Cost::free(),
            Card::Aqueduct => Cost::stone(3),
            Card::Forum => Cost::clay(2),
            Card::Vineyard => Cost::free(),
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

    fn power(&self) -> Power {
        match self {
            Card::LumberYard => Power::PurchasableProducer(ProducedResources::Single(Resource::Wood)),
            Card::TreeFarm => Power::PurchasableProducer(ProducedResources::Choice(vec![Resource::Wood, Resource::Clay])),
            Card::Loom => Power::PurchasableProducer(ProducedResources::Single(Resource::Loom)),
            Card::Baths => Power::VictoryPoints(3),
            Card::Tavern => Power::Coins(5),
            Card::EastTradingPost => Power::BuyAntiClockwise,
            Card::Aqueduct => Power::VictoryPoints(5),
            Card::Forum => Power::Producer(ProducedResources::Choice(vec![Resource::Wood, Resource::Stone, Resource::Ore, Resource::Clay])),
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

fn main() {
    let card = Card::Baths;
    println!("Baths cost {:?}", card.cost());
}
