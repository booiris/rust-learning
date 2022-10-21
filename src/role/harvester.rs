use super::role_action::*;
use screeps::*;

use crate::{
    model::{long_memory::CreepLongMemory, memory::CreepMemory},
    utils::reload::Action,
};
pub struct Harvester<'a> {
    creep: &'a Creep,
    mem: &'a mut CreepMemory,
    long_mem: &'a mut CreepLongMemory,
}

impl Harvester<'_> {
    pub fn new<'a>(
        creep: &'a Creep,
        mem: &'a mut CreepMemory,
        long_mem: &'a mut CreepLongMemory,
    ) -> Harvester<'a> {
        Harvester {
            creep,
            mem,
            long_mem,
        }
    }
}

impl RoleAction<Source> for Harvester<'_> {
    fn find_target(&mut self) -> Source {
        todo!()
    }
    fn work(&self, target: Source) -> ReturnCode {
        self.creep.harvest(&target)
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
    fn action_move(&mut self, target: &Source, id: RawObjectId) {
        let creep = self.creep;
        creep.custom_move(target, id, self.mem)
    }
}
