use crate::power::Power;
use crate::resources::Resources;

#[derive(Debug)]
pub enum Wonder {
    ColossusOfRhodesA
}

impl Wonder {
    fn name(&self) -> &str {
        match self {
            Wonder::ColossusOfRhodesA => "Colossus of Rhodes",
        }
    }

    fn starting_resource(&self) -> Resources {
        match self {
            Wonder::ColossusOfRhodesA => Resources::ore(1),
        }
    }
}

pub enum WonderSlot {
    ColossusOfRhodesASlot1
}

impl WonderSlot {
    pub fn cost(&self) -> Resources {
        match self {
            WonderSlot::ColossusOfRhodesASlot1 => Resources::wood(2),
        }
    }

    fn power(&self) -> Power {
        match self {
            WonderSlot::ColossusOfRhodesASlot1 => Power::VictoryPoints(3),
        }
    }
}