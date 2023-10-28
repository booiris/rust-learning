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

#[derive(Default, Debug)]
pub struct DBParamsType {
    measurement: String,
    tags: Vec<(String, String)>,
    fields: Vec<(String, String)>,
    time: String,
}

impl DBParamsType {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_measurement(mut self, measurement: &str) -> Self {
        self.measurement = measurement.to_string();
        self
    }

    pub fn set_time(mut self, time: &str) -> Self {
        self.time = time.to_string();
        self
    }

    pub fn add_tag<T: ToString>(mut self, tag: &str, v: T) -> Self {
        self.tags.push((tag.to_string(), v.to_string()));
        self
    }

    pub fn add_field<T: ToString>(mut self, field: &str, v: T) -> Self {
        self.fields.push((field.into(), v.to_string()));
        self
    }

    pub fn build(self) -> String {
        let mut res = String::new();

        res.push_str(&self.measurement);
        if !self.tags.is_empty() {
            res.push(',');
        }
        res.push_str(
            &self
                .tags
                .iter()
                .map(|x| format!("{}={}", x.0, x.1))
                .collect::<Vec<String>>()
                .join(","),
        );
        res.push(' ');
        res.push_str(
            &self
                .fields
                .iter()
                .map(|x| format!("{}={}", x.0, x.1))
                .collect::<Vec<String>>()
                .join(","),
        );
        res.push(' ');
        res.push_str(&self.time);

        res
    }
}
