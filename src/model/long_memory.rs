use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    Nobody,
    Harvester,
    Builder,
    Upgrader,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreepLongMemory {
    pub role: Option<Role>,
}
