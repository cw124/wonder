use std::ops::{AddAssign};

#[derive(Default)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Resources {
    pub coins: u32,

    pub wood: u32,
    pub stone: u32,
    pub ore: u32,
    pub clay: u32,

    pub glass: u32,
    pub loom: u32,
    pub papyrus: u32,
}

impl Resources {
    pub fn free() -> Resources {
        Resources { ..Default::default() }
    }

    pub fn coins(num: u32) -> Resources {
        Resources { coins: num, ..Default::default() }
    }

    pub fn wood(num: u32) -> Resources {
        Resources { wood: num, ..Default::default() }
    }

    pub fn stone(num: u32) -> Resources {
        Resources { stone: num, ..Default::default() }
    }

    pub fn ore(num: u32) -> Resources {
        Resources { ore: num, ..Default::default() }
    }

    pub fn clay(num: u32) -> Resources {
        Resources { clay: num, ..Default::default() }
    }

    pub fn glass(num: u32) -> Resources {
        Resources { glass: num, ..Default::default() }
    }

    pub fn loom(num: u32) -> Resources {
        Resources { loom: num, ..Default::default() }
    }

    pub fn papyrus(num: u32) -> Resources {
        Resources { papyrus: num, ..Default::default() }
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
