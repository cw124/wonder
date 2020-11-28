use crate::power::Power;
use crate::resources::Resources;

#[derive(Debug)]
pub enum Wonder {
    ColossusOfRhodesA
}

#[allow(dead_code)]
impl Wonder {
    fn name(&self) -> &str {
        match self {
            Wonder::ColossusOfRhodesA => "Colossus of Rhodes",
        }
    }

    pub fn starting_resource(&self) -> Resources {
        match self {
            Wonder::ColossusOfRhodesA => Resources::ore(1),
        }
    }
}

#[allow(dead_code)]
pub enum WonderSlot {
    ColossusOfRhodesASlot1
}

#[allow(dead_code)]
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