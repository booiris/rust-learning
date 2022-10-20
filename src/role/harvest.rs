use screeps::*;

use crate::{model::memory::CreepMemory, utils::reload::Action};
pub trait Harvest {
    fn custom_harvest<T: Harvestable + HasId>(&self, target: &T, mem: &mut CreepMemory);
}

impl Harvest for Creep {
    fn custom_harvest<T>(&self, target: &T, mem: &mut CreepMemory)
    where
        T: Harvestable + HasId,
    {
        self.custom_move(target, target.raw_id(), mem);
        self.harvest(target);
    }
}
