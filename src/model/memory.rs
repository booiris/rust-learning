use screeps::{
    find, game, prelude::*, Creep, ObjectId, Part, RawObjectId, ResourceType, ReturnCode,
    RoomObjectProperties, Source, StructureController, StructureObject,
};

use std::collections::HashMap;

// this enum will represent a creep's lock on a specific target object, storing a js reference to the object id so that we can grab a fresh reference to the object each successive tick, since screeps game objects become 'stale' and shouldn't be used beyond the tick they were fetched
#[derive(Clone)]
pub enum CreepTarget {
    Upgrade(ObjectId<StructureController>),
    Harvest(ObjectId<Source>),
}

pub struct CreepMemory {
    pub target: CreepTarget,
    // pub path: i32,
}

pub struct RoomMemory {}

pub struct Memory {
    pub creep_memory: HashMap<RawObjectId, CreepMemory>,
    pub room_memory: HashMap<RawObjectId, RoomMemory>,
}
