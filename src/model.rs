use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreepState {
    pub gcl: Gcl,
    pub cpu: Cpu,
    pub time: i64,
    pub rooms: HashMap<String, RoomType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gcl {
    pub progress: f64,
    #[serde(rename = "progressTotal")]
    pub progress_total: f64,
    pub level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cpu {
    pub bucket: i64,
    pub limit: i64,
    pub used: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomType {
    #[serde(rename = "storageEnergy")]
    pub storage_energy: i64,
    #[serde(rename = "terminalEnergy")]
    pub terminal_energy: i64,
    #[serde(rename = "energyAvailable")]
    pub energy_available: i64,
    #[serde(rename = "energyCapacityAvailable")]
    pub energy_capacity_available: i64,
    #[serde(rename = "controllerProgress")]
    pub controller_progress: i64,
    #[serde(rename = "controllerProgressTotal")]
    pub controller_progress_total: i64,
    #[serde(rename = "controllerLevel")]
    pub controller_level: i64,
    #[serde(rename = "creepRoleAmount")]
    pub creep_role_amount: CreepRoleAmount,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreepRoleAmount {}
