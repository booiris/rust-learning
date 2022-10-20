use js_sys::JsString;
use screeps::*;

use std::collections::HashMap;

pub struct CreepMemory {
    pub path: Option<JsString>,
    pub target: Option<RawObjectId>,
    pub stay_times: u8,
    pub pre_pos: Option<Position>,
}

impl CreepMemory {
    pub fn new() -> CreepMemory {
        CreepMemory {
            path: None,
            target: None,
            stay_times: 0,
            pre_pos: None,
        }
    }
}

pub struct RoomMemory {}

pub struct Memory {
    pub creep_memory: HashMap<RawObjectId, CreepMemory>,
    pub room_memory: HashMap<RawObjectId, RoomMemory>,
}
