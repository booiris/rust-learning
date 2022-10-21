use serde::{Deserialize, Serialize};

use crate::role::Role;
// use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreepLongMemory {
    pub role: Option<Role>,
    pub need_move: Option<bool>,
}

impl CreepLongMemory {
    pub fn new(role: Role) -> CreepLongMemory {
        CreepLongMemory {
            role: Some(role),
            need_move: Some(true),
        }
    }
}
