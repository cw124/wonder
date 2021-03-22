use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::SubAssign;

use crate::utils::plural;

#[derive(Debug, Clone)]
pub enum Resource {
    Wood,
    Stone,
    Ore,
    Clay,

    Glass,
    Loom,
    Papyrus,
}

impl Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Resource::Wood => "wood",
                Resource::Stone => "stone",
                Resource::Ore => "ore",
                Resource::Clay => "clay",
                Resource::Glass => "glass",
                Resource::Loom => "loom",
                Resource::Papyrus => "papyrus",
            }
        )
    }
}

/// The cost of a card.
#[derive(Default, Debug, Clone)]
pub struct Cost {
    pub coins: i32,

    pub wood: i32,
    pub stone: i32,
    pub ore: i32,
    pub clay: i32,

    pub glass: i32,
    pub loom: i32,
    pub papyrus: i32,
}

impl Cost {
    pub fn free() -> Cost {
        Cost { ..Default::default() }
    }

    pub fn coins(num: i32) -> Cost {
        Cost {
            coins: num,
            ..Default::default()
        }
    }

    pub fn wood(num: i32) -> Cost {
        Cost {
            wood: num,
            ..Default::default()
        }
    }

    pub fn stone(num: i32) -> Cost {
        Cost {
            stone: num,
            ..Default::default()
        }
    }

    pub fn ore(num: i32) -> Cost {
        Cost {
            ore: num,
            ..Default::default()
        }
    }

    pub fn clay(num: i32) -> Cost {
        Cost {
            clay: num,
            ..Default::default()
        }
    }

    pub fn glass(num: i32) -> Cost {
        Cost {
            glass: num,
            ..Default::default()
        }
    }

    pub fn loom(num: i32) -> Cost {
        Cost {
            loom: num,
            ..Default::default()
        }
    }

    pub fn papyrus(num: i32) -> Cost {
        Cost {
            papyrus: num,
            ..Default::default()
        }
    }

    /// Returns true if and only if all individual resource counts are at zero or below. If a cost is initialised as a
    /// Cost object and then available resources are subtracted from it, then this returns true if there were enough
    /// resources to afford the cost.
    pub fn satisfied(&self) -> bool {
        self.coins <= 0
            && self.wood <= 0
            && self.stone <= 0
            && self.ore <= 0
            && self.clay <= 0
            && self.glass <= 0
            && self.loom <= 0
            && self.papyrus <= 0
    }

    /// Returns true if and only if this cost includes at least one of the given resource.
    pub fn has(&self, resource: &Resource) -> bool {
        match resource {
            Resource::Wood => self.wood > 0,
            Resource::Stone => self.stone > 0,
            Resource::Ore => self.ore > 0,
            Resource::Clay => self.clay > 0,
            Resource::Glass => self.glass > 0,
            Resource::Loom => self.loom > 0,
            Resource::Papyrus => self.papyrus > 0,
        }
    }
}

impl SubAssign<&Resource> for Cost {
    fn sub_assign(&mut self, resource: &Resource) {
        match resource {
            Resource::Wood => self.wood -= 1,
            Resource::Stone => self.stone -= 1,
            Resource::Ore => self.ore -= 1,
            Resource::Clay => self.clay -= 1,
            Resource::Glass => self.glass -= 1,
            Resource::Loom => self.loom -= 1,
            Resource::Papyrus => self.papyrus -= 1,
        }
    }
}

/// Example formatting: `2 wood, 1 glass, 1 papyrus`
impl Display for Cost {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fn add_if_non_zero(count: i32, resource: &str, resources: &mut Vec<String>) {
            if count > 0 {
                resources.push(format!("{} {}", count, resource));
            }
        }

        let mut resources: Vec<String> = Vec::new();
        if self.coins > 0 {
            resources.push(plural(self.coins, "coin"));
        }
        add_if_non_zero(self.wood, "wood", &mut resources);
        add_if_non_zero(self.stone, "stone", &mut resources);
        add_if_non_zero(self.ore, "ore", &mut resources);
        add_if_non_zero(self.clay, "clay", &mut resources);
        add_if_non_zero(self.glass, "glass", &mut resources);
        add_if_non_zero(self.loom, "loom", &mut resources);
        add_if_non_zero(self.papyrus, "papyrus", &mut resources);

        if resources.is_empty() {
            write!(f, "free")
        } else {
            write!(f, "{}", resources.join(", "))
        }
    }
}
