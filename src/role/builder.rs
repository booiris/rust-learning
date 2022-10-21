use super::role_action::*;
use screeps::*;

use crate::{
    model::{long_memory::CreepLongMemory, memory::CreepMemory},
    utils::reload::Action,
};
pub struct Builder<'a> {
    creep: &'a Creep,
    mem: &'a mut CreepMemory,
    long_mem: &'a mut CreepLongMemory,
}

impl Builder<'_> {
    pub fn new<'a>(
        creep: &'a Creep,
        mem: &'a mut CreepMemory,
        long_mem: &'a mut CreepLongMemory,
    ) -> Builder<'a> {
        Builder {
            creep,
            mem,
            long_mem,
        }
    }
}

impl RoleAction<ConstructionSite> for Builder<'_> {
    fn find_target(&mut self) -> ConstructionSite {
        todo!()
    }

    fn work(&self, target: ConstructionSite) -> ReturnCode {
        self.creep.build(&target)
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
    fn action_move(&mut self, target: &ConstructionSite, id: RawObjectId) {
        let creep = self.creep;
        creep.custom_move(target, id, self.mem)
    }
}
