use js_sys::{JsString, Math::random};
use screeps::*;

use std::collections::HashMap;

#[derive(Debug)]
pub struct CreepMemory {
    pub path: Option<JsString>,
    pub target: Option<RawObjectId>,
    pub stay_times: u8,
    pub pre_pos: Option<Position>,
    pub max_stay_times: u8,
}

impl CreepMemory {
    pub fn new() -> CreepMemory {
        CreepMemory {
            path: None,
            target: None,
            stay_times: 0,
            pre_pos: None,
            max_stay_times: if random() > 0.6 { 4 } else { 2 },
        }
    }
}

pub struct RoomMemory {}

pub struct Memory {
    pub creep_memory: HashMap<RawObjectId, CreepMemory>,
    pub room_memory: HashMap<RawObjectId, RoomMemory>,
}
