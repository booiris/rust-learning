pub mod builder;
pub mod harvester;
pub mod role_action;
pub mod upgrader;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    Harvester,
    Builder,
    Upgrader,
}
