use crate::power::{CountableGameItem, ScienceItem};
use crate::power::PerGameItemReward;
use crate::power::Power;
use crate::resources::{ProducedResources, Resources};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Card {

    // Age 1
    // =====

    // Brown
    LumberYard,
    StonePit,
    ClayPool,
    OreVein,
    TreeFarm,
    Excavation,
    ClayPit,
    TimberYard,
    ForestCave,
    Mine,

    // Grey
    Loom1,
    Glassworks1,
    Press1,

    // Blue
    Pawnshop,
    Baths,
    Altar,
    Theater,

    // Yellow
    Tavern,
    EastTradingPost,
    WestTradingPost,
    Marketplace,

    // Green
    Apothecary,
    Workshop,
    Scriptorium,

    // Red
    Stockade,
    Barracks,
    GuardTower,


    // Age 2
    // =====

    // Brown
    Sawmill,
    Quarry,
    Brickyard,
    Foundry,

    // Grey
    Loom2,
    Glassworks2,
    Press2,

    // Blue
    Aqueduct,
    Temple,
    Statue,
    Courthouse,

    // Yellow
    Forum,
    Caravansery,
    Vineyard,
    Bazar,

    // Green
    Dispensary,
    Laboratory,
    Library,
    School,

    // Red
    Walls,
    TrainingGround,
    Stables,
    ArcheryRange,


    // Age 3
    // =====

    // Blue
    Pantheon,
    Gardens,
    TownHall,
    Palace,
    Senate,

    // Yellow
    Haven,
    Lighthouse,
    ChamberOfCommerce,
    Arena,

    // Green
    Lodge,
    Observatory,
    University,
    Academy,
    Study,

    // Red
    Fortifications,
    Circus,
    Arsenal,
    SiegeWorkshop,

    // Purple
    WorkersGuild,
    CraftsmensGuild,
    TradersGuild,
    PhilosophersGuild,
    SpiesGuild,
    StrategistsGuild,
    ShipownersGuild,
    ScientistsGuild,
    MagistratesGuild,
    BuildersGuild
}

#[derive(PartialEq)]
pub enum Colour {
    Brown,
    Grey,
    Blue,
    Yellow,
    Red,
    Green,
    Purple
}

#[allow(dead_code)]
struct CardInfo<'a> {
    name: &'a str,
    players_needed: Vec<u32>,
    cost: Resources,
    chains_to: Vec<Card>,
    colour: Colour,
    power: Power,
}

#[allow(dead_code)]
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

            Card::StonePit => CardInfo {
                name: "Stone Pit",
                players_needed: vec![3, 5],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::stone(1))),
            },

            Card::ClayPool => CardInfo {
                name: "Clay Pool",
                players_needed: vec![3, 5],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::clay(1))),
            },

            Card::OreVein => CardInfo {
                name: "Ore Vein",
                players_needed: vec![3, 4],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::ore(1))),
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

            Card::Excavation => CardInfo {
                name: "Excavation",
                players_needed: vec![4],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Choice(vec![
                    Resources::stone(1),
                    Resources::clay(1)])),
            },

            Card::ClayPit => CardInfo {
                name: "Clay Pit",
                players_needed: vec![3],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Choice(vec![
                    Resources::clay(1),
                    Resources::ore(1)])),
            },

            Card::TimberYard => CardInfo {
                name: "Timber Yard",
                players_needed: vec![3],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Choice(vec![
                    Resources::stone(1),
                    Resources::wood(1)])),
            },

            Card::ForestCave => CardInfo {
                name: "Forest Cave",
                players_needed: vec![5],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Choice(vec![
                    Resources::wood(1),
                    Resources::ore(1)])),
            },

            Card::Mine => CardInfo {
                name: "Mine",
                players_needed: vec![6],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Choice(vec![
                    Resources::ore(1),
                    Resources::stone(1)])),
            },

            Card::Loom1 => CardInfo {
                name: "Loom",
                players_needed: vec![3, 6],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Grey,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::loom(1))),
            },

            Card::Glassworks1 => CardInfo {
                name: "Glassworks",
                players_needed: vec![3, 6],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Grey,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::glass(1))),
            },

            Card::Press1 => CardInfo {
                name: "Press",
                players_needed: vec![3, 6],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Grey,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::papyrus(1))),
            },

            Card::Pawnshop => CardInfo {
                name: "Pawnshop",
                players_needed: vec![4, 7],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(3),
            },

            Card::Baths => CardInfo {
                name: "Baths",
                players_needed: vec![3, 7],
                cost: Resources::stone(1),
                chains_to: vec![Card::Aqueduct],
                colour: Colour::Blue,
                power: Power::VictoryPoints(3),
            },

            Card::Altar => CardInfo {
                name: "Altar",
                players_needed: vec![3, 5],
                cost: Resources::free(),
                chains_to: vec![Card::Temple],
                colour: Colour::Blue,
                power: Power::VictoryPoints(2),
            },

            Card::Theater => CardInfo {
                name: "Theater",
                players_needed: vec![3, 6],
                cost: Resources::free(),
                chains_to: vec![Card::Statue],
                colour: Colour::Blue,
                power: Power::VictoryPoints(2),
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
                power: Power::BuyBrownAntiClockwise,
            },

            Card::WestTradingPost => CardInfo {
                name: "West Trading Post",
                players_needed: vec![3, 7],
                cost: Resources::free(),
                chains_to: vec![Card::Forum],
                colour: Colour::Yellow,
                power: Power::BuyBrownClockwise,
            },

            Card::Marketplace => CardInfo {
                name: "Marketplace",
                players_needed: vec![3, 6],
                cost: Resources::free(),
                chains_to: vec![Card::Caravansery],
                colour: Colour::Yellow,
                power: Power::BuyGrey,
            },

            Card::Apothecary => CardInfo {
                name: "Apothecary",
                players_needed: vec![3, 5],
                cost: Resources::loom(1),
                chains_to: vec![Card::Stables, Card::Dispensary],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Compass]),
            },

            Card::Workshop => CardInfo {
                name: "Workshop",
                players_needed: vec![3, 7],
                cost: Resources::glass(1),
                chains_to: vec![Card::ArcheryRange, Card::Laboratory],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Cog]),
            },

            Card::Scriptorium => CardInfo {
                name: "Scriptorium",
                players_needed: vec![3, 4],
                cost: Resources::papyrus(1),
                chains_to: vec![Card::Courthouse, Card::Library],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Tablet]),
            },

            Card::Stockade => CardInfo {
                name: "Stockade",
                players_needed: vec![3, 7],
                cost: Resources::wood(1),
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(1),
            },

            Card::Barracks => CardInfo {
                name: "Barracks",
                players_needed: vec![3, 5],
                cost: Resources::ore(1),
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(1),
            },

            Card::GuardTower => CardInfo {
                name: "Guard Tower",
                players_needed: vec![3, 4],
                cost: Resources::clay(1),
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(1),
            },

            Card::Sawmill => CardInfo {
                name: "Sawmill",
                players_needed: vec![3, 4],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::wood(2))),
            },

            Card::Quarry => CardInfo {
                name: "Quarry",
                players_needed: vec![3, 4],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::stone(2))),
            },

            Card::Brickyard => CardInfo {
                name: "Brickyard",
                players_needed: vec![3, 4],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::clay(2))),
            },

            Card::Foundry => CardInfo {
                name: "Foundry",
                players_needed: vec![3, 4],
                cost: Resources::coins(1),
                chains_to: vec![],
                colour: Colour::Brown,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::ore(2))),
            },

            Card::Loom2 => CardInfo {
                name: "Loom",
                players_needed: vec![3, 5],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Grey,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::loom(1))),
            },

            Card::Glassworks2 => CardInfo {
                name: "Glassworks",
                players_needed: vec![3, 5],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Grey,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::glass(1))),
            },

            Card::Press2 => CardInfo {
                name: "Press",
                players_needed: vec![3, 5],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Grey,
                power: Power::PurchasableProducer(ProducedResources::Single(Resources::papyrus(1))),
            },

            Card::Aqueduct => CardInfo {
                name: "Aqueduct",
                players_needed: vec![3, 7],
                cost: Resources::stone(3),
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(5),
            },

            Card::Temple => CardInfo {
                name: "Temple",
                players_needed: vec![3, 6],
                cost: Resources { wood: 1, clay: 1, glass: 1, ..Default::default() },
                chains_to: vec![Card::Pantheon],
                colour: Colour::Blue,
                power: Power::VictoryPoints(3),
            },

            Card::Statue => CardInfo {
                name: "Statue",
                players_needed: vec![3, 7],
                cost: Resources { wood: 1, ore: 2, ..Default::default() },
                chains_to: vec![Card::Gardens],
                colour: Colour::Blue,
                power: Power::VictoryPoints(4),
            },

            Card::Courthouse => CardInfo {
                name: "Courthouse",
                players_needed: vec![3, 5],
                cost: Resources { clay: 2, loom: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(4),
            },

            Card::Forum => CardInfo {
                name: "Forum",
                players_needed: vec![3, 6, 7],
                cost: Resources::clay(2),
                chains_to: vec![Card::Haven],
                colour: Colour::Yellow,
                power: Power::Producer(ProducedResources::Choice(vec![
                    Resources::loom(1),
                    Resources::glass(1),
                    Resources::papyrus(1)])),
            },

            Card::Caravansery => CardInfo {
                name: "Caravansery",
                players_needed: vec![3, 5, 6],
                cost: Resources::wood(2),
                chains_to: vec![Card::Lighthouse],
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
                power: Power::per_card_reward(Colour::Brown, true, true, 1, 0)
            },

            Card::Bazar => CardInfo {
                name: "Bazar",
                players_needed: vec![4, 7],
                cost: Resources::free(),
                chains_to: vec![],
                colour: Colour::Yellow,
                power: Power::per_card_reward(Colour::Grey, true, true, 2, 0)
            },

            Card::Dispensary => CardInfo {
                name: "Dispensary",
                players_needed: vec![3, 4],
                cost: Resources { ore: 2, glass: 1, ..Default::default() },
                chains_to: vec![Card::Arena, Card::Lodge],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Compass]),
            },

            Card::Laboratory => CardInfo {
                name: "Laboratory",
                players_needed: vec![3, 5],
                cost: Resources { clay: 2, papyrus: 1, ..Default::default() },
                chains_to: vec![Card::SiegeWorkshop, Card::Observatory],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Cog]),
            },

            Card::Library => CardInfo {
                name: "Library",
                players_needed: vec![3, 6],
                cost: Resources { stone: 2, loom: 1, ..Default::default() },
                chains_to: vec![Card::Senate, Card::University],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Tablet]),
            },

            Card::School => CardInfo {
                name: "School",
                players_needed: vec![3, 7],
                cost: Resources { wood: 1, papyrus: 1, ..Default::default() },
                chains_to: vec![Card::Academy, Card::Study],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Tablet]),
            },

            Card::Walls => CardInfo {
                name: "Walls",
                players_needed: vec![3, 7],
                cost: Resources::stone(3),
                chains_to: vec![Card::Fortifications],
                colour: Colour::Red,
                power: Power::Shields(2),
            },

            Card::TrainingGround => CardInfo {
                name: "Training Ground",
                players_needed: vec![4, 6, 7],
                cost: Resources { wood: 1, ore: 2, ..Default::default() },
                chains_to: vec![Card::Circus],
                colour: Colour::Red,
                power: Power::Shields(2),
            },

            Card::Stables => CardInfo {
                name: "Stables",
                players_needed: vec![3, 5],
                cost: Resources { ore: 1, clay: 1, wood: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(2),
            },

            Card::ArcheryRange => CardInfo {
                name: "Archery Range",
                players_needed: vec![3, 6],
                cost: Resources { wood: 2, ore: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(2),
            },

            Card::Pantheon => CardInfo {
                name: "Pantheon",
                players_needed: vec![3, 6],
                cost: Resources { clay: 2, ore: 1, papyrus: 1, loom: 1, glass: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(7),
            },

            Card::Gardens => CardInfo {
                name: "Gardens",
                players_needed: vec![3, 4],
                cost: Resources { wood: 2, clay: 2, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(5),
            },

            Card::TownHall => CardInfo {
                name: "Town Hall",
                players_needed: vec![3, 5, 6],
                cost: Resources { glass: 1, ore: 1, stone: 2, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(6),
            },

            Card::Palace => CardInfo {
                name: "Palace",
                players_needed: vec![3, 7],
                cost: Resources { glass: 1, papyrus: 1, loom: 1, clay: 1, wood: 1, ore: 1, stone: 1, coins: 0 },
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(8),
            },

            Card::Senate => CardInfo {
                name: "Senate",
                players_needed: vec![3, 5],
                cost: Resources { ore: 1, stone: 1, wood: 2, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Blue,
                power: Power::VictoryPoints(6),
            },

            Card::Haven => CardInfo {
                name: "Haven",
                players_needed: vec![3, 4],
                cost: Resources { loom: 1, ore: 1, wood: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Yellow,
                power: Power::per_card_reward(Colour::Brown, true, false, 1, 1)
            },

            Card::Lighthouse => CardInfo {
                name: "Lighthouse",
                players_needed: vec![3, 6],
                cost: Resources { glass: 1, stone: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Yellow,
                power: Power::per_card_reward(Colour::Yellow, true, false, 1, 1)
            },

            Card::ChamberOfCommerce => CardInfo {
                name: "Chamber Of Commerce",
                players_needed: vec![4, 6],
                cost: Resources { clay: 2, papyrus: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Yellow,
                power: Power::per_card_reward(Colour::Grey, true, false, 2, 2),
            },

            Card::Arena => CardInfo {
                name: "Arena",
                players_needed: vec![4, 5, 7],
                cost: Resources { ore: 1, stone: 2, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Yellow,
                power: Power::PerGameItemRewards(vec![PerGameItemReward {
                    game_item: Box::new(|game_item| match game_item {
                        CountableGameItem::CompletedWonderStage => true,
                        _ => false
                    }),
                    me: true,
                    neighbours: false,
                    coins_per_thing: 3,
                    points_per_thing: 1
                }])
            },

            Card::Lodge => CardInfo {
                name: "Lodge",
                players_needed: vec![3, 6],
                cost: Resources { clay: 2, loom: 1, papyrus: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Compass]),
            },

            Card::Observatory => CardInfo {
                name: "Observatory",
                players_needed: vec![3, 7],
                cost: Resources { ore: 2, glass: 1, loom: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Cog]),
            },

            Card::University => CardInfo {
                name: "University",
                players_needed: vec![3, 4],
                cost: Resources { wood: 2, papyrus: 1, glass: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Tablet]),
            },

            Card::Academy => CardInfo {
                name: "Academy",
                players_needed: vec![3, 7],
                cost: Resources { stone: 3, glass: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Compass]),
            },

            Card::Study => CardInfo {
                name: "Study",
                players_needed: vec![3, 5],
                cost: Resources { wood: 1, papyrus: 1, loom: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Green,
                power: Power::Science(vec![ScienceItem::Cog]),
            },

            Card::Fortifications => CardInfo {
                name: "Fortifications",
                players_needed: vec![3, 7],
                cost: Resources { stone: 1, ore: 3, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(3),
            },

            Card::Circus => CardInfo {
                name: "Circus",
                players_needed: vec![4, 5, 6],
                cost: Resources { stone: 3, ore: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(3),
            },

            Card::Arsenal => CardInfo {
                name: "Arsenal",
                players_needed: vec![3, 4, 7],
                cost: Resources { ore: 1, wood: 2, loom: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(3),
            },

            Card::SiegeWorkshop => CardInfo {
                name: "Siege Workshop",
                players_needed: vec![3, 5],
                cost: Resources { wood: 1, clay: 3, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Red,
                power: Power::Shields(3),
            },

            Card::WorkersGuild => CardInfo {
                name: "Workers Guild",
                players_needed: vec![3],
                cost: Resources { ore: 2, clay: 1, stone: 1, wood: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::per_card_reward(Colour::Brown, false, true, 0, 1)
            },

            Card::CraftsmensGuild => CardInfo {
                name: "Craftsmens Guild",
                players_needed: vec![3],
                cost: Resources { ore: 2, stone: 2, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::per_card_reward(Colour::Grey, false, true, 0, 2)
            },

            Card::TradersGuild => CardInfo {
                name: "Traders Guild",
                players_needed: vec![3],
                cost: Resources { loom: 1, papyrus: 1, glass: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::per_card_reward(Colour::Yellow, false, true, 0, 1)
            },

            Card::PhilosophersGuild => CardInfo {
                name: "Philosophers Guild",
                players_needed: vec![3],
                cost: Resources { clay: 3, loom: 1, papyrus: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::per_card_reward(Colour::Green, false, true, 0, 1)
            },

            Card::SpiesGuild => CardInfo {
                name: "Spies Guild",
                players_needed: vec![3],
                cost: Resources { clay: 3, glass: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::per_card_reward(Colour::Red, false, true, 0, 2)
            },

            Card::StrategistsGuild => CardInfo {
                name: "Strategists Guild",
                players_needed: vec![3],
                cost: Resources { ore: 2, stone: 1, loom: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::PerGameItemRewards(vec![PerGameItemReward {
                    game_item: Box::new(|game_item| match game_item {
                        CountableGameItem::DefeatToken => true,
                        _ => false
                    }),
                    me: false,
                    neighbours: true,
                    coins_per_thing: 0,
                    points_per_thing: 1
                }])
            },

            Card::ShipownersGuild => CardInfo {
                name: "Shipowners Guild",
                players_needed: vec![3],
                cost: Resources { wood: 3, papyrus: 1, glass: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::PerGameItemRewards(vec![PerGameItemReward {
                    game_item: Box::new(|game_item| match game_item {
                        CountableGameItem::CountableCard(card) if
                                card.info().colour == Colour::Brown ||
                                card.info().colour == Colour::Grey ||
                                card.info().colour == Colour::Purple => true,
                        _ => false
                    }),
                    me: true,
                    neighbours: false,
                    coins_per_thing: 0,
                    points_per_thing: 1
                }])
            },

            Card::ScientistsGuild => CardInfo {
                name: "Scientists Guild",
                players_needed: vec![3],
                cost: Resources { wood: 2, ore: 2, papyrus: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::Science(vec![ScienceItem::Compass, ScienceItem::Cog, ScienceItem::Tablet]),
            },

            Card::MagistratesGuild => CardInfo {
                name: "Migistrates Guild",
                players_needed: vec![3],
                cost: Resources { wood: 3, stone: 1, loom: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::per_card_reward(Colour::Blue, false, true, 0, 1)
            },

            Card::BuildersGuild => CardInfo {
                name: "Builders Guild",
                players_needed: vec![3],
                cost: Resources { stone: 2, clay: 2, glass: 1, ..Default::default() },
                chains_to: vec![],
                colour: Colour::Purple,
                power: Power::PerGameItemRewards(vec![PerGameItemReward {
                    game_item: Box::new(|game_item| match game_item {
                        CountableGameItem::CompletedWonderStage => true,
                        _ => false
                    }),
                    me: true,
                    neighbours: true,
                    coins_per_thing: 0,
                    points_per_thing: 1
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
