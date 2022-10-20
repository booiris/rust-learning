use screeps::{
    find, game, pathfinder, prelude::*, Creep, ObjectId, Part, RawObjectId, ResourceType,
    ReturnCode, Room, RoomObjectProperties, Source, StructureController, StructureObject,
};

use crate::model::memory::CreepMemory;

trait Action {
    fn custom_move<T>(&self, target: &T, mem: &CreepMemory)
    where
        T: HasPosition;
}

// impl Action for Creep {
//     fn custom_move<T>(&self, target: &T, mem: &CreepMemory)
//     where
//         T: HasPosition,
//     {
//         let pos = target.pos();
//         let a = [pathfinder::SearchGoal { pos, range: 1 }].into_iter();
//         // pathfinder::search_many(self.pos(), a, None);
//     }
// }
