use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreepLongMemory {
    pub name: String,
}
