use super::role_action::*;
use screeps::*;

use crate::{
    model::{long_memory::CreepLongMemory, memory::CreepMemory},
    utils::reload::Action,
};
pub struct Upgrader<'a> {
    creep: &'a Creep,
    mem: &'a mut CreepMemory,
    long_mem: &'a mut CreepLongMemory,
}

impl Upgrader<'_> {
    pub fn new<'a>(
        creep: &'a Creep,
        mem: &'a mut CreepMemory,
        long_mem: &'a mut CreepLongMemory,
    ) -> Upgrader<'a> {
        Upgrader {
            creep,
            mem,
            long_mem,
        }
    }
}

impl RoleAction<StructureController> for Upgrader<'_> {
    fn find_target(&mut self) -> StructureController {
        todo!()
    }
    fn work(&self, target: StructureController) -> ReturnCode {
        self.creep.upgrade_controller(&target)
    }
    fn memory(&mut self) -> &mut CreepMemory {
        self.mem
    }
    fn long_memory(&mut self) -> &mut CreepLongMemory {
        self.long_mem
    }
    fn creep(&self) -> &Creep {
        self.creep
    }
    fn action_move(&mut self, target: &StructureController, id: RawObjectId) {
        let creep = self.creep;
        creep.custom_move(target, id, self.mem)
    }
}
