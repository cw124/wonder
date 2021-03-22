use crate::resources::{Cost, Resource};
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub enum WonderType {
    ColossusOfRhodes,
    LighthouseOfAlexandria,
    TempleOfArtemis,
    HangingGardensOfBabylon,
    StatueOfZeus,
    MausoleumOfHalicarnassus,
    PyramidsOfGiza,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub enum WonderSide {
    A,
    B,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct WonderBoard {
    pub wonder_type: WonderType,
    pub wonder_side: WonderSide,
}

#[allow(dead_code)]
impl WonderType {
    pub fn name(&self) -> &str {
        match self {
            WonderType::ColossusOfRhodes => "The Colossus of Rhodes",
            WonderType::LighthouseOfAlexandria => "The Lighthouse of Alexandria",
            WonderType::TempleOfArtemis => "The Temple of Artemis in Aphesus",
            WonderType::HangingGardensOfBabylon => "The Hanging Gardens of Babylon",
            WonderType::StatueOfZeus => "The Statue of Zeus in Olympia",
            WonderType::MausoleumOfHalicarnassus => "The Mausoleum of Halicarnassus",
            WonderType::PyramidsOfGiza => "The Pyramids of Giza",
        }
    }

    fn starting_resource(&self) -> Resource {
        match self {
            WonderType::ColossusOfRhodes => Resource::Ore,
            WonderType::LighthouseOfAlexandria => Resource::Glass,
            WonderType::TempleOfArtemis => Resource::Papyrus,
            WonderType::HangingGardensOfBabylon => Resource::Clay,
            WonderType::StatueOfZeus => Resource::Wood,
            WonderType::MausoleumOfHalicarnassus => Resource::Loom,
            WonderType::PyramidsOfGiza => Resource::Stone,
        }
    }
}

#[allow(dead_code)]
impl WonderBoard {
    pub fn name(&self) -> &str {
        self.wonder_type.name()
    }

    pub fn starting_resource(&self) -> Resource {
        self.wonder_type.starting_resource()
    }

    pub fn cost(&self, position: u32) -> Cost {
        match (&self.wonder_type, &self.wonder_side, position) {
            (WonderType::ColossusOfRhodes, WonderSide::A, 0) => Cost::wood(2),
            (WonderType::ColossusOfRhodes, WonderSide::A, 1) => Cost::clay(3),
            (WonderType::ColossusOfRhodes, WonderSide::A, 2) => Cost::ore(4),
            (WonderType::ColossusOfRhodes, WonderSide::A, _) => panic!(),

            (WonderType::ColossusOfRhodes, WonderSide::B, 0) => Cost::stone(3),
            (WonderType::ColossusOfRhodes, WonderSide::B, 1) => Cost::ore(4),
            (WonderType::ColossusOfRhodes, WonderSide::B, _) => panic!(),

            (WonderType::LighthouseOfAlexandria, WonderSide::A, 0) => Cost::stone(2),
            (WonderType::LighthouseOfAlexandria, WonderSide::A, 1) => Cost::ore(2),
            (WonderType::LighthouseOfAlexandria, WonderSide::A, 2) => Cost::glass(2),
            (WonderType::LighthouseOfAlexandria, WonderSide::A, _) => panic!(),

            (WonderType::LighthouseOfAlexandria, WonderSide::B, 0) => Cost::clay(2),
            (WonderType::LighthouseOfAlexandria, WonderSide::B, 1) => Cost::wood(2),
            (WonderType::LighthouseOfAlexandria, WonderSide::B, 2) => Cost::stone(3),
            (WonderType::LighthouseOfAlexandria, WonderSide::B, _) => panic!(),

            _ => todo!(),
        }
    }

    // TODO: power
}
