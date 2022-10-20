use screeps::*;

use crate::{model::memory::CreepMemory, utils::reload::Action};
pub trait Build {
    fn custom_build(&self, target: &ConstructionSite, mem: &mut CreepMemory);
}

impl Build for Creep {
    fn custom_build(&self, target: &ConstructionSite, mem: &mut CreepMemory) {
        if let Some(id) = target.try_raw_id() {
            self.custom_move(target, id, mem);
            self.build(target);
        }
    }
}
