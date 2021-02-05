use crate::resources::Resources;
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

    fn starting_resource(&self) -> Resources {
        match self {
            WonderType::ColossusOfRhodes => Resources::ore(1),
            WonderType::LighthouseOfAlexandria => Resources::glass(1),
            WonderType::TempleOfArtemis => Resources::papyrus(1),
            WonderType::HangingGardensOfBabylon => Resources::clay(1),
            WonderType::StatueOfZeus => Resources::wood(1),
            WonderType::MausoleumOfHalicarnassus => Resources::loom(1),
            WonderType::PyramidsOfGiza => Resources::stone(1),
        }
    }
}

#[allow(dead_code)]
impl WonderBoard {
    pub fn name(&self) -> &str {
        self.wonder_type.name()
    }

    pub fn starting_resource(&self) -> Resources {
        self.wonder_type.starting_resource()
    }

    pub fn cost(&self, position: u32) -> Resources {
        match (&self.wonder_type, &self.wonder_side, position) {
            (WonderType::ColossusOfRhodes, WonderSide::A, 0) => Resources::wood(2),
            (WonderType::ColossusOfRhodes, WonderSide::A, 1) => Resources::clay(3),
            (WonderType::ColossusOfRhodes, WonderSide::A, 2) => Resources::ore(4),
            (WonderType::ColossusOfRhodes, WonderSide::A, _) => panic!(),

            (WonderType::ColossusOfRhodes, WonderSide::B, 0) => Resources::stone(3),
            (WonderType::ColossusOfRhodes, WonderSide::B, 1) => Resources::ore(4),
            (WonderType::ColossusOfRhodes, WonderSide::B, _) => panic!(),

            (WonderType::LighthouseOfAlexandria, WonderSide::A, 0) => Resources::stone(2),
            (WonderType::LighthouseOfAlexandria, WonderSide::A, 1) => Resources::ore(2),
            (WonderType::LighthouseOfAlexandria, WonderSide::A, 2) => Resources::glass(2),
            (WonderType::LighthouseOfAlexandria, WonderSide::A, _) => panic!(),

            (WonderType::LighthouseOfAlexandria, WonderSide::B, 0) => Resources::clay(2),
            (WonderType::LighthouseOfAlexandria, WonderSide::B, 1) => Resources::wood(2),
            (WonderType::LighthouseOfAlexandria, WonderSide::B, 2) => Resources::stone(3),
            (WonderType::LighthouseOfAlexandria, WonderSide::B, _) => panic!(),

            _ => todo!(),
        }
    }

    // TODO: power
}
