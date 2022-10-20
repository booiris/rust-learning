use js_sys::JsString;
use screeps::*;

use std::collections::HashMap;

pub struct CreepMemory {
    pub path: Option<JsString>,
    pub target: Option<RawObjectId>,
}

impl CreepMemory {
    pub fn new() -> CreepMemory {
        CreepMemory {
            path: None,
            target: None,
        }
    }
}

pub struct RoomMemory {}

pub struct Memory {
    pub creep_memory: HashMap<RawObjectId, CreepMemory>,
    pub room_memory: HashMap<RawObjectId, RoomMemory>,
}
