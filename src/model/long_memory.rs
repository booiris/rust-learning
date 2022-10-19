use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreepLongMemory {
    pub _move: ffff,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ffff {
    pub time: i64,
    pub room: String,
}
