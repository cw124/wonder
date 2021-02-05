use std::cmp::max;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, SubAssign};

use crate::utils::plural;

#[derive(Default, Debug, Clone)]
pub struct Resources {
    pub coins: i32,

    pub wood: i32,
    pub stone: i32,
    pub ore: i32,
    pub clay: i32,

    pub glass: i32,
    pub loom: i32,
    pub papyrus: i32,
}

impl Resources {
    pub fn free() -> Resources {
        Resources { ..Default::default() }
    }

    pub fn coins(num: i32) -> Resources {
        Resources {
            coins: num,
            ..Default::default()
        }
    }

    pub fn wood(num: i32) -> Resources {
        Resources {
            wood: num,
            ..Default::default()
        }
    }

    pub fn stone(num: i32) -> Resources {
        Resources {
            stone: num,
            ..Default::default()
        }
    }

    pub fn ore(num: i32) -> Resources {
        Resources {
            ore: num,
            ..Default::default()
        }
    }

    pub fn clay(num: i32) -> Resources {
        Resources {
            clay: num,
            ..Default::default()
        }
    }

    pub fn glass(num: i32) -> Resources {
        Resources {
            glass: num,
            ..Default::default()
        }
    }

    pub fn loom(num: i32) -> Resources {
        Resources {
            loom: num,
            ..Default::default()
        }
    }

    pub fn papyrus(num: i32) -> Resources {
        Resources {
            papyrus: num,
            ..Default::default()
        }
    }

    // TODO: the below methods (satisfied, has, not_needed, max, split) are pretty weird and hacky. I think maybe we
    //  need to separate the concept of an overall Resources object as we have today, representing something like the
    //  cost of building a card (eg. the Town Hall: 1 glass, 1 ore, 2 stone); and the concept of a resource as provided
    //  by a built brown, grey, or yellow card, which is always (possibly a choice of) a single resource type. They work
    //  slightly differently, are needed for different things, but occasionally interact when we need to add up
    //  instances of the latter to see if we can get to the former.

    /// Returns true if and only if all individual resource counts are at zero or below. If a cost is initialised as a
    /// Resources object and then available resources are subtracted from it, then this returns true if there were
    /// enough resources to afford the cost.
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

    /// Returns true if and only if this resource has at least one of each individual resource the given Resources
    /// object has.
    pub fn has(&self, required: &Resources) -> bool {
        required.coins > 0 && self.coins > 0
            || required.wood > 0 && self.wood > 0
            || required.stone > 0 && self.stone > 0
            || required.ore > 0 && self.ore > 0
            || required.clay > 0 && self.clay > 0
            || required.glass > 0 && self.glass > 0
            || required.loom > 0 && self.loom > 0
            || required.papyrus > 0 && self.papyrus > 0
    }

    /// Returns true if and only if, for some individual resource, the given Resources object has that resource and this
    /// one does not.
    pub fn not_needed(&self, r: &Resources) -> bool {
        (r.wood > 0 && self.wood <= 0)
            || (r.stone > 0 && self.stone <= 0)
            || (r.ore > 0 && self.ore <= 0)
            || (r.clay > 0 && self.clay <= 0)
            || (r.glass > 0 && self.glass <= 0)
            || (r.loom > 0 && self.loom <= 0)
            || (r.papyrus > 0 && self.papyrus <= 0)
    }

    /// Returns the maximum number of items of any given single resource.
    pub fn max(&self) -> i32 {
        max(
            self.wood,
            max(
                self.stone,
                max(self.ore, max(self.clay, max(self.glass, max(self.loom, self.papyrus)))),
            ),
        )
    }

    /// Returns a new Resources object with each individual resource quantity halved.
    pub fn split(&self) -> Resources {
        Resources {
            coins: self.coins,
            wood: self.wood / 2,
            stone: self.stone / 2,
            ore: self.ore / 2,
            clay: self.clay / 2,
            glass: self.glass / 2,
            loom: self.loom / 2,
            papyrus: self.papyrus / 2,
        }
    }
}

impl AddAssign<&Resources> for Resources {
    fn add_assign(&mut self, other: &Self) {
        *self = Self {
            coins: self.coins + other.coins,
            wood: self.wood + other.wood,
            stone: self.stone + other.stone,
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            glass: self.glass + other.glass,
            loom: self.loom + other.loom,
            papyrus: self.papyrus + other.papyrus,
        }
    }
}

impl SubAssign<&Resources> for Resources {
    fn sub_assign(&mut self, other: &Self) {
        *self = Self {
            coins: self.coins - other.coins,
            wood: self.wood - other.wood,
            stone: self.stone - other.stone,
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            glass: self.glass - other.glass,
            loom: self.loom - other.loom,
            papyrus: self.papyrus - other.papyrus,
        }
    }
}

/// Example formatting: `2 wood, 1 glass, 1 papyrus`
impl Display for Resources {
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
