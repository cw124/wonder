use std::ops::{AddAssign, SubAssign};

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
}

pub enum ProducedResources {
    // TODO: do we need this distinction or can things just use a vector of 1 to represent Single?
    Single(Resources),
    Choice(Vec<Resources>),
}
