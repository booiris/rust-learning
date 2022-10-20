use screeps::*;

use crate::{model::memory::CreepMemory, utils::reload::Action};
pub trait Upgrade {
    fn custom_upgrade(&self, target: &StructureController, mem: &mut CreepMemory);
}

impl Upgrade for Creep {
    fn custom_upgrade(&self, target: &StructureController, mem: &mut CreepMemory) {
        self.custom_move(target, target.raw_id(), mem);
        self.upgrade_controller(target);
    }
}
