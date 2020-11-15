use std::ops::{AddAssign};

#[derive(Default)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Resources {
    coins: u32,

    wood: u32,
    stone: u32,
    ore: u32,
    clay: u32,

    glass: u32,
    loom: u32,
    papyrus: u32,
}

impl Resources {
    pub fn free() -> Resources {
        return Resources { ..Default::default() }
    }

    pub fn coins(num: u32) -> Resources {
        return Resources { coins: num, ..Default::default() }
    }

    pub fn wood(num: u32) -> Resources {
        return Resources { wood: num, ..Default::default() }
    }

    pub fn stone(num: u32) -> Resources {
        return Resources { stone: num, ..Default::default() }
    }

    pub fn ore(num: u32) -> Resources {
        return Resources { ore: num, ..Default::default() }
    }

    pub fn clay(num: u32) -> Resources {
        return Resources { clay: num, ..Default::default() }
    }

    pub fn glass(num: u32) -> Resources {
        return Resources { glass: num, ..Default::default() }
    }

    pub fn loom(num: u32) -> Resources {
        return Resources { loom: num, ..Default::default() }
    }

    pub fn papyrus(num: u32) -> Resources {
        return Resources { papyrus: num, ..Default::default() }
    }

    /// If this Resources object represents the resources at a user's disposal, then returns `true`
    /// if the user can afford a card whose cost is represented by the given Resources object.
    ///
    /// More formally, returns `true` if and only if, for each resource type, this Resources object
    /// has at least as much of the resource type as the given Resources object.
    pub fn can_afford(&self, other: &Self) -> bool {
        self.coins >= other.coins &&
            self.wood >= other.wood &&
            self.stone >= other.stone &&
            self.ore >= other.ore &&
            self.clay >= other.clay &&
            self.glass >= other.glass &&
            self.loom >= other.loom &&
            self.papyrus >= other.papyrus
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
            papyrus: self.papyrus + other.papyrus
        }
    }
}

pub enum ProducedResources {
    // TODO: do we need this distinction or can things just use a vector of 1 to represent Single?
    Single(Resources),
    Choice(Vec<Resources>),
}
